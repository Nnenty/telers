use crate::{
    client::Bot,
    context::Context,
    dispatcher::{
        event::{
            bases::{EventReturn, PropagateEventResult},
            service::{BoxFuture, Service, ServiceFactory},
            telegram::handler::{
                Handler, HandlerObject, HandlerObjectService, Request as HandlerRequest,
            },
        },
        middlewares::{
            inner::{
                base::Middlewares as InnerMiddlewares, manager::Manager as InnerMiddlewareManager,
            },
            outer::{
                base::Middlewares as OuterMiddlewares, manager::Manager as OuterMiddlewareManager,
            },
        },
    },
    error::app,
    extract::FromEventAndContext,
    filters::base::Filter,
    types::Update,
};

use std::{
    fmt::{self, Debug, Formatter},
    sync::Arc,
};

#[derive(Clone)]
pub struct Request {
    bot: Arc<Bot>,
    update: Arc<Update>,
    context: Arc<Context>,
}

impl PartialEq for Request {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.bot, &other.bot)
            && Arc::ptr_eq(&self.update, &other.update)
            && Arc::ptr_eq(&self.context, &other.context)
    }
}

impl Request {
    #[must_use]
    pub fn new<B: Into<Arc<Bot>>, U: Into<Arc<Update>>, C: Into<Arc<Context>>>(
        bot: B,
        update: U,
        context: C,
    ) -> Self {
        Self {
            bot: bot.into(),
            update: update.into(),
            context: context.into(),
        }
    }

    #[must_use]
    pub fn bot(&self) -> Arc<Bot> {
        Arc::clone(&self.bot)
    }

    #[must_use]
    pub fn update(&self) -> Arc<Update> {
        Arc::clone(&self.update)
    }

    #[must_use]
    pub fn context(&self) -> Arc<Context> {
        Arc::clone(&self.context)
    }
}

impl From<Request> for HandlerRequest {
    fn from(req: Request) -> Self {
        Self::new(req.bot, req.update, req.context)
    }
}

pub struct Response {
    request: Request,
    response: PropagateEventResult,
}

impl Response {
    #[must_use]
    pub fn request(&self) -> &Request {
        &self.request
    }

    #[must_use]
    pub fn response(&self) -> &PropagateEventResult {
        &self.response
    }
}

/// Event observer for Telegram events.
/// Here you can register handler with filter.
/// This observer will stop event propagation when first handler is pass.
pub struct Observer {
    event_name: &'static str,
    handlers: Vec<HandlerObject>,
    /// Common handler of the observer with dummy callback which never will be used
    common_handler: HandlerObject,
    pub middlewares: InnerMiddlewareManager,
    pub outer_middlewares: OuterMiddlewareManager,
}

impl Observer {
    /// Create a new event observer
    /// # Arguments
    /// * `event_name` - Event observer name, can be used for logging
    #[must_use]
    pub fn new(event_name: &'static str) -> Self {
        Self {
            event_name,
            handlers: vec![],
            common_handler: HandlerObject::new(
                || async move { unreachable!("This is only for observer filters and without logic") },
                vec![],
            ),
            middlewares: InnerMiddlewareManager::default(),
            outer_middlewares: OuterMiddlewareManager::default(),
        }
    }

    #[must_use]
    pub fn event_name(&self) -> &str {
        self.event_name
    }

    #[must_use]
    pub fn handlers(&self) -> &[HandlerObject] {
        &self.handlers
    }

    /// Get filters for all handlers of this event observer
    #[must_use]
    pub fn filters(&self) -> &[Box<dyn Filter>] {
        self.common_handler.filters()
    }

    /// Register filter for all handlers of this event observer
    /// # Arguments
    /// * `filter` - Filter for the observer
    pub fn filter(&mut self, filter: Box<dyn Filter>) {
        self.common_handler.filter(filter);
    }

    /// Register event handler
    /// # Arguments
    /// * `handler` - Handler for the observer
    /// * `filters` - Filters for the handler
    pub fn register<H, Args>(&mut self, handler: H, filters: Vec<Box<dyn Filter>>)
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send + Sync + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.handlers.push(HandlerObject::new(handler, filters));
    }

    /// Alias to [`Observer::register`] method
    pub fn on<H, Args>(&mut self, handler: H, filters: Vec<Box<dyn Filter>>)
    where
        H: Handler<Args> + Clone + Send + Sync + 'static,
        H::Future: Send + Sync + 'static,
        H::Output: Into<EventReturn>,
        Args: FromEventAndContext + 'static,
    {
        self.register(handler, filters);
    }
}

impl Debug for Observer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Observer")
            .field("event_name", &self.event_name)
            .finish()
    }
}

impl Default for Observer {
    fn default() -> Self {
        Self::new("default")
    }
}

impl AsRef<Observer> for Observer {
    #[must_use]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ServiceFactory<Request> for Observer {
    type Response = Response;
    type Error = app::ErrorKind;
    type Config = ();
    type Service = ObserverService;
    type InitError = ();
    type Future = BoxFuture<Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let event_name = self.event_name;
        let futs = self
            .handlers
            .iter()
            .map(|handler| handler.new_service(()))
            .collect::<Vec<_>>();
        let fut = self.common_handler.new_service(());
        let middlewares = self.middlewares.middlewares().clone();
        let outer_middlewares = self.outer_middlewares.middlewares().clone();

        Box::pin(async move {
            let mut handlers = vec![];
            for fut in futs {
                handlers.push(fut.await?);
            }

            let common_handler = fut.await?;

            Ok(ObserverService {
                event_name,
                handlers: Arc::new(handlers),
                common_handler: Arc::new(common_handler),
                middlewares: middlewares.clone(),
                outer_middlewares: outer_middlewares.clone(),
            })
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct ObserverService {
    event_name: &'static str,
    handlers: Arc<Vec<HandlerObjectService>>,
    /// Common handler service of the observer with dummy callback which never will be used
    common_handler: Arc<HandlerObjectService>,
    middlewares: InnerMiddlewares,
    outer_middlewares: OuterMiddlewares,
}

impl ObserverService {
    #[must_use]
    pub fn event_name(&self) -> &str {
        self.event_name
    }

    #[must_use]
    pub fn middlewares(&self) -> &InnerMiddlewares {
        &self.middlewares
    }

    #[must_use]
    pub fn outer_middlewares(&self) -> &OuterMiddlewares {
        &self.outer_middlewares
    }

    /// Propagate event to handlers and stops propagation on first match.
    /// Handler will be called when all its filters is pass.
    /// # Errors
    /// - If any handler returns error. Probably it's error to extract args to the handler.
    #[allow(clippy::similar_names)]
    pub async fn trigger(&self, req: Request) -> Result<Response, app::ErrorKind> {
        let handler_req = req.clone().into();

        // Check observer filters
        if !self.common_handler.check(&handler_req) {
            return Ok(Response {
                request: req,
                response: PropagateEventResult::Rejected,
            });
        }

        for handler in self.handlers.iter() {
            if !handler.check(&handler_req) {
                continue;
            }

            let res = if self.middlewares.is_empty() {
                handler.call(handler_req.clone()).await?
            } else {
                let middleware = Arc::clone(&self.middlewares[0]);
                let next_middlewares = Box::new(self.middlewares[1..].to_vec().clone().into_iter());

                // Call first middleware (it will call next middlewares or handler)
                middleware
                    .call(handler.service(), handler_req.clone(), next_middlewares)
                    .await?
            };

            let handler_response = res.response();

            return if handler_response.is_skip() {
                continue;
            } else if handler_response.is_cancel() {
                Ok(Response {
                    request: req,
                    response: PropagateEventResult::Rejected,
                })
            } else {
                Ok(Response {
                    request: req,
                    response: PropagateEventResult::Handled(res),
                })
            };
        }

        // Return a response if the event unhandled by observer
        Ok(Response {
            request: req,
            response: PropagateEventResult::Unhandled,
        })
    }
}

impl Debug for ObserverService {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ObserverService")
            .field("event_name", &self.event_name)
            .finish()
    }
}

impl Service<Request> for ObserverService {
    type Response = Response;
    type Error = app::ErrorKind;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn call(&self, _: Request) -> Self::Future {
        log::error!("{:?}: Should not be called", self);

        unimplemented!(
            "ObserverService is not intended to be called directly. \
            Use ObserverService::trigger instead"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dispatcher::event::bases::Action, filters::command, types::Message};

    use tokio;

    #[tokio::test]
    async fn test_observer_trigger() {
        let bot = Bot::default();
        let context = Context::default();

        let mut observer = Observer::new("test");
        // Register common filter, which handlers can't pass
        observer.filter(Box::new(command::Command {
            commands: vec![command::PatternType::Text("start")],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        }));
        observer.register(|| async {}, vec![]);
        observer.register(
            || async {
                unreachable!("It's shouldn't trigger because the first handler handles the event")
            },
            vec![],
        );

        let observer_service = observer.new_service(()).await.unwrap();
        let req = Request::new(bot, Update::default(), context);
        let res = observer_service.trigger(req.clone()).await.unwrap();

        // Filter not pass, so handler should be rejected
        match res.response() {
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }

        let req = Request::new(
            req.bot(),
            Update {
                message: Some(Message {
                    text: Some("/start".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            req.context(),
        );
        let res = observer_service.trigger(req).await.unwrap();

        // Filter pass, so handler should be handled
        match res.response() {
            PropagateEventResult::Handled(_) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[tokio::test]
    async fn test_observer_event_return() {
        let bot = Bot::default();
        let context = Context::default();
        let update = Update::default();

        let mut observer = Observer::new("test");
        observer.register(|| async { Action::Skip }, vec![]);
        observer.register(|| async {}, vec![]);

        let observer_service = observer.new_service(()).await.unwrap();

        let req = Request::new(bot, update, context);
        let res = observer_service.trigger(req.clone()).await.unwrap();

        // First handler returns `Action::Skip`, so second handler should be called
        match res.response() {
            PropagateEventResult::Handled(handler_res) => {
                assert_eq!(*handler_res.response(), EventReturn::default());
            }
            _ => panic!("Unexpected result"),
        }

        let mut observer = Observer::new("test2");
        observer.register(|| async { Action::Skip }, vec![]);
        observer.register(|| async { Action::Cancel }, vec![]);

        let observer_service = observer.new_service(()).await.unwrap();

        let res = observer_service.trigger(req).await.unwrap();

        // First handler returns `Action::Skip`, so second handler should be called and it returns `Action::Cancel`,
        // so response should be `PropagateEventResult::Rejected`
        match res.response() {
            PropagateEventResult::Rejected => {}
            _ => panic!("Unexpected result"),
        }
    }
}

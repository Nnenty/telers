#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

mod dispatcher;

pub mod event;
pub mod middlewares;
pub mod router;

pub use dispatcher::{Dispatcher, DispatcherService};
pub use router::Router;

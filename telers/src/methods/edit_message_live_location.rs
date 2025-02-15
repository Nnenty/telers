use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InlineKeyboardMarkup, MessageOrTrue},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to edit live location messages. A location can be edited until its `live_period` expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](crate::methods::StopMessageLiveLocation).
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagelivelocation>
/// # Returns
/// On success, if the edited message is not an inline message, the edited [`MessageOrTrue`] is returned,
/// otherwise `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EditMessageLiveLocation {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    pub business_connection_id: Option<String>,
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: Option<ChatIdKind>,
    /// Required if `inline_message_id` is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// Longitude of new location
    pub longitude: f64,
    /// Latitude of new location
    pub latitude: f64,
    /// New period in seconds during which the location can be updated, starting from the message send date. If `0x7FFFFFFF` is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current `live_period` by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then `live_period` remains unchanged
    pub live_period: Option<i64>,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageLiveLocation {
    #[must_use]
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            business_connection_id: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            longitude,
            latitude,
            live_period: None,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn business_connection_id(self, val: impl Into<String>) -> Self {
        Self {
            business_connection_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn inline_message_id(self, val: impl Into<String>) -> Self {
        Self {
            inline_message_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn longitude(self, val: f64) -> Self {
        Self {
            longitude: val,
            ..self
        }
    }

    #[must_use]
    pub fn latitude(self, val: f64) -> Self {
        Self {
            latitude: val,
            ..self
        }
    }

    #[must_use]
    pub fn live_period(self, val: i64) -> Self {
        Self {
            live_period: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn horizontal_accuracy(self, val: f64) -> Self {
        Self {
            horizontal_accuracy: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn heading(self, val: i64) -> Self {
        Self {
            heading: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn proximity_alert_radius(self, val: i64) -> Self {
        Self {
            proximity_alert_radius: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }
}

impl EditMessageLiveLocation {
    #[must_use]
    pub fn business_connection_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            business_connection_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn chat_id_option(self, val: Option<impl Into<ChatIdKind>>) -> Self {
        Self {
            chat_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn message_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn inline_message_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            inline_message_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn live_period_option(self, val: Option<i64>) -> Self {
        Self {
            live_period: val,
            ..self
        }
    }

    #[must_use]
    pub fn horizontal_accuracy_option(self, val: Option<f64>) -> Self {
        Self {
            horizontal_accuracy: val,
            ..self
        }
    }

    #[must_use]
    pub fn heading_option(self, val: Option<i64>) -> Self {
        Self {
            heading: val,
            ..self
        }
    }

    #[must_use]
    pub fn proximity_alert_radius_option(self, val: Option<i64>) -> Self {
        Self {
            proximity_alert_radius: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<InlineKeyboardMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for EditMessageLiveLocation {
    type Method = Self;
    type Return = MessageOrTrue;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("editMessageLiveLocation", self, None)
    }
}

impl AsRef<EditMessageLiveLocation> for EditMessageLiveLocation {
    fn as_ref(&self) -> &Self {
        self
    }
}

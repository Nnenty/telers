use crate::types::{Update, UpdateKind};

use std::borrow::Cow;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the update
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum UpdateType {
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "inline_query")]
    InlineQuery,
    #[strum(serialize = "chosen_inline_result")]
    ChosenInlineResult,
    #[strum(serialize = "callback_query")]
    CallbackQuery,
    #[strum(serialize = "channel_post")]
    ChannelPost,
    #[strum(serialize = "edited_message")]
    EditedMessage,
    #[strum(serialize = "edited_channel_post")]
    EditedChannelPost,
    #[strum(serialize = "message_reaction")]
    MessageReaction,
    #[strum(serialize = "shipping_query")]
    ShippingQuery,
    #[strum(serialize = "pre_checkout_query")]
    PreCheckoutQuery,
    #[strum(serialize = "poll")]
    Poll,
    #[strum(serialize = "poll_answer")]
    PollAnswer,
    #[strum(serialize = "my_chat_member")]
    MyChatMember,
    #[strum(serialize = "chat_member")]
    ChatMember,
    #[strum(serialize = "chat_join_request")]
    ChatJoinRequest,
}

impl UpdateType {
    #[must_use]
    pub const fn all() -> &'static [UpdateType; 15] {
        &[
            UpdateType::Message,
            UpdateType::InlineQuery,
            UpdateType::ChosenInlineResult,
            UpdateType::CallbackQuery,
            UpdateType::ChannelPost,
            UpdateType::EditedMessage,
            UpdateType::EditedChannelPost,
            UpdateType::MessageReaction,
            UpdateType::ShippingQuery,
            UpdateType::PreCheckoutQuery,
            UpdateType::Poll,
            UpdateType::PollAnswer,
            UpdateType::MyChatMember,
            UpdateType::ChatMember,
            UpdateType::ChatJoinRequest,
        ]
    }
}

impl From<UpdateType> for Cow<'static, str> {
    fn from(update_type: UpdateType) -> Self {
        Into::<&'static str>::into(update_type).into()
    }
}

impl From<&UpdateType> for Cow<'static, str> {
    fn from(update_type: &UpdateType) -> Self {
        Into::<&'static str>::into(update_type).into()
    }
}

impl<'a> PartialEq<&'a str> for UpdateType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl<'a> From<&'a UpdateKind> for UpdateType {
    fn from(update_kind: &UpdateKind) -> Self {
        match update_kind {
            UpdateKind::Message(_) => UpdateType::Message,
            UpdateKind::EditedMessage(_) => UpdateType::EditedMessage,
            UpdateKind::ChannelPost(_) => UpdateType::ChannelPost,
            UpdateKind::EditedChannelPost(_) => UpdateType::EditedChannelPost,
            UpdateKind::MessageReaction(_) => UpdateType::MessageReaction,
            UpdateKind::InlineQuery(_) => UpdateType::InlineQuery,
            UpdateKind::ChosenInlineResult(_) => UpdateType::ChosenInlineResult,
            UpdateKind::CallbackQuery(_) => UpdateType::CallbackQuery,
            UpdateKind::ShippingQuery(_) => UpdateType::ShippingQuery,
            UpdateKind::PreCheckoutQuery(_) => UpdateType::PreCheckoutQuery,
            UpdateKind::Poll(_) => UpdateType::Poll,
            UpdateKind::PollAnswer(_) => UpdateType::PollAnswer,
            UpdateKind::MyChatMember(_) => UpdateType::MyChatMember,
            UpdateKind::ChatMember(_) => UpdateType::ChatMember,
            UpdateKind::ChatJoinRequest(_) => UpdateType::ChatJoinRequest,
        }
    }
}

impl<'a> From<&'a Update> for UpdateType {
    fn from(update: &'a Update) -> Self {
        UpdateType::from(update.kind())
    }
}

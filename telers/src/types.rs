//! This module contains telegram types from the [Telegram Bot API](https://core.telegram.org/bots/api).
//! Each type has a description and a link to the official documentation,
//! but we recommend to use documentation instead of description (docs copy-paste) because it has actual info
//!
//! Telegram types are represented as Rust structs, if a field:
//! - is optional, it will be wrapped in [`Option`],
//! - is an array, it will be wrapped in [`Vec`] or slice wrapped in [`Box`],
//! - is a recursive type, it will be wrapped in [`Box`].
//! - is a tagged union, it will be wrapped in `enum` with variants named as in the documentation,
//! - is a type with optional fields, it can be represented as an enum with variants for specific cases (check [`Message`] for example).
//! - is a string, it will be represented as [`String`] or [`str`] wrapped in [`Box`],
//! - is a number, it will be represented as [`i64`] or [`u16`] if it's UTF-16 code unit,
//! - is a float, it will be represented as [`f64`],
//! - is a boolean, it will be represented as [`bool`],
//! - is a file, it will be represented as [`InputFile`],
//! - is a chat id with kind (integer or string), it will be represented as [`ChatIdKind`],
//! - is a date, it will be represented as [`i64`] (unix timestamp).
//!
//! Tagged unions are represented as enums with variants named as in the documentation
//! and we implement [`From`] trait for them to make it easier to convert from them to the enum.
//! For example, [`BotCommandScope`] is represented as enum with variants:
//! - [`BotCommandScopeDefault`]
//! - [`BotCommandScopeAllPrivateChats`]
//! - [`BotCommandScopeAllGroupChats`]
//! - [`BotCommandScopeAllChatAdministrators`]
//! - [`BotCommandScopeChat`]
//! - [`BotCommandScopeChatAdministrators`]
//! - [`BotCommandScopeChatMember`]
//! Each variant has an implementation of [`From`] trait to convert from the variant to the [`BotCommandScope`],
//! so you can write `from` and `into` to convert between them instead of boilerplate code.
//! Many methods in the library accept "union" and tagged types as generic parameters with [`Into`] trait bounds,
//! so you can pass any of the variants to them.
//!
//! The telegram type with optional fields can be represented as an enum with variants for specific cases.
//! For example, [`Message`] is represented as enum with variants:
//! - [`MessageText`]
//! - [`MessageAnimation`]
//! and so on... (see [`Message`] for full list of variants).
//! Each variant has an implementation of [`Into`] trait to convert from the variant to the [`Message`].

pub mod animation;
pub mod audio;
pub mod background_fill;
pub mod background_fill_freeform_gradient;
pub mod background_fill_gradient;
pub mod background_fill_solid;
pub mod background_type;
pub mod background_type_chat_theme;
pub mod background_type_fill;
pub mod background_type_pattern;
pub mod background_type_wallpaper;
pub mod birthdate;
pub mod bot_command;
pub mod bot_command_scope;
pub mod bot_command_scope_all_chat_administrators;
pub mod bot_command_scope_all_group_chats;
pub mod bot_command_scope_all_private_chats;
pub mod bot_command_scope_chat;
pub mod bot_command_scope_chat_administrators;
pub mod bot_command_scope_chat_member;
pub mod bot_command_scope_default;
pub mod bot_description;
pub mod bot_name;
pub mod bot_short_description;
pub mod business_connection;
pub mod business_intro;
pub mod business_location;
pub mod business_messages_deleted;
pub mod business_opening_hours;
pub mod business_opening_hours_interval;
pub mod callback_game;
pub mod callback_query;
pub mod chat;
pub mod chat_administrator_rights;
pub mod chat_background;
pub mod chat_boost;
pub mod chat_boost_added;
pub mod chat_boost_removed;
pub mod chat_boost_source;
pub mod chat_boost_source_gift_code;
pub mod chat_boost_source_giveaway;
pub mod chat_boost_source_premium;
pub mod chat_boost_updated;
pub mod chat_full_info;
pub mod chat_id_kind;
pub mod chat_invite_link;
pub mod chat_join_request;
pub mod chat_location;
pub mod chat_member;
pub mod chat_member_administrator;
pub mod chat_member_banned;
pub mod chat_member_left;
pub mod chat_member_member;
pub mod chat_member_owner;
pub mod chat_member_restricted;
pub mod chat_member_updated;
pub mod chat_permissions;
pub mod chat_photo;
pub mod chat_shared;
pub mod chosen_inline_result;
pub mod contact;
pub mod dice;
pub mod document;
pub mod encrypted_credentials;
pub mod encrypted_passport_element;
pub mod external_reply_info;
pub mod file;
pub mod force_reply;
pub mod forum_topic;
pub mod forum_topic_closed;
pub mod forum_topic_created;
pub mod forum_topic_edited;
pub mod forum_topic_reopened;
pub mod game;
pub mod game_high_score;
pub mod general_forum_topic_hidden;
pub mod general_forum_topic_unhidden;
pub mod giveaway;
pub mod giveaway_completed;
pub mod giveaway_created;
pub mod giveaway_winners;
pub mod inaccessible_message;
pub mod inline_keyboard_button;
pub mod inline_keyboard_markup;
pub mod inline_query;
pub mod inline_query_result;
pub mod inline_query_result_article;
pub mod inline_query_result_audio;
pub mod inline_query_result_cached_audio;
pub mod inline_query_result_cached_document;
pub mod inline_query_result_cached_gif;
pub mod inline_query_result_cached_mpeg4_gif;
pub mod inline_query_result_cached_photo;
pub mod inline_query_result_cached_sticker;
pub mod inline_query_result_cached_video;
pub mod inline_query_result_cached_voice;
pub mod inline_query_result_contact;
pub mod inline_query_result_document;
pub mod inline_query_result_game;
pub mod inline_query_result_gif;
pub mod inline_query_result_location;
pub mod inline_query_result_mpeg4_gif;
pub mod inline_query_result_photo;
pub mod inline_query_result_venue;
pub mod inline_query_result_video;
pub mod inline_query_result_voice;
pub mod inline_query_results_button;
pub mod input_contact_message_content;
pub mod input_file;
pub mod input_invoice_message_content;
pub mod input_location_message_content;
pub mod input_media;
pub mod input_media_animation;
pub mod input_media_audio;
pub mod input_media_document;
pub mod input_media_photo;
pub mod input_media_video;
pub mod input_message_content;
pub mod input_paid_media;
pub mod input_paid_media_photo;
pub mod input_paid_media_video;
pub mod input_poll_option;
pub mod input_sticker;
pub mod input_text_message_content;
pub mod input_venue_message_content;
pub mod invoice;
pub mod keyboard_button;
pub mod keyboard_button_poll_type;
pub mod keyboard_button_request_chat;
pub mod keyboard_button_request_users;
pub mod labeled_price;
pub mod link_preview_options;
pub mod location;
pub mod login_url;
pub mod mask_position;
pub mod maybe_inaccessible_message;
pub mod menu_button;
pub mod menu_button_commands;
pub mod menu_button_default;
pub mod menu_button_web_app;
pub mod message;
pub mod message_auto_delete_timer_changed;
pub mod message_entity;
pub mod message_id;
pub mod message_or_true;
pub mod message_origin;
pub mod message_origin_channel;
pub mod message_origin_chat;
pub mod message_origin_hidden_user;
pub mod message_origin_user;
pub mod message_reaction_count_updated;
pub mod message_reaction_updated;
pub mod order_info;
pub mod paid_media;
pub mod paid_media_info;
pub mod paid_media_photo;
pub mod paid_media_preview;
pub mod paid_media_video;
pub mod passport_data;
pub mod passport_element_error;
pub mod passport_element_error_data_field;
pub mod passport_element_error_file;
pub mod passport_element_error_files;
pub mod passport_element_error_front_side;
pub mod passport_element_error_reverse_side;
pub mod passport_element_error_selfie;
pub mod passport_element_error_translation_file;
pub mod passport_element_error_translation_files;
pub mod passport_element_error_unspecified;
pub mod passport_file;
pub mod photo_size;
pub mod poll;
pub mod poll_answer;
pub mod poll_option;
pub mod pre_checkout_query;
pub mod proximity_alert_triggered;
pub mod reaction_count;
pub mod reaction_type;
pub mod reaction_type_custom_emoji;
pub mod reaction_type_emoji;
pub mod reaction_type_paid;
pub mod refunded_payment;
pub mod reply_keyboard_markup;
pub mod reply_keyboard_remove;
pub mod reply_markup;
pub mod reply_parameters;
pub mod response_parameters;
pub mod revenue_withdrawal_state;
pub mod revenue_withdrawal_state_failed;
pub mod revenue_withdrawal_state_pending;
pub mod revenue_withdrawal_state_succeeded;
pub mod sent_web_app_message;
pub mod shared_user;
pub mod shipping_address;
pub mod shipping_option;
pub mod shipping_query;
pub mod star_transaction;
pub mod star_transactions;
pub mod sticker;
pub mod sticker_set;
pub mod story;
pub mod successful_payment;
pub mod switch_inline_query_chosen_chat;
pub mod text_quote;
pub mod transaction_partner;
pub mod transaction_partner_fragment;
pub mod transaction_partner_other;
pub mod transaction_partner_telegram_ads;
pub mod transaction_partner_user;
pub mod update;
pub mod user;
pub mod user_chat_boosts;
pub mod user_profile_photos;
pub mod users_shared;
pub mod venue;
pub mod video;
pub mod video_chat_ended;
pub mod video_chat_participants_invited;
pub mod video_chat_scheduled;
pub mod video_chat_started;
pub mod video_note;
pub mod voice;
pub mod web_app_data;
pub mod web_app_info;
pub mod web_app_user;
pub mod webhook_info;
pub mod write_access_allowed;

pub use animation::Animation;
pub use audio::Audio;
pub use background_fill::BackgroundFill;
pub use background_fill_freeform_gradient::BackgroundFillFreeformGradient;
pub use background_fill_gradient::BackgroundFillGradient;
pub use background_fill_solid::BackgroundFillSolid;
pub use background_type::BackgroundType;
pub use background_type_chat_theme::BackgroundTypeChatTheme;
pub use background_type_fill::BackgroundTypeFill;
pub use background_type_pattern::BackgroundTypePattern;
pub use background_type_wallpaper::BackgroundTypeWallpaper;
pub use birthdate::Birthdate;
pub use bot_command::BotCommand;
pub use bot_command_scope::BotCommandScope;
pub use bot_command_scope_all_chat_administrators::BotCommandScopeAllChatAdministrators;
pub use bot_command_scope_all_group_chats::BotCommandScopeAllGroupChats;
pub use bot_command_scope_all_private_chats::BotCommandScopeAllPrivateChats;
pub use bot_command_scope_chat::BotCommandScopeChat;
pub use bot_command_scope_chat_administrators::BotCommandScopeChatAdministrators;
pub use bot_command_scope_chat_member::BotCommandScopeChatMember;
pub use bot_command_scope_default::BotCommandScopeDefault;
pub use bot_description::BotDescription;
pub use bot_name::BotName;
pub use bot_short_description::BotShortDescription;
pub use business_connection::BusinessConnection;
pub use business_intro::BusinessIntro;
pub use business_location::BusinessLocation;
pub use business_messages_deleted::BusinessMessagesDeleted;
pub use business_opening_hours::BusinessOpeningHours;
pub use business_opening_hours_interval::BusinessOpeningHoursInterval;
pub use callback_game::CallbackGame;
pub use callback_query::CallbackQuery;
pub use chat::{Channel, Chat, Group, Private, Supergroup};
pub use chat_administrator_rights::ChatAdministratorRights;
pub use chat_background::ChatBackground;
pub use chat_boost::ChatBoost;
pub use chat_boost_added::ChatBoostAdded;
pub use chat_boost_removed::ChatBoostRemoved;
pub use chat_boost_source::ChatBoostSource;
pub use chat_boost_source_gift_code::ChatBoostSourceGiftCode;
pub use chat_boost_source_giveaway::ChatBoostSourceGiveaway;
pub use chat_boost_source_premium::ChatBoostSourcePremium;
pub use chat_boost_updated::ChatBoostUpdated;
pub use chat_full_info::{
    ChannelFullInfo, ChatFullInfo, GroupFullInfo, PrivateFullInfo, SupergroupFullInfo,
};
pub use chat_id_kind::ChatIdKind;
pub use chat_invite_link::ChatInviteLink;
pub use chat_join_request::ChatJoinRequest;
pub use chat_location::ChatLocation;
pub use chat_member::ChatMember;
pub use chat_member_administrator::ChatMemberAdministrator;
pub use chat_member_banned::ChatMemberBanned;
pub use chat_member_left::ChatMemberLeft;
pub use chat_member_member::ChatMemberMember;
pub use chat_member_owner::ChatMemberOwner;
pub use chat_member_restricted::ChatMemberRestricted;
pub use chat_member_updated::ChatMemberUpdated;
pub use chat_permissions::ChatPermissions;
pub use chat_photo::ChatPhoto;
pub use chat_shared::ChatShared;
pub use chosen_inline_result::ChosenInlineResult;
pub use contact::Contact;
pub use dice::Dice;
pub use document::Document;
pub use encrypted_credentials::EncryptedCredentials;
pub use encrypted_passport_element::{
    Address as EncryptedPassportElementAddress,
    BankStatement as EncryptedPassportElementBankStatement,
    DriverLicense as EncryptedPassportElementDriverLicense, Email as EncryptedPassportElementEmail,
    EncryptedPassportElement, IdentityCard as EncryptedPassportElementIdentityCard,
    InternalPassport as EncryptedPassportElementInternalPassport,
    Passport as EncryptedPassportElementPassport,
    PassportRegistration as EncryptedPassportElementPassportRegistration,
    PersonalDetails as EncryptedPassportElementPersonalDetails,
    PhoneNumber as EncryptedPassportElementPhoneNumber,
    RentalAgreement as EncryptedPassportElementRentalAgreement,
    TemporaryRegistration as EncryptedPassportElementTemporaryRegistration,
    UtilityBill as EncryptedPassportElementUtilityBill,
};
pub use external_reply_info::{
    Animation as ExternalReplyInfoAnimation, Audio as ExternalReplyInfoAudio,
    Contact as ExternalReplyInfoContact, Dice as ExternalReplyInfoDice,
    Document as ExternalReplyInfoDocument, ExternalReplyInfo, Game as ExternalReplyInfoGame,
    Giveaway as ExternalReplyInfoGiveaway, GiveawayWinners as ExternalReplyInfoGiveawayWinners,
    Invoice as ExternalReplyInfoInvoice, Location as ExternalReplyInfoLocation,
    PaidMedia as ExternalReplyInfoPaidMedia, Photo as ExternalReplyInfoPhoto,
    Poll as ExternalReplyInfoPoll, Sticker as ExternalReplyInfoSticker,
    Story as ExternalReplyInfoStory, Venue as ExternalReplyInfoVenue,
    Video as ExternalReplyInfoVideo, VideoNote as ExternalReplyInfoVideoNote,
    Voice as ExternalReplyInfoVoice,
};
pub use file::File;
pub use force_reply::ForceReply;
pub use forum_topic::ForumTopic;
pub use forum_topic_closed::ForumTopicClosed;
pub use forum_topic_created::ForumTopicCreated;
pub use forum_topic_edited::ForumTopicEdited;
pub use forum_topic_reopened::ForumTopicReopened;
pub use game::Game;
pub use game_high_score::GameHighScore;
pub use general_forum_topic_hidden::GeneralForumTopicHidden;
pub use general_forum_topic_unhidden::GeneralForumTopicUnhidden;
pub use giveaway::Giveaway;
pub use giveaway_completed::GiveawayCompleted;
pub use giveaway_created::GiveawayCreated;
pub use giveaway_winners::GiveawayWinners;
pub use inaccessible_message::InaccessibleMessage;
pub use inline_keyboard_button::InlineKeyboardButton;
pub use inline_keyboard_markup::InlineKeyboardMarkup;
pub use inline_query::InlineQuery;
pub use inline_query_result::InlineQueryResult;
pub use inline_query_result_article::InlineQueryResultArticle;
pub use inline_query_result_audio::InlineQueryResultAudio;
pub use inline_query_result_cached_audio::InlineQueryResultCachedAudio;
pub use inline_query_result_cached_document::InlineQueryResultCachedDocument;
pub use inline_query_result_cached_gif::InlineQueryResultCachedGif;
pub use inline_query_result_cached_mpeg4_gif::InlineQueryResultCachedMpeg4Gif;
pub use inline_query_result_cached_photo::InlineQueryResultCachedPhoto;
pub use inline_query_result_cached_sticker::InlineQueryResultCachedSticker;
pub use inline_query_result_cached_video::InlineQueryResultCachedVideo;
pub use inline_query_result_cached_voice::InlineQueryResultCachedVoice;
pub use inline_query_result_contact::InlineQueryResultContact;
pub use inline_query_result_document::InlineQueryResultDocument;
pub use inline_query_result_game::InlineQueryResultGame;
pub use inline_query_result_gif::InlineQueryResultGif;
pub use inline_query_result_location::InlineQueryResultLocation;
pub use inline_query_result_mpeg4_gif::InlineQueryResultMpeg4Gif;
pub use inline_query_result_photo::InlineQueryResultPhoto;
pub use inline_query_result_venue::InlineQueryResultVenue;
pub use inline_query_result_video::InlineQueryResultVideo;
pub use inline_query_result_voice::InlineQueryResultVoice;
pub use inline_query_results_button::InlineQueryResultsButton;
pub use input_contact_message_content::InputContactMessageContent;
pub use input_file::{
    BufferedFile as InputBufferedFile, FSFile as InputFSFile, FileId as InputFileId, InputFile,
    StreamFile as InputStreamFile, UrlFile as InputUrlFile,
};
pub use input_invoice_message_content::InputInvoiceMessageContent;
pub use input_location_message_content::InputLocationMessageContent;
pub use input_media::InputMedia;
pub use input_media_animation::InputMediaAnimation;
pub use input_media_audio::InputMediaAudio;
pub use input_media_document::InputMediaDocument;
pub use input_media_photo::InputMediaPhoto;
pub use input_media_video::InputMediaVideo;
pub use input_message_content::InputMessageContent;
pub use input_paid_media::InputPaidMedia;
pub use input_paid_media_photo::InputPaidMediaPhoto;
pub use input_paid_media_video::InputPaidMediaVideo;
pub use input_poll_option::InputPollOption;
pub use input_sticker::InputSticker;
pub use input_text_message_content::InputTextMessageContent;
pub use input_venue_message_content::InputVenueMessageContent;
pub use invoice::Invoice;
pub use keyboard_button::KeyboardButton;
pub use keyboard_button_poll_type::KeyboardButtonPollType;
pub use keyboard_button_request_chat::KeyboardButtonRequestChat;
pub use keyboard_button_request_users::KeyboardButtonRequestUsers;
pub use labeled_price::LabeledPrice;
pub use link_preview_options::LinkPreviewOptions;
pub use location::Location;
pub use login_url::LoginUrl;
pub use mask_position::MaskPosition;
pub use maybe_inaccessible_message::MaybeInaccessibleMessage;
pub use menu_button::MenuButton;
pub use menu_button_commands::MenuButtonCommands;
pub use menu_button_default::MenuButtonDefault;
pub use menu_button_web_app::MenuButtonWebApp;
pub use message::{
    Animation as MessageAnimation, Audio as MessageAudio,
    ChannelChatCreated as MessageChannelChatCreated, ChatShared as MessageChatShared,
    ConnectedWebsite as MessageConnectedWebsite, Contact as MessageContact,
    DeleteChatPhoto as MessageDeleteChatPhoto, Dice as MessageDice, Document as MessageDocument,
    ForumTopicClosed as MessageForumTopicClosed, ForumTopicCreated as MessageForumTopicCreated,
    ForumTopicEdited as MessageForumTopicEdited, ForumTopicReopened as MessageForumTopicReopened,
    Game as MessageGame, GeneralForumTopicHidden as MessageGeneralForumTopicHidden,
    GeneralForumTopicUnhidden as MessageGeneralForumTopicUnhidden, Giveaway as MessageGiveaway,
    GiveawayCompleted as MessageGiveawayCompleted, GiveawayCreated as MessageGiveawayCreated,
    GiveawayWinners as MessageGiveawayWinners, GroupChatCreated as MessageGroupChatCreated,
    Invoice as MessageInvoice, LeftChatMember as MessageLeftChatMember,
    Location as MessageLocation, Message,
    MessageAutoDeleteTimerChanged as MessageMessageAutoDeleteTimerChanged,
    MigrateFromChat as MessageMigrateFromChat, MigrateToChat as MessageMigrateToChat,
    NewChatMembers as MessageNewChatMembers, NewChatPhoto as MessageNewChatPhoto,
    NewChatTitle as MessageNewChatTitle, PaidMedia as MessagePaidMedia,
    PassportData as MessagePassportData, Photo as MessagePhoto, Pinned as MessagePinned,
    Poll as MessagePoll, ProximityAlertTriggered as MessageProximityAlertTriggered,
    RefundedPayment as MessageRefundedPayment, Sticker as MessageSticker, Story as MessageStory,
    SuccessfulPayment as MessageSuccessfulPayment,
    SupergroupChatCreated as MessageSupergroupChatCreated, Text as MessageText,
    UsersShared as MessageUsersShared, Venue as MessageVenue, Video as MessageVideo,
    VideoChatEnded as MessageVideoChatEnded,
    VideoChatParticipantsInvited as MessageVideoChatParticipantsInvited,
    VideoChatScheduled as MessageVideoChatScheduled, VideoChatStarted as MessageVideoChatStarted,
    VideoNote as MessageVideoNote, Voice as MessageVoice, WebAppData as MessageWebAppData,
    WriteAccessAllowed as MessageWriteAccessAllowed,
};
pub use message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
pub use message_entity::{
    CustomEmoji as CustomEmojiMessageEntity, Kind as MessageEntityKind, MessageEntity,
    Pre as PreMessageEntity, TextLink as TextLinkMessageEntity,
    TextMention as TextMentionMessageEntity,
};
pub use message_id::MessageId;
pub use message_or_true::MessageOrTrue;
pub use message_origin::MessageOrigin;
pub use message_origin_channel::MessageOriginChannel;
pub use message_origin_chat::MessageOriginChat;
pub use message_origin_hidden_user::MessageOriginHiddenUser;
pub use message_origin_user::MessageOriginUser;
pub use message_reaction_count_updated::MessageReactionCountUpdated;
pub use message_reaction_updated::MessageReactionUpdated;
pub use order_info::OrderInfo;
pub use paid_media::PaidMedia;
pub use paid_media_info::PaidMediaInfo;
pub use paid_media_photo::PaidMediaPhoto;
pub use paid_media_preview::PaidMediaPreview;
pub use paid_media_video::PaidMediaVideo;
pub use passport_data::PassportData;
pub use passport_element_error::PassportElementError;
pub use passport_element_error_data_field::{
    ElementType as PassportElementErrorDataFieldType, PassportElementErrorDataField,
};
pub use passport_element_error_file::{
    ElementType as PassportElementErrorFileType, PassportElementErrorFile,
};
pub use passport_element_error_files::{
    ElementType as PassportElementErrorFilesType, PassportElementErrorFiles,
};
pub use passport_element_error_front_side::{
    ElementType as PassportElementErrorFrontSideType, PassportElementErrorFrontSide,
};
pub use passport_element_error_reverse_side::{
    ElementType as PassportElementErrorReverseSideType, PassportElementErrorReverseSide,
};
pub use passport_element_error_selfie::{
    ElementType as PassportElementErrorSelfieType, PassportElementErrorSelfie,
};
pub use passport_element_error_translation_file::{
    ElementType as PassportElementErrorTranslationFileType, PassportElementErrorTranslationFile,
};
pub use passport_element_error_translation_files::{
    ElementType as PassportElementErrorTranslationFilesType, PassportElementErrorTranslationFiles,
};
pub use passport_element_error_unspecified::PassportElementErrorUnspecified;
pub use passport_file::PassportFile;
pub use photo_size::PhotoSize;
pub use poll::{Poll, Quiz as PollQuiz, Regular as PollRegular};
pub use poll_answer::PollAnswer;
pub use poll_option::PollOption;
pub use pre_checkout_query::PreCheckoutQuery;
pub use proximity_alert_triggered::ProximityAlertTriggered;
pub use reaction_count::ReactionCount;
pub use reaction_type::ReactionType;
pub use reaction_type_custom_emoji::ReactionTypeCustomEmoji;
pub use reaction_type_emoji::ReactionTypeEmoji;
pub use reaction_type_paid::ReactionTypePaid;
pub use refunded_payment::RefundedPayment;
pub use reply_keyboard_markup::ReplyKeyboardMarkup;
pub use reply_keyboard_remove::ReplyKeyboardRemove;
pub use reply_markup::ReplyMarkup;
pub use reply_parameters::ReplyParameters;
pub use response_parameters::ResponseParameters;
pub use revenue_withdrawal_state::RevenueWithdrawalState;
pub use revenue_withdrawal_state_failed::RevenueWithdrawalStateFailed;
pub use revenue_withdrawal_state_pending::RevenueWithdrawalStatePending;
pub use revenue_withdrawal_state_succeeded::RevenueWithdrawalStateSucceeded;
pub use sent_web_app_message::SentWebAppMessage;
pub use shared_user::SharedUser;
pub use shipping_address::ShippingAddress;
pub use shipping_option::ShippingOption;
pub use shipping_query::ShippingQuery;
pub use star_transaction::{
    Receiver as StarTransactionReceiver, Source as StarTransactionSource, StarTransaction,
};
pub use star_transactions::StarTransactions;
pub use sticker::Sticker;
pub use sticker_set::StickerSet;
pub use story::Story;
pub use successful_payment::SuccessfulPayment;
pub use switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;
pub use text_quote::TextQuote;
pub use transaction_partner::TransactionPartner;
pub use transaction_partner_fragment::TransactionPartnerFragment;
pub use transaction_partner_other::TransactionPartnerOther;
pub use transaction_partner_telegram_ads::TransactionPartnerTelegramAds;
pub use transaction_partner_user::TransactionPartnerUser;
pub use update::{Kind as UpdateKind, Update};
pub use user::User;
pub use user_chat_boosts::UserChatBoosts;
pub use user_profile_photos::UserProfilePhotos;
pub use users_shared::UsersShared;
pub use venue::Venue;
pub use video::Video;
pub use video_chat_ended::VideoChatEnded;
pub use video_chat_participants_invited::VideoChatParticipantsInvited;
pub use video_chat_scheduled::VideoChatScheduled;
pub use video_chat_started::VideoChatStarted;
pub use video_note::VideoNote;
pub use voice::Voice;
pub use web_app_data::WebAppData;
pub use web_app_info::WebAppInfo;
pub use web_app_user::WebAppUser;
pub use webhook_info::WebhookInfo;
pub use write_access_allowed::WriteAccessAllowed;

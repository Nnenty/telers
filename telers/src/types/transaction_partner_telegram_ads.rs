use serde::{Deserialize, Serialize};

/// Describes a withdrawal transaction to the Telegram Ads platform.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnertelegramads>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransactionPartnerTelegramAds {}

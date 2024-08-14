use serde::{Deserialize, Serialize};

/// Describes a transaction with an unknown source or recipient.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnertelegramads>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransactionPartnerOther {}

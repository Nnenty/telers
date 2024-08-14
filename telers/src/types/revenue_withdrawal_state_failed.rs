use serde::{Deserialize, Serialize};

/// The withdrawal failed and the transaction was refunded.
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstatefailed>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct RevenueWithdrawalStateFailed {}

use serde::{Deserialize, Serialize};

/// The withdrawal is in progress.
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstatepending>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct RevenueWithdrawalStatePending {}

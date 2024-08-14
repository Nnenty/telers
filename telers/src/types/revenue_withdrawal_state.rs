use super::{
    RevenueWithdrawalStateFailed, RevenueWithdrawalStatePending, RevenueWithdrawalStateSucceeded,
};

use serde::{Deserialize, Serialize};

/// This object describes the state of a revenue withdrawal operation. Currently, it can be one of
/// - [`RevenueWithdrawalStatePending`]
/// - [`RevenueWithdrawalStateSucceeded`]
/// - [`RevenueWithdrawalStateFailed`]
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstate>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RevenueWithdrawalState {
    Pending(RevenueWithdrawalStatePending),
    Succeeded(RevenueWithdrawalStateSucceeded),
    Failed(RevenueWithdrawalStateFailed),
}

impl From<RevenueWithdrawalStatePending> for RevenueWithdrawalState {
    fn from(state: RevenueWithdrawalStatePending) -> Self {
        Self::Pending(state)
    }
}

impl From<RevenueWithdrawalStateSucceeded> for RevenueWithdrawalState {
    fn from(state: RevenueWithdrawalStateSucceeded) -> Self {
        Self::Succeeded(state)
    }
}

impl From<RevenueWithdrawalStateFailed> for RevenueWithdrawalState {
    fn from(state: RevenueWithdrawalStateFailed) -> Self {
        Self::Failed(state)
    }
}

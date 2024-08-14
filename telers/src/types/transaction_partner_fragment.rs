use super::RevenueWithdrawalState;

use serde::{Deserialize, Serialize};

/// Describes a withdrawal transaction with Fragment.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnerfragment>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TransactionPartnerFragment {
    /// State of the transaction if the transaction is outgoing
    pub withdrawal_state: RevenueWithdrawalState,
}

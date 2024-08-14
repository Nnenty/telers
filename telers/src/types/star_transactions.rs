use super::StarTransaction;

use serde::{Deserialize, Serialize};

/// Contains a list of Telegram Star transactions.
/// # Documentation
/// <https://core.telegram.org/bots/api#startransactions>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StarTransactions {
    /// The list of transactions
    pub transactions: Box<[StarTransaction]>,
}

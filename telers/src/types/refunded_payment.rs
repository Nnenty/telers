use serde::{Deserialize, Serialize};

/// This object contains basic information about a refunded payment.
/// # Documentation
/// <https://core.telegram.org/bots/api#refundedpayment>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct RefundedPayment {
    /// Three-letter ISO 4217 [`currency`](https://core.telegram.org/bots/payments#supported-currencies) code, or `XTR` for payments in [`Telegram Stars`](https://t.me/BotNews/90). Currently, always `XTR`
    pub currency: Box<str>,
    /// Total refunded price in the _smallest units_ of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45`, `total_amount = 145`. See the exp parameter in [`currencies.json`](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot-specified invoice payload
    pub invoice_payload: Box<str>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: Box<str>,
    /// Provider payment identifier
    pub provider_payment_charge_id: Option<Box<str>>,
}

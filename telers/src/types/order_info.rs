use super::ShippingAddress;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents information about an order.
/// # Documentation
/// <https://core.telegram.org/bots/api#orderinfo>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct OrderInfo {
    /// User name
    pub name: Option<Box<str>>,
    /// User's phone number
    pub phone_number: Option<Box<str>>,
    /// User email
    pub email: Option<Box<str>>,
    /// User shipping address
    pub shipping_address: Option<ShippingAddress>,
}

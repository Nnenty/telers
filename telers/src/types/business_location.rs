use super::Location;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// # Documentation
/// <https://core.telegram.org/bots/api#businesslocation>
#[skip_serializing_none]
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct BusinessLocation {
    /// Address of the business
    pub address: Box<str>,
    /// Location of the business
    pub location: Option<Location>,
}

use super::Location;

use serde::Deserialize;

/// # Documentation
/// <https://core.telegram.org/bots/api#businesslocation>
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct BusinessLocation {
    /// Address of the business
    pub address: Box<str>,
    /// Location of the business
    pub location: Option<Location>,
}

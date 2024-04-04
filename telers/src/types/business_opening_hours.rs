use super::BusinessOpeningHoursInterval;

use serde::Deserialize;

/// # Documentation
/// <https://core.telegram.org/bots/api#businessopeninghours>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct BusinessOpeningHours {
    /// Unique name of the time zone for which the opening hours are defined
    pub time_zone_name: Box<str>,
    /// List of time intervals describing business opening hours
    pub opening_hours: Box<[BusinessOpeningHoursInterval]>,
}

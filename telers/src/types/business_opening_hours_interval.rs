use serde::{Deserialize, Serialize};

/// # Documentation
/// <https://core.telegram.org/bots/api#businessopeninghoursinterval>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BusinessOpeningHoursInterval {
    /// The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0 - 7 * 24 * 60
    pub opening_minute: i64,
    /// The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 0 - 8 * 24 * 60
    pub closing_minute: i64,
}

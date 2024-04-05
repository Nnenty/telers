use serde::Deserialize;

/// # Documentation
/// <https://core.telegram.org/bots/api#birthdate>
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Birthdate {
    /// Day of the user's birth; 1-31
    pub day: i8,
    /// Month of the user's birth; 1-12
    pub month: i8,
    /// Year of the user's birth
    pub year: i16,
}

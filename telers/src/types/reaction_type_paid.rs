use serde::{Deserialize, Serialize};

/// The reaction is paid.
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontypepaid>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReactionTypePaid {}

impl ReactionTypePaid {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

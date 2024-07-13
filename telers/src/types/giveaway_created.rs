use serde::{Deserialize, Serialize};

/// This object represents a service message about the creation of a scheduled giveaway. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaycreated>
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct GiveawayCreated {}

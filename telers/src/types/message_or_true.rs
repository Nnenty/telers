use super::Message;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Message),
    True(bool),
}

use super::{EncryptedCredentials, EncryptedPassportElement};

use serde::{Deserialize, Serialize};

/// Describes Telegram Passport data shared with the bot by the user.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportdata>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    pub data: Box<[EncryptedPassportElement]>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}

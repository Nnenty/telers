use serde::{Deserialize, Serialize};

/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrortranslationfile>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct PassportElementErrorTranslationFile {
    /// Error source, must be *translation_file*
    pub source: Box<str>,
    #[serde(rename = "type")]
    pub element_type: Box<str>,
    /// Base64-encoded file hash
    pub file_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}

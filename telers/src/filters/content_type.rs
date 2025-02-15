use super::base::Filter;

use crate::{client::Bot, context::Context, enums::ContentType as ContentTypeEnum, types::Update};

use async_trait::async_trait;

/// Filter for checking the type of content
#[derive(Debug, Clone)]
pub struct ContentType {
    content_types: Box<[ContentTypeEnum]>,
}

impl ContentType {
    /// Creates a new [`ContentType`] filter with one allowed content type
    /// # Notes
    /// You can use [`ContentTypeEnum`] or its string representation
    #[must_use]
    pub fn one(content_type: impl Into<ContentTypeEnum>) -> Self {
        Self {
            content_types: [content_type.into()].into(),
        }
    }

    /// Creates a new [`ContentType`] filter with many allowed content types
    /// # Notes
    /// You can use [`ContentTypeEnum`] or its string representation
    #[must_use]
    pub fn many<T, I>(content_types: I) -> Self
    where
        T: Into<ContentTypeEnum>,
        I: IntoIterator<Item = T>,
    {
        Self {
            content_types: content_types.into_iter().map(Into::into).collect(),
        }
    }
}

impl ContentType {
    #[must_use]
    pub fn validate_content_type(&self, content_type: ContentTypeEnum) -> bool {
        self.content_types
            .iter()
            .any(|allowed_content_type| allowed_content_type == &content_type)
    }
}

#[async_trait]
impl<Client> Filter<Client> for ContentType {
    async fn check(&self, _bot: &Bot<Client>, update: &Update, _context: &Context) -> bool {
        let Some(message) = update.message() else {
            return false;
        };

        self.validate_content_type(ContentTypeEnum::from(message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_content_type() {
        let filter = ContentType::many([ContentTypeEnum::Text, ContentTypeEnum::Photo]);

        assert!(filter.validate_content_type(ContentTypeEnum::Text));
        assert!(filter.validate_content_type(ContentTypeEnum::Photo));
        assert!(!filter.validate_content_type(ContentTypeEnum::Audio));
    }
}

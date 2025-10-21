//! [![CI](https://github.com/m1sk9/serenity-embed-builder/actions/workflows/ci.yaml/badge.svg)](https://github.com/m1sk9/serenity-embed-builder/actions/workflows/ci.yaml)
//! [![Release serenity-embed-builder](https://github.com/m1sk9/serenity-embed-builder/actions/workflows/release.yaml/badge.svg)](https://github.com/m1sk9/serenity-embed-builder/actions/workflows/release.yaml)
//! [![Apache License 2.0](https://img.shields.io/github/license/m1sk9/babyrite?color=%239944ee)](https://github.com/m1sk9/babyrite/blob/main/LICENSE)
//!
//! A Builder to Make Serenity Embeds Easier to Handle.

#![deny(clippy::all)]
#![allow(dead_code)]

use crate::model::SerenityEmbed;
use serenity::all::{Colour, CreateEmbed};

mod model;

#[derive(thiserror::Error, Debug)]
pub enum SerenityEmbedConvertError {
    #[error("The description exceeds the maximum length of 4096 characters.")]
    DescriptionTooLong,
    #[error("The number of fields exceeds the maximum of 25.")]
    TooManyFields,
}

impl SerenityEmbed {
    /// Convert the embedded structure created in Builder into a model usable in Serenity.
    pub fn convert(&self) -> Result<CreateEmbed, SerenityEmbedConvertError> {
        let mut embed = serenity::builder::CreateEmbed::default();

        if let Some(title) = &self.title {
            embed = embed.title(title.text.clone()).url(title.url.clone())
        }

        if let Some(description) = &self.description {
            if description.len() >= 4096 {
                return Err(SerenityEmbedConvertError::DescriptionTooLong);
            }

            embed = embed.description(description);
        }

        if let Some(timestamp) = &self.timestamp {
            embed = embed.timestamp(timestamp);
        }

        if let Some(color) = &self.color {
            embed = embed.color(Colour(*color));
        }

        if let Some(footer_text) = &self.footer_text {
            let mut footer = serenity::builder::CreateEmbedFooter::new(footer_text);
            if let Some(icon_url) = &self.footer_icon_url {
                footer = footer.icon_url(icon_url);
            }
            embed = embed.footer(footer);
        }

        if let Some(image_url) = &self.image_url {
            embed = embed.image(image_url);
        }

        if let Some(thumbnail_url) = &self.thumbnail_url {
            embed = embed.thumbnail(thumbnail_url);
        }

        if let Some(author_name) = &self.author_name {
            let mut author = serenity::builder::CreateEmbedAuthor::new(author_name);
            if let Some(url) = &self.author_url {
                author = author.url(url);
            }
            if let Some(icon_url) = &self.author_icon_url {
                author = author.icon_url(icon_url);
            }
            embed = embed.author(author);
        }

        if let Some(fields) = &self.fields {
            if fields.len() > 25 {
                return Err(SerenityEmbedConvertError::TooManyFields);
            }
            // 明示的に (String, String, bool) を作って渡すことで
            // `Into<String>` の曖昧性（複数の impl による推論失敗）を回避する
            let mapped = fields
                .iter()
                .map(|f| (f.name.clone(), f.value.clone(), f.inline));
            embed = embed.fields(mapped)
        }

        Ok(embed)
    }
}

#[cfg(test)]
mod tests {
    use serenity::all::Timestamp;

    use super::*;
    use crate::model::{SerenityEmbedField, SerenityEmbedTitle};

    static MOCK_TEXT: &str = "This is a test text.";
    static MOCK_URL: &str = "https://example.com";
    static MOCK_TIMESTAMP_STR: &str = "2024-01-01T00:00:00Z";
    static MOCK_COLOR: u32 = 0xff0000;

    #[test]
    fn test_embed_conversion() {
        let fields = vec![
            SerenityEmbedField::builder()
                .name(MOCK_TEXT)
                .value(MOCK_TEXT)
                .inline(true)
                .build(),
            SerenityEmbedField::builder()
                .name(MOCK_TEXT)
                .value(MOCK_TEXT)
                .build(),
        ];

        // serenity-embed-builder
        let mock_embed = SerenityEmbed::builder()
            .title(
                SerenityEmbedTitle::builder()
                    .text(MOCK_TEXT)
                    .url(MOCK_URL)
                    .build(),
            )
            .description(MOCK_TEXT)
            .timestamp(Timestamp::parse(MOCK_TIMESTAMP_STR).unwrap())
            .color(MOCK_COLOR)
            .footer_text(MOCK_TEXT)
            .footer_icon_url(MOCK_URL)
            .image_url(MOCK_URL)
            .thumbnail_url(MOCK_URL)
            .author_name(MOCK_TEXT)
            .author_url(MOCK_URL)
            .author_icon_url(MOCK_URL)
            .fields(fields)
            .build();
        // serenity
        let serenity_embed = CreateEmbed::default()
            .title(MOCK_TEXT)
            .url(MOCK_URL)
            .description(MOCK_TEXT)
            .timestamp(Timestamp::parse(MOCK_TIMESTAMP_STR).unwrap())
            .color(Colour(MOCK_COLOR))
            .footer(serenity::builder::CreateEmbedFooter::new(MOCK_TEXT).icon_url(MOCK_URL))
            .image(MOCK_URL)
            .thumbnail(MOCK_URL)
            .author(
                serenity::builder::CreateEmbedAuthor::new(MOCK_TEXT)
                    .url(MOCK_URL)
                    .icon_url(MOCK_URL),
            )
            .fields(vec![
                (MOCK_TEXT.to_string(), MOCK_TEXT.to_string(), true),
                (MOCK_TEXT.to_string(), MOCK_TEXT.to_string(), false),
            ]);

        let converted = mock_embed.convert();

        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), serenity_embed);
    }
}

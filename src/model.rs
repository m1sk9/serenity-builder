/// A builder for creating Discord embeds.
/// This struct allows you to build rich embed messages for Discord using a fluent interface.
///
/// Fundamentally, this structure provides the same functionality as Serenity, but this library explicitly removes values that became unnecessary during optimization as a builder.
///
/// # Values
///
/// The following values have been removed from Serenity:
///
/// - `kind`: The type of embed. Discord currently only supports "rich" embeds, so this field is unnecessary.
/// - `video`: `rich` embeds do not support video content, so this field is unnecessary.
/// - `provider`: `rich` embeds do not support provider information, so this field is unnecessary.
///
/// [Serenity docs: model/channel/embed](https://docs.rs/serenity/latest/serenity/model/channel/struct.Embed.html)
#[derive(serde::Deserialize, typed_builder::TypedBuilder, Clone)]
pub struct SerenityEmbed {
    /// The title of the embed.
    #[builder(default, setter(strip_option, into))]
    pub title: Option<EmbedTitle>,
    /// The description of the embed. (up to 4096 characters)
    #[builder(default, setter(strip_option, into))]
    pub description: Option<String>,
    /**
     * The timestamp of the embed content.
     * It will be displayed at the bottom of the embed.
     */
    #[builder(default, setter(strip_option, into))]
    pub timestamp: Option<serenity::all::Timestamp>,
    /**
     * The color of the embed.
     * Serenity uses Colour, but Builder only allows direct specification from color codes.
     * e.g. `0xff0000` for red.
     */
    #[builder(default, setter(strip_option, into))]
    pub color: Option<u32>,
    /// The footer of the embed.
    #[builder(default, setter(strip_option, into))]
    pub footer: Option<SerenityEmbedFooter>,
    /// The image url of the embed.
    #[builder(default, setter(strip_option, into))]
    pub image_url: Option<String>,
    /// The thumbnail url of the embed.
    #[builder(default, setter(strip_option, into))]
    pub thumbnail_url: Option<String>,
    /// The author of the embed.
    #[builder(default, setter(strip_option, into))]
    pub author: Option<EmbedAuthor>,
    /// The fields of the embed. (up to 25 fields)
    #[builder(default, setter(strip_option, into))]
    pub fields: Option<Vec<SerenityEmbedField>>,
}

/// Structure for the title of the embed used by [EmbedBuilder].
#[derive(serde::Deserialize, typed_builder::TypedBuilder, Clone)]
pub struct SerenityEmbedTitle {
    /// The text of the embed title.
    #[builder(default, setter(strip_option, into))]
    pub title: Option<String>,
    /// The url of the embed title.
    #[builder(default, setter(strip_option, into))]
    pub url: Option<String>,
}

/// Type alias for [EmbedTitleBuilder].
pub type EmbedTitle = SerenityEmbedTitle;

/// Structure for the footer of the embed used by [EmbedBuilder].
#[derive(serde::Deserialize, typed_builder::TypedBuilder, Clone)]
pub struct SerenityEmbedFooter {
    /// The text of the embed footer.
    #[builder(default, setter(strip_option, into))]
    pub text: Option<String>,
    /// The icon url of the embed footer.
    #[builder(default, setter(strip_option, into))]
    pub icon_url: Option<String>,
}

/// Type alias for [EmbedFooterBuilder].
pub type EmbedFooter = SerenityEmbedFooter;

/// Structure for the author of the embed used by [EmbedBuilder].
#[derive(serde::Deserialize, typed_builder::TypedBuilder, Clone)]
pub struct SerenityEmbedAuthor {
    /// The name of the embed author.
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,
    /// The url of the embed author.
    #[builder(default, setter(strip_option, into))]
    pub url: Option<String>,
    /// The icon url of the embed author.
    #[builder(default, setter(strip_option, into))]
    pub icon_url: Option<String>,
}

/// Type alias for [EmbedAuthorBuilder].
pub type EmbedAuthor = SerenityEmbedAuthor;

/// Structure for the field of the embed used by [EmbedBuilder].
#[derive(serde::Deserialize, typed_builder::TypedBuilder, Clone)]
pub struct SerenityEmbedField {
    /// The name of the embed field.
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,
    /// The value of the embed field.
    #[builder(default, setter(strip_option, into))]
    pub value: Option<String>,
    /// Whether the field is inline.
    #[builder(default, setter(strip_option, into))]
    pub inline: Option<bool>,
}

/// Type alias for [EmbedFieldBuilder].
pub type EmbedField = SerenityEmbedField;

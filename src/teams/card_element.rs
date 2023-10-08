use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub enum Colour {
    #[default]
    Default,
    Dark,
    Light,
    Accent,
    Good,
    Warning,
    Attention,
}

#[derive(Default, Serialize, Deserialize)]
pub enum FontType {
    #[default]
    Default,
    Monospace,
}

#[derive(Default, Serialize, Deserialize)]
pub enum FontSize {
    #[default]
    Default,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Default, Serialize, Deserialize)]
pub enum FontWeight {
    #[default]
    Default,
    Lighter,
    Bolder,
}

#[derive(Default, Serialize, Deserialize)]
pub enum ImageSize {
    #[default]
    Auto,
    Stretch,
    Small,
    Medium,
    Large,
}

#[derive(Default, Serialize, Deserialize)]
pub enum ImageStyle {
    #[default]
    Default,
    Person,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CardElement {
    TextBlock {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        color: Option<Colour>,
        size: Option<FontSize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        weight: Option<FontWeight>,
        #[serde(default)]
        wrap: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "isSubtle")]
        is_subtle: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "maxLines")]
        max_lines: Option<String>,
        #[serde(rename = "fontType")]
        font_type: Option<FontType>,
    },
    Image {
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<ImageStyle>,
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<ImageSize>,
    },
}

impl Default for CardElement {
    fn default() -> Self {
        CardElement::TextBlock {
            text: "This is some text".to_string(),
            color: Some(Colour::Default),
            size: Some(FontSize::Default),
            weight: Some(FontWeight::Default),
            wrap: true,
            is_subtle: None,
            max_lines: None,
            font_type: Some(FontType::Default),
        }
    }
}

impl Display for CardElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self).unwrap();
        write!(f, "{}", json)
    }
}

impl CardElement {
    pub fn text_block(text: &str) -> Self {
        CardElement::TextBlock {
            text: text.to_string(),
            color: Some(Colour::Default),
            size: Some(FontSize::Default),
            weight: Some(FontWeight::Default),
            wrap: true,
            is_subtle: None,
            max_lines: None,
            font_type: Some(FontType::Default),
        }
    }
    pub fn heading_block(text: &str) -> Self {
        CardElement::TextBlock {
            text: text.to_string(),
            color: Some(Colour::Default),
            size: Some(FontSize::Large),
            weight: Some(FontWeight::Bolder),
            wrap: true,
            is_subtle: None,
            max_lines: None,
            font_type: Some(FontType::Default),
        }
    }
    pub fn _image(url: &str) -> Self {
        CardElement::Image {
            url: url.to_string(),
            style: None,
            size: None,
        }
    }
}

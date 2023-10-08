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
        #[serde(default)]
        size: String,
        #[serde(default)]
        style: String,
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

use std::fmt::Display;

use super::card_element::CardElement;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdaptiveCard {
    #[serde(rename = "type")]
    card_type: String,
    body: Vec<CardElement>,
    #[serde(rename = "$schema")]
    schema: String,
    version: String,
}

impl Default for AdaptiveCard {
    fn default() -> Self {
        AdaptiveCard {
            card_type: "AdaptiveCard".to_string(),
            body: vec![CardElement::default()],
            schema: "http://adaptivecards.io/schemas/adaptive-card.json".to_string(),
            version: "1.6".to_string(),
        }
    }
}

impl Display for AdaptiveCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self).unwrap();
        write!(f, "{{\"type\": \"message\", \"attachments\": [{}]}}", json)
    }
}

impl AdaptiveCard {
    pub fn test_card() -> Self {
        Self::default()
    }
}

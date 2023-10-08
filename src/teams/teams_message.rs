use super::adaptive_card::AdaptiveCard;
use super::publish::PublishType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    message_type: String,
    #[serde(rename = "attachments")]
    attachments: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "contentType")]
    content_type: String,
    content: AdaptiveCard,
}

impl From<AdaptiveCard> for Message {
    fn from(card: AdaptiveCard) -> Self {
        Message {
            message_type: "message".to_string(),
            attachments: vec![Content {
                content_type: "application/vnd.microsoft.card.adaptive".to_string(),
                content: card,
            }],
        }
    }
}

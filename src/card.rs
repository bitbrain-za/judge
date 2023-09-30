use std::fmt::Display;

use chrono::prelude::*;
use scoreboard_db::Score;
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

impl Message {
    pub fn new(score: &Score, leaders: &[Score]) -> Self {
        let card = AdaptiveCard::new(score, leaders);
        let content = Content {
            content_type: "application/vnd.microsoft.card.adaptive".to_string(),
            content: card,
        };

        let attachments = vec![content];
        Message {
            message_type: "message".to_string(),
            attachments,
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self).unwrap();
        let json = json.replace("{n}", r"\n");
        write!(f, "{}", json)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AdaptiveCard {
    #[serde(rename = "type")]
    card_type: String,
    body: Vec<CardElement>,
    #[serde(rename = "$schema")]
    schema: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum CardElement {
    TextBlock {
        size: String,
        weight: String,
        text: String,
        #[serde(default)]
        wrap: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        spacing: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "isSubtle")]
        is_subtle: Option<bool>,
    },
    ColumnSet {
        columns: Vec<Column>,
    },
    Image {
        url: String,
        #[serde(default)]
        size: String,
        #[serde(default)]
        style: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Column {
    items: Vec<CardElement>,
    #[serde(default)]
    width: String,
}

impl AdaptiveCard {
    pub fn new(score: &Score, leaders: &[Score]) -> Self {
        let card = AdaptiveCard::default();
        let now: String = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

        let s: String = serde_json::to_string(&card).unwrap();
        let s = s.replace("{PLAYER}", &score.name);
        let s = s.replace("{DATE}", &now);

        let mut body = format!("{}", score);
        body.push_str("{n}{n}");
        body.push_str("Leaderboard:{n}{n}");
        for (i, leader) in leaders.iter().enumerate() {
            body.push_str(&format!("{}. {}{{n}}", i + 1, leader));
        }

        let s = s.replace("{BODY}", &body);
        serde_json::from_str(&s).unwrap()
    }
}

impl Default for AdaptiveCard {
    fn default() -> Self {
        AdaptiveCard {
            card_type: "AdaptiveCard".to_string(),
            body: vec![
                CardElement::TextBlock {
                    size: "large".to_string(),
                    weight: "bolder".to_string(),
                    text: "New Run!".to_string(),
                    wrap: false,
                    spacing: None,
                    is_subtle: None,
                },
                CardElement::TextBlock {
                    size: "Medium".to_string(),
                    weight: "Lighter".to_string(),
                    text: "{BODY}".to_string(),
                    wrap: true,
                    spacing: None,
                    is_subtle: Some(false),
                },
            ],
            schema: "http://adaptivecards.io/schemas/adaptive-card.json".to_string(),
            version: "1.6".to_string(),
        }
    }
}

use std::fmt::Display;

use super::card_element::CardElement;
use super::publish::PublishType;
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

impl From<PublishType> for AdaptiveCard {
    fn from(publish: PublishType) -> Self {
        match publish {
            PublishType::Message(msg) => AdaptiveCard {
                body: vec![CardElement::text_block(&msg)],
                ..Default::default()
            },
            PublishType::NewScore((challenge, score, scores)) => {
                let heading = format!("New attempt at {}!", &challenge);
                let subheading = score.to_string();
                let scoreboard_title = "Leaderboard".to_string();

                let mut scoreboard = String::new();
                for (i, score) in scores.iter().enumerate() {
                    scoreboard.push_str(&format!("{}. {}\n", i + 1, score));
                }
                AdaptiveCard {
                    body: vec![
                        CardElement::heading_block(&heading),
                        CardElement::text_block(&subheading),
                        CardElement::text_block(&scoreboard_title),
                        CardElement::text_block(&scoreboard),
                    ],
                    ..Default::default()
                }
            }
            PublishType::Announcement((heading, msg)) => AdaptiveCard {
                body: vec![
                    CardElement::heading_block(&heading),
                    CardElement::text_block(&msg),
                ],
                ..Default::default()
            },
            PublishType::Prize((challenge, score)) => {
                let heading = format!("{} claims the spot prize for {}!", &score.name, &challenge);
                let subheading = format!("For the first submission using {}. To claim your prize, reply here with your code", score.language);
                AdaptiveCard {
                    body: vec![
                        CardElement::heading_block(&heading),
                        CardElement::text_block(&subheading),
                    ],
                    ..Default::default()
                }
            }
            PublishType::CopyCard {
                challenge,
                thief,
                victim,
                scores,
            } => {
                let heading = format!("Old attempt at {}!", &challenge);
                let subheading = format!("{}, tried to submit code already submitted by {}. The leaderboard remains the same...", thief, victim);
                let scoreboard_title = "Leaderboard".to_string();

                let mut scoreboard = String::new();
                for (i, score) in scores.iter().enumerate() {
                    scoreboard.push_str(&format!("{}. {}\n", i + 1, score));
                }
                AdaptiveCard {
                    body: vec![
                        CardElement::heading_block(&heading),
                        CardElement::text_block(&subheading),
                        CardElement::text_block(&scoreboard_title),
                        CardElement::text_block(&scoreboard),
                    ],
                    ..Default::default()
                }
            }
        }
    }
}

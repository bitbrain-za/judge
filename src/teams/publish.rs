use super::{adaptive_card::AdaptiveCard, teams_message::Message};
use log::debug;
use scoreboard_db::Score;
use serde_json;
use std::{error::Error, fmt::Display};

pub enum PublishType {
    NewScore((String, Score, Vec<Score>)),
    CopyCard {
        challenge: String,
        thief: String,
        victim: String,
        scores: Vec<Score>,
    },
    Announcement((String, String)),
    Prize((String, Score)),
    Message(String),
}

impl Display for PublishType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PublishType::Message(msg) => write!(f, "{{\"text\": \"{}\"}}", msg),
            _ => todo!(),
        }
    }
}

pub struct Publisher {
    webhook: String,
}

impl Publisher {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let webhook = match option_env!("WEBHOOK") {
            Some(pass) => pass,
            None => {
                return Err(
                    "This program needs to be compiled with the $WEBHOOK env variable set".into(),
                )
            }
        }
        .to_string();
        Ok(Publisher { webhook })
    }

    pub fn publish(&self, content: PublishType) -> Result<(), Box<dyn std::error::Error>> {
        let message = Message::from(content);
        self.send(&message)
    }

    fn send(&self, body: &Message) -> Result<(), Box<dyn std::error::Error>> {
        debug!("BODY: {}", serde_json::to_string_pretty(body)?);
        let client = reqwest::blocking::Client::new();
        let req = client
            .post(&self.webhook)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(body);

        debug!("Request: {:?}", req);

        let res = req.send()?;
        debug!("Response: {:?}", res);
        debug!("Response: {:?}", res.text()?);
        Ok(())
    }

    pub fn send_test_card(&self) {
        let adaptive_card = AdaptiveCard::test_card();
        let message: Message = Message::from(adaptive_card);

        self.send(&message).unwrap();
    }
}

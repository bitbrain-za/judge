use super::{adaptive_card::AdaptiveCard, card, teams_message::Message};
use log::debug;
use scoreboard_db::Score;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{error::Error, fmt::Display};

pub enum PublishType {
    NewScore((Score, Vec<Score>)),
    CopyCard(Score),
    Announcement(String),
    Prize((Score, Vec<Score>)),
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
        self.send(&format!("{}", content))
    }

    fn send(&self, body: &str) -> Result<(), Box<dyn std::error::Error>> {
        let body = String::from(body);
        debug!("BODY: {:?}", body);
        let client = reqwest::blocking::Client::new();
        let req = client
            .post(&self.webhook)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body);

        debug!("Request: {:?}", req);

        let res = req.send()?;
        debug!("Response: {:?}", res);
        debug!("Response: {:?}", res.text()?);
        Ok(())
    }

    fn send_message(&self, body: &Message) -> Result<(), Box<dyn std::error::Error>> {
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
        let message: Message = Message::new_adaptive_card(adaptive_card);

        self.send_message(&message).unwrap();
    }
}

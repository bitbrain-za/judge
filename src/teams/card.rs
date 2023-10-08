use chrono::prelude::*;
use log::debug;
use reqwest::header::CONTENT_TYPE;
use scoreboard_db::Builder as FilterBuilder;
use scoreboard_db::{Db, Filter, Score, ScoreBoard, SortColumn};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
    const MAX_SCORES: Option<usize> = Some(1000);
    pub fn new(score: &Score, leaders: &[Score], challenge: &str) -> Self {
        let card = AdaptiveCard::new(score, leaders, challenge);
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

    pub fn send_card(
        db: &mut Db,
        score: &Score,
        challenge: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let scores: Vec<Score> = db.get_scores(Self::MAX_SCORES)?;

        let filters = FilterBuilder::new()
            .add_filter(Filter::UniquePlayers)
            .add_filter(Filter::Sort(SortColumn::Time))
            .add_filter(Filter::Top(5));
        let scores = ScoreBoard::new(scores.clone())
            .filter(filters.clone())
            .scores();

        let card = Message::new(score, &scores, challenge);
        card.send()
    }

    pub fn send_copy_message(
        db: &mut Db,
        t: &str,
        v: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let scores: Vec<Score> = db.get_scores(Self::MAX_SCORES)?;

        let filters = FilterBuilder::new()
            .add_filter(Filter::UniquePlayers)
            .add_filter(Filter::Sort(SortColumn::Time))
            .add_filter(Filter::Top(5));

        let scores = ScoreBoard::new(scores.clone())
            .filter(filters.clone())
            .scores();

        let card = AdaptiveCard::new_cheat(t, v, &scores);
        let content = Content {
            content_type: "application/vnd.microsoft.card.adaptive".to_string(),
            content: card,
        };

        let attachments = vec![content];
        let card = Message {
            message_type: "message".to_string(),
            attachments,
        };
        card.send()
    }

    fn send(&self) -> Result<(), Box<dyn std::error::Error>> {
        let body = format!("{}", self);
        let hook = match option_env!("WEBHOOK") {
            Some(pass) => pass,
            None => {
                return Err(
                    "This program needs to be compiled with the $WEBHOOK env variable set".into(),
                )
            }
        };
        debug!("Card body: {}", body);

        let client = reqwest::blocking::Client::new();
        let req = client
            .post(hook)
            .header(CONTENT_TYPE, "application/json")
            .body(body);

        debug!("Request: {:?}", req);

        let res = req.send()?;
        debug!("Response: {:?}", res);
        debug!("Response: {:?}", res.text()?);
        Ok(())
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
    pub fn new(score: &Score, leaders: &[Score], challenge: &str) -> Self {
        let card = AdaptiveCard::default();
        let now: String = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

        let s: String = serde_json::to_string(&card).unwrap();
        let s = s.replace("{PLAYER}", &score.name);
        let s = s.replace("{CHALLENGE}", challenge);
        let s = s.replace("{DATE}", &now);

        let mut body = format!("{}", score);
        Self::append_score(&mut body, leaders);

        let s = s.replace("{BODY}", &body);
        serde_json::from_str(&s).unwrap()
    }

    pub fn new_cheat(thief: &str, victim: &str, leaders: &[Score]) -> Self {
        let card = AdaptiveCard::default();
        let now: String = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

        let s: String = serde_json::to_string(&card).unwrap();
        let s = s.replace("{PLAYER}", thief);
        let s = s.replace("{DATE}", &now);

        let mut body = format!(
            "{}, tried to submit code already submitted by {}. The leaderboard remains the same...",
            thief, victim
        );
        Self::append_score(&mut body, leaders);

        let s = s.replace("{BODY}", &body);
        serde_json::from_str(&s).unwrap()
    }

    pub fn filter_unique_players(input: &[Score], limit: Option<usize>) -> Vec<Score> {
        let limit = limit.unwrap_or(input.len());
        let mut scores: Vec<Score> = Vec::new();
        let mut seen = Vec::new();
        for score in input {
            if !seen.contains(&score.name) {
                seen.push(score.name.clone());
                scores.push(score.clone());
                if scores.len() >= limit {
                    break;
                }
            }
        }
        scores
    }

    fn append_score(body: &mut String, scores: &[Score]) {
        body.push_str("{n}{n}");
        body.push_str("Leaderboard:{n}{n}");

        let scores = Self::filter_unique_players(scores, Some(5));

        for (i, leader) in scores.iter().enumerate() {
            body.push_str(&format!("{}. {}{{n}}", i + 1, leader));
        }
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
                    text: "New Run for {CHALLENGE}!".to_string(),
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

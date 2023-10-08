use chrono::prelude::*;
use scoreboard_db::Score;
use serde::Deserialize;
use std::{error::Error, fs::File, io::Read};

#[derive(Deserialize, Debug)]
pub struct ChallengeConfig {
    id: String,
    start: String,
    end: String,
    language_prize: String,
    #[serde(skip_deserializing)]
    start_dt: DateTime<FixedOffset>,
    #[serde(skip_deserializing)]
    end_dt: DateTime<FixedOffset>,
    #[serde(skip_deserializing)]
    status: Status,
}

impl ChallengeConfig {
    pub fn get_status(&self) -> Status {
        let now = chrono::Utc::now();
        let tz: FixedOffset = FixedOffset::east_opt(2 * 60 * 60).unwrap();
        let now = now.with_timezone(&tz);

        if now < self.start_dt {
            return Status::Closed;
        }
        if now > self.end_dt {
            return Status::Past;
        }
        Status::Active
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.start_dt = self.start.parse::<DateTime<FixedOffset>>()?;
        self.end_dt = self.end.parse::<DateTime<FixedOffset>>()?;
        self.status = self.get_status();

        Ok(())
    }

    pub fn gets_a_prize(&self, language: &str, previous: &[Score]) -> Result<bool, Box<dyn Error>> {
        if self.language_prize == language {
            return self.is_prize_not_claimed(previous);
        }
        Ok(false)
    }

    fn is_prize_not_claimed(&self, past: &[Score]) -> Result<bool, Box<dyn Error>> {
        for score in past {
            if score.language == self.language_prize {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[derive(Default, Debug)]
pub enum Status {
    Active,
    Past,
    #[default]
    Closed,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    challenges: Vec<ChallengeConfig>,
}

impl Settings {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut settings: Settings = serde_json::from_str(&contents)?;
        settings.update_challenges()?;
        Ok(settings)
    }

    fn update_challenges(&mut self) -> Result<(), Box<dyn Error>> {
        for challenge in &mut self.challenges {
            challenge.update()?;
        }
        Ok(())
    }

    pub fn gets_a_prize(
        &self,
        id: &str,
        language: &str,
        previous: &[Score],
    ) -> Result<bool, Box<dyn Error>> {
        let challenge = self.get_challenge(id)?;
        challenge.gets_a_prize(language, previous)
    }

    fn get_challenge(&self, id: &str) -> Result<&ChallengeConfig, Box<dyn Error>> {
        for challenge in &self.challenges {
            if id == challenge.id {
                return Ok(challenge);
            }
        }
        Err("Challenge not found".into())
    }
}

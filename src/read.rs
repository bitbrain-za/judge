use std::fmt::Display;

use crate::config;
use scoreboard_db::Builder as FilterBuilder;
use scoreboard_db::{Db, NiceTime, Score, ScoreBoard};

pub struct Reader {
    pub scores: Vec<Score>,
    pub filters: FilterBuilder,
}

impl Reader {
    const MAX_SCORES: Option<usize> = Some(1000);
    pub fn new(scores: Vec<Score>) -> Self {
        Self {
            filters: FilterBuilder::new(),
            scores,
        }
    }

    pub fn filter(self, filters: FilterBuilder) -> Self {
        Self {
            filters,
            scores: self.scores,
        }
    }

    pub fn scores(&self) -> Result<Vec<Score>, Box<dyn std::error::Error>> {
        Ok(ScoreBoard::new(self.scores.clone())
            .filter(self.filters.clone())
            .scores())
    }

    pub fn pretty_print(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut s = String::from("# Scoreboard\n");
        s = format!("{}| # | Player |  Time | Language | Program |\n", s);
        s = format!("{}| --- | --- | --- | ------- |\n", s);
        let scores = self.scores()?;
        for (i, score) in scores.iter().enumerate() {
            s = format!(
                "{}| {place} | {player} | {time} | {language} | {program} |\n",
                s,
                player = score.name,
                time = NiceTime::new(score.time_ns),
                language = score.language,
                program = score.command,
                place = i + 1,
            );
        }

        termimad::print_text(&s);

        Ok(())
    }
}

impl Display for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("--=Scoreboard==--\n");
        let scores = match self.scores() {
            Ok(scores) => scores,
            Err(e) => {
                return write!(f, "Failed to read scores: {}", e);
            }
        };
        for (i, score) in scores.iter().enumerate() {
            s = format!("{}{}. {}\n", s, i + 1, score);
        }
        write!(f, "{}--==/==--", s)
    }
}

pub fn read_scores(
    config: config::ReadConfig,
    db: &mut Db,
) -> Result<Reader, Box<dyn std::error::Error>> {
    let scores: Vec<Score> = db.get_scores(Reader::MAX_SCORES)?;
    Ok(Reader::new(scores).filter(config.filters))
}

use crate::config;
use scoreboard_db::{Db, Score};

pub fn read_scores(
    config: config::ReadConfig,
    db: &mut Db,
) -> Result<Vec<Score>, Box<dyn std::error::Error>> {
    let scores: Vec<Score> = db.get_scores(config.limit, config.all)?;

    println!("--==Scoreboard==--");
    for (i, score) in scores.iter().enumerate() {
        println!("{}. {}", i + 1, score);
    }
    println!("--==/==--");
    Ok(scores)
}

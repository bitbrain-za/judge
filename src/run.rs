use crate::card::Message;
use crate::config::WriteConfig;
use crate::generator::{challenges::Challenges, TestResult};
use log::{debug, warn};
use scoreboard_db::{Db, NiceTime, Score};
use sha256::try_digest;
use std::path::Path;

pub fn run(
    db: &mut Db,
    config: &WriteConfig,
    count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut spinner = cliclack::spinner();
    spinner.start("Preparing for challenge");
    let hash = get_hash(&config.command)?;
    debug!("hash: {}", hash);
    let score = Score::new(&config.name, &config.command, 0.0, hash, &config.language);

    spinner.start("Generating challenge data");
    let challenges = Challenges::new();
    let mut generator = match challenges.make_generator(&config.challenge.command, count) {
        Some(g) => g,
        None => {
            warn!("Failed to create generator for {}", config.challenge.name);
            return Ok(());
        }
    };
    generator.setup()?;
    spinner.stop("Done setting up");

    let result = generator.run(TestResult::Success(score))?;
    let result = check_for_plagiarism(result, db)?;

    match result {
        TestResult::Fail(msg) => {
            warn!("Test failed: {}", msg);
        }
        TestResult::Success(score) => {
            if !config.test_mode {
                db.insert_score(&score)?;
            }

            println!(
                "Well done {name}, you ran {command} in {elapsed}",
                name = config.name,
                command = config.command,
                elapsed = NiceTime::new(score.time_ns)
            );
            if config.publish {
                let _ = Message::send_card(db, &score);
            }
        }
        TestResult::Stolen(thief, victim) => {
            println!(
                "Well done {name}, you ran {command} in {elapsed}",
                name = config.name,
                command = config.command,
                elapsed = NiceTime::new(thief.time_ns)
            );
            println!(
                "Unfortunately this solution was already submitted by {}",
                victim.name
            );
            if !config.test_mode {
                let _ = Message::send_copy_message(db, &thief.name, &victim.name);
            }
        }
    }

    Ok(())
}

fn get_hash(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parts = path.split(' ');
    let path = parts.last().ok_or("Failed to parse path")?;
    // let path = path.replace(&['(', ')', ',', '\"', '.', ';', ':', '/'][..], "");
    debug!("Is this the program: {}?", path);

    match try_digest(Path::new(path)) {
        Ok(hash) => Ok(hash),
        Err(e) => Err(format!("Failed to load file: {}", e).into()),
    }
}

fn check_for_plagiarism(
    result: TestResult,
    db: &mut Db,
) -> Result<TestResult, Box<dyn std::error::Error>> {
    if let TestResult::Success(ref new_score) = result {
        let scores: Vec<Score> = db.get_scores(None)?;
        for score in scores {
            if score.hash == new_score.hash && score.name != new_score.name {
                return Ok(TestResult::Stolen(new_score.clone(), score));
            }
        }
    }
    Ok(result)
}

use crate::card::Message;
use crate::config::WriteConfig;
use crate::generator;
use log::{debug, info, warn};
use scoreboard_db::{Db, NiceTime, Score};
use sha256::try_digest;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

#[derive(Clone)]
enum TestResult {
    Success(Score),
    Fail(String),
    Stolen(String, String),
}

const PATH: &str = "test.json";

/* wrap run_sim so we cleanup after ourselves */
pub fn run(
    db: &mut Db,
    config: &WriteConfig,
    count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = run_sim(db, config, count);
    std::fs::remove_file(PATH).expect("could not remove file");
    result
}

fn run_sim(
    db: &mut Db,
    config: &WriteConfig,
    count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("setting up to run {}", config.command);
    let gen = generator::Generator::new(count);
    gen.save_to_file(PATH)?;
    let hash = get_hash(&config.command)?;
    debug!("hash: {}", hash);
    let mut score = Score::new(&config.name, &config.command, 0.0, hash);

    // run the test
    let ex = format!("{} {}", config.command, PATH);
    let mut uut = Command::new("sh");
    uut.arg("-c").arg(ex);
    info!("Running {}", config.command);

    let start = Instant::now();
    let output = uut
        .output()
        .map_err(|e| format!("Failed to run your program: {}", e))?;

    let elapsed = start.elapsed().as_nanos();
    let result = if elapsed > u64::MAX as u128 {
        TestResult::Fail(String::from("Your program is way too slow"))
    } else {
        score.time_ns = elapsed as f64;
        match output.status.success() {
            false => TestResult::Fail(format!(
                "Your program exited with a non-zero status code: {}",
                String::from_utf8(output.stderr)?
            )),
            true => {
                let out = String::from_utf8(output.stdout)?;
                match gen.check_answer(&out)? {
                    false => TestResult::Fail(String::from(
                        "Your program did not produce the correct result",
                    )),
                    true => TestResult::Success(score),
                }
            }
        }
    };

    let result = check_for_plagiarism(result, db)?;

    match result {
        TestResult::Fail(msg) => {
            warn!("Test failed: {}", msg);
        }
        TestResult::Success(score) => {
            if !config.test_mode {
                db.insert_score(&score)?;
            }

            info!(
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
            info!(
                "Unfortunately this solution was already submitted by {}",
                victim
            );
            if !config.test_mode {
                let _ = Message::send_copy_message(db, &thief, &victim);
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
        let scores: Vec<Score> = db.get_scores(None, true)?;
        for score in scores {
            if score.hash == new_score.hash && score.name != new_score.name {
                return Ok(TestResult::Stolen(new_score.name.clone(), score.name));
            }
        }
    }
    Ok(result)
}

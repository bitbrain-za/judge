use crate::card::Message;
use crate::config::WriteConfig;
use crate::generator;
use log::{info, warn};
use scoreboard_db::{Db, NiceTime, Score};
use std::process::Command;
use std::time::Instant;

enum TestResult {
    Success(Score),
    Fail(String),
}

pub fn run_sim(
    db: &mut Db,
    config: &WriteConfig,
    count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    const PATH: &str = "test.json";
    let gen = generator::Generator::new(count);
    gen.save_to_file(PATH)?;
    let mut score = Score::new(&config.name, &config.command, 0.0);

    // run the test
    let ex = format!("{} {}", config.command, PATH);
    let mut uut = Command::new("sh");
    uut.arg("-c").arg(ex);

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
    }

    std::fs::remove_file(PATH).expect("could not remove file");
    Ok(())
}

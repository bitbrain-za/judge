mod config;
use reqwest::header::CONTENT_TYPE;
use scoreboard_db::{Db, NiceTime, Score};
mod debug_config;
mod generator;
use log::{debug, error, info, warn};
use std::process::Command;
use std::time::Instant;
mod card;
use card::Message;

const TEST_SAMPLES: usize = 100_000;

enum TestResult {
    Success(Score),
    Fail(String),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    debug_config::init_debug(&args);

    let db_pass = match option_env!("DB_PASSWORD") {
        Some(pass) => pass,
        None => {
            return Err(
                "This program needs to be compiled with the $DB_PASSWORD env variable set".into(),
            )
        }
    };

    let mut db = match Db::new("localhost", 3306, "code_challenge", db_pass, "23_3_1") {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            return Ok(());
        }
    };

    let config = match config::RunMode::from_args(&args) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to parse arguments: {}", e);
            return Ok(());
        }
    };
    debug!("Config: {:?}", config);

    match config {
        config::RunMode::Update(config) => {
            info!("Welcome to the code challenge {}!", whoami::realname());
            match run_sim(&mut db, &config.name, &config.command, config.publish) {
                Ok(_) => {}
                Err(e) => {
                    warn!("Failed to run your program: {}", e);
                }
            }
        }
        config::RunMode::Read(config) => match read_scores(config, &mut db) {
            Ok(_) => {}
            Err(e) => {
                warn!("Failed to read scores: {}", e);
            }
        },
        config::RunMode::Wipe => match db.clear_table() {
            Ok(_) => {}
            Err(e) => {
                warn!("Failed to wipe scores: {}", e);
            }
        },
    }
    Ok(())
}

fn read_scores(
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

fn run_sim(
    db: &mut Db,
    name: &str,
    command: &str,
    publish: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    const PATH: &str = "test.json";
    let gen = generator::Generator::new(TEST_SAMPLES);
    gen.save_to_file(PATH)?;
    let mut score = Score::new(name, command, 0.0);

    // run the test
    let ex = format!("{} {}", command, PATH);
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
            db.insert_score(&score)?;

            info!(
                "Well done {name}, you ran {command} in {elapsed}",
                name = name,
                command = command,
                elapsed = NiceTime::new(score.time_ns)
            );
            if publish {
                let _ = send_card(db, &score);
            }
        }
    }

    std::fs::remove_file(PATH).expect("could not remove file");
    Ok(())
}

fn send_card(db: &mut Db, score: &Score) -> Result<(), Box<dyn std::error::Error>> {
    let scores: Vec<Score> = db.get_scores(Some(3), false)?;
    let card = Message::new(score, &scores);
    let body = format!("{}", card);

    let hook = match option_env!("WEBHOOK") {
        Some(pass) => pass,
        None => {
            return Err(
                "This program needs to be compiled with the $WEBHOOK env variable set".into(),
            )
        }
    };

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

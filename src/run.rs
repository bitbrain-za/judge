use crate::config::WriteConfig;
use crate::generator::{challenges::Challenges, TestResult};
use crate::settings;
use crate::teams::publish;
use log::{debug, error, warn};
use scoreboard_db::{
    Builder as FilterBuilder, Db, Filter, NiceTime, Score, ScoreBoard, SortColumn,
};
use sha256::try_digest;
use std::path::Path;

pub fn run(db: &mut Db, config: &WriteConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut spinner = cliclack::spinner();
    spinner.start("Preparing for challenge");
    let hash = get_hash(&config.command)?;
    debug!("hash: {}", hash);
    let score = Score::new(&config.name, &config.command, 0.0, hash, &config.language);

    spinner.start("Generating challenge data");
    let challenges = Challenges::new();
    let mut generator = match challenges.make_generator(&config.challenge.command, config.test_mode)
    {
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
                /* Check if a spot prize is awarded */
                let previous = db.get_scores(None)?;

                let settings_path = match option_env!("SETTINGS_PATH") {
                    Some(path) => path,
                    None => return Err(
                        "This program needs to be compiled with the $SETTINGS_PATH env variable set"
                            .into(),
                    ),
                };
                let settings = match settings::Settings::load(settings_path) {
                    Ok(settings) => settings,
                    Err(e) => {
                        error!("Failed to load settings: {}", e);
                        return Ok(());
                    }
                };

                let prize =
                    settings.gets_a_prize(&config.challenge.command, &score.language, &previous)?;
                db.insert_score(&score)?;

                if prize {
                    cliclack::note(
                        "Congratulations",
                        format!(
                            "You have won a spot prize for the first {} submission",
                            score.language
                        ),
                    )?;

                    let _ = publish::Publisher::new()?.publish(publish::PublishType::Prize((
                        config.challenge.name.clone(),
                        score.clone(),
                    )));
                }
            }

            println!(
                "Well done {name}, you ran {command} in {elapsed}",
                name = config.name,
                command = config.command,
                elapsed = NiceTime::new(score.time_ns)
            );

            if config.publish {
                let scores = db.get_scores(None)?;
                let filters = FilterBuilder::new()
                    .add_filter(Filter::UniquePlayers)
                    .add_filter(Filter::Sort(SortColumn::Time))
                    .add_filter(Filter::Top(5));
                let scores = ScoreBoard::new(scores.clone())
                    .filter(filters.clone())
                    .scores();

                let _ = publish::Publisher::new()?.publish(publish::PublishType::NewScore((
                    config.challenge.name.clone(),
                    score,
                    scores,
                )));
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
                let scores = db.get_scores(None)?;
                let filters = FilterBuilder::new()
                    .add_filter(Filter::UniquePlayers)
                    .add_filter(Filter::Sort(SortColumn::Time))
                    .add_filter(Filter::Top(5));
                let scores = ScoreBoard::new(scores.clone())
                    .filter(filters.clone())
                    .scores();
                let _ = publish::Publisher::new()?.publish(publish::PublishType::CopyCard {
                    challenge: config.challenge.name.clone(),
                    thief: thief.name,
                    victim: victim.name,
                    scores,
                });
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

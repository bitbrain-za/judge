mod config;
use scoreboard_db::{Db, Score};
mod debug_config;
mod generator;
use log::debug;

const TEST_SAMPLES: usize = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    debug_config::init_debug(&args);

    let db_pass = match option_env!("DB_PASSWORD") {
        Some(pass) => pass,
        None => {
            return Err(
                "This program needs to be compiled with the $DB_PASS env variable set".into(),
            )
        }
    };

    let mut db = Db::new("localhost", 3306, "code_challenge", db_pass, "23_3_1")?;

    let config = config::RunMode::from_args(&args)?;
    debug!("Config: {:?}", config);

    match config {
        config::RunMode::Update(_config) => {
            run_sim(&mut db)?;
        }
        config::RunMode::Read(config) => {
            read_scores(config, &mut db)?;
        }
        config::RunMode::Wipe => {
            db.clear_table()?;
        }
    }
    Ok(())
}

fn read_scores(
    config: config::ReadConfig,
    db: &mut Db,
) -> Result<Vec<Score>, Box<dyn std::error::Error>> {
    let scores: Vec<Score> = db.get_scores(config.limit, Some(config.sort))?;
    println!("Scores: {:?}", scores);
    Ok(scores)
}

fn run_sim(_db: &mut Db) -> Result<(), Box<dyn std::error::Error>> {
    const PATH: &str = "test.json";
    let gen = generator::Generator::new(TEST_SAMPLES);
    gen.save_to_file(PATH)?;

    std::fs::remove_file(PATH).expect("could not remove file");
    Ok(())
}

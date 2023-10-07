mod config;
use scoreboard_db::Db;
mod debug_config;
mod generator;
use log::{debug, error, info, warn};
mod card;
mod read;
mod run;
use generator::Generator;
mod menu;

const ATTEMP_SAMPLES: usize = 100_000;
const TEST_SAMPLES: usize = 500;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    debug_config::init_debug(&args);

    info!("Firing up judge_2331 {}", env!("CARGO_PKG_VERSION"));
    let db_pass = match option_env!("DB_PASSWORD") {
        Some(pass) => pass,
        None => {
            return Err(
                "This program needs to be compiled with the $DB_PASSWORD env variable set".into(),
            )
        }
    };

    let config = match config::RunMode::from_args(&args) {
        Ok(config) => config,
        Err(_) => match menu::Menu::run(None) {
            Ok(config) => config,
            Err(e) => {
                error!("Failed to establish run mode: {}", e);
                return Ok(());
            }
        },
    };
    debug!("Config: {:?}", config);

    match config {
        config::RunMode::Update(config) => {
            debug!(
                "Connecting to database code_challenge.{}",
                config.challenge.table
            );
            let mut db = match Db::new(
                "localhost",
                3306,
                "code_challenge",
                db_pass,
                &config.challenge.table,
            ) {
                Ok(db) => db,
                Err(e) => {
                    error!("Failed to connect to database: {}", e);
                    return Ok(());
                }
            };
            println!("Welcome to the code challenge {}!", whoami::realname());
            info!("setting up to run {}", config.command);
            // TODO Move this inside the challenge itself (incl the number of cases)
            let mut generator = match config.test_mode {
                true => generator::G2331::new(TEST_SAMPLES),
                false => generator::G2331::new(ATTEMP_SAMPLES),
            };
            match run::run(&mut db, &config, &mut generator) {
                Ok(_) => {}
                Err(e) => {
                    warn!("Failed to run your program: {}", e);
                }
            }
        }
        config::RunMode::Read(config) => {
            debug!(
                "Connecting to database code_challenge.{}",
                config.challenge.table
            );
            let mut db = match Db::new(
                "localhost",
                3306,
                "code_challenge",
                db_pass,
                &config.challenge.table,
            ) {
                Ok(db) => db,
                Err(e) => {
                    error!("Failed to connect to database: {}", e);
                    return Ok(());
                }
            };
            match read::read_scores(config, &mut db) {
                Ok(reader) => {
                    if let Err(e) = reader.pretty_print() {
                        error!("Failed to print scores: {}", e);
                    }
                }
                Err(e) => {
                    warn!("Failed to read scores: {}", e);
                }
            }
        }
        config::RunMode::Wipe(table) => {
            debug!("Connecting to database code_challenge.{}", table);
            let mut db = match Db::new("localhost", 3306, "code_challenge", db_pass, &table) {
                Ok(db) => db,
                Err(e) => {
                    error!("Failed to connect to database: {}", e);
                    return Ok(());
                }
            };
            match db.clear_table() {
                Ok(_) => {}
                Err(e) => {
                    warn!("Failed to wipe scores: {}", e);
                }
            }
        }
        config::RunMode::Announce(_message) => {
            todo!()
        }
    }
    Ok(())
}

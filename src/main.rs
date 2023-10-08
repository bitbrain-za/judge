mod config;
use scoreboard_db::Db;
mod debug_config;
mod generator;
use log::{debug, error, info, warn};
mod menu;
mod read;
mod run;
mod settings;
mod teams;
use teams::publish;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    debug_config::init_debug(&args);

    info!("Firing up judge v{}", env!("CARGO_PKG_VERSION"));
    let db_pass = match option_env!("DB_PASSWORD") {
        Some(pass) => pass,
        None => {
            return Err(
                "This program needs to be compiled with the $DB_PASSWORD env variable set".into(),
            )
        }
    };

    let settings_path = match option_env!("SETTINGS_PATH") {
        Some(path) => path,
        None => {
            return Err(
                "This program needs to be compiled with the $SETTINGS_PATH env variable set".into(),
            )
        }
    };
    let settings = match settings::Settings::load(settings_path) {
        Ok(settings) => settings,
        Err(e) => {
            error!("Failed to load settings: {}", e);
            return Ok(());
        }
    };
    debug!("Settings: {:?}", settings);

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
            cliclack::intro(format!(
                "Welcome to the code challenge {}!",
                whoami::realname()
            ))?;

            if !settings.allowed_to_run(&config.challenge.command)? {
                cliclack::outro("This challenge is not yet available for submissions")?;
                return Ok(());
            }

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

            info!("setting up to run {}", config.command);
            match run::run(&mut db, &config) {
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
        config::RunMode::Announce(message) => {
            match message.as_str() {
                "release" => {
                    info!("Announcing release");

                    let release = format!("judge v{} is live\n", env!("CARGO_PKG_VERSION"));
                    let changes = include_str!("../CHANGELOG.md");

                    let _ = publish::Publisher::new()?.publish(publish::PublishType::Announcement(
                        (release, changes.to_string()),
                    ));
                }
                "launch" => {
                    info!("Announcing launch");

                    let launch = "Announcement!".to_string();
                    let changes =
                        "New challenge is live. Check in with judge for details".to_string();

                    let _ = publish::Publisher::new()?
                        .publish(publish::PublishType::Announcement((launch, changes)));
                }
                _ => {
                    error!("Unknown announcement: {}", message);
                }
            }
            todo!()
        }
    }
    Ok(())
}

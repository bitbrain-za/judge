use std::str::FromStr;

use crate::generator::challenges::{Challenge, Challenges};
use log::{debug, error};

use scoreboard_db::Builder as FilterBuilder;
use scoreboard_db::{Filter, SortColumn};

#[derive(Debug)]
pub enum RunMode {
    Update(WriteConfig),
    Read(ReadConfig),
    Wipe(String),
}

impl RunMode {
    fn print_help() {
        termimad::print_text(include_str!("../README.md"));
    }

    pub fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let user = whoami::username();

        if args.contains(&String::from("-h"))
            || args.contains(&String::from("--help"))
            || args.contains(&String::from("-?"))
            || args.is_empty()
        {
            Self::print_help();
            std::process::exit(0);
        }

        if args.contains(&String::from("--version")) {
            println!("judge_2331 {}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }

        let mode = if args.contains(&String::from("-c")) {
            RunMode::Update(WriteConfig::from_args(args)?)
        } else if args.contains(&String::from("-p")) {
            RunMode::Read(ReadConfig::from_args(args)?)
        } else if args.contains(&String::from("-C")) {
            let challenges = Challenges::new();
            let index = args.iter().position(|r| r == "-C").unwrap();
            let c = args
                .get(index + 1)
                .ok_or("-C must provide a string")?
                .to_string();
            if c == "?" {
                println!("Available challenges:");
                println!("{}", challenges);
                std::process::exit(0);
            }
            let challenge = match challenges.get_challenge(&c) {
                Some(c) => c,
                None => {
                    error!("Invalid chllenge: {}", c);
                    println!("Available challenges:");
                    println!("{}", challenges);
                    std::process::exit(1);
                }
            };
            debug!("Printing challenge: {:?}", challenge);
            challenge.print();
            std::process::exit(0);
        } else if args.contains(&String::from("-w")) {
            if "root" != user {
                error!("You need to be root to wipe the DB");
                return Err("Not root".into());
            }
            let index = args.iter().position(|r| r == "-w").unwrap();
            let table = args
                .get(index + 1)
                .ok_or("-w must provide a table name")?
                .to_string();

            RunMode::Wipe(table)
        } else {
            error!("You need to provide arguments");
            Self::print_help();
            return Err("No mode provided".into());
        };
        Ok(mode)
    }
}

#[derive(Debug)]
pub struct ReadConfig {
    pub challenge: Challenge,
    pub filters: FilterBuilder,
}

impl ReadConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let challenges = Challenges::new();
        let mut challenge = challenges.get_challenge("2331").expect("FIX ME!");
        let mut filters = FilterBuilder::new();

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-l" => {
                    let limit = args
                        .get(i + 1)
                        .ok_or("-l must provide a string")?
                        .parse::<usize>()
                        .map_err(|e| format!("-l must provide a number: {}", e))?;

                    filters.append(Filter::Top(limit));
                }
                "-C" => {
                    let c = args
                        .get(i + 1)
                        .ok_or("-C must provide a string")?
                        .to_string();
                    challenge = match challenges.get_challenge(&c) {
                        Some(c) => c,
                        None => {
                            error!("Challenge {} not found", c);
                            println!("Available challenges:");
                            println!("{}", challenges);
                            std::process::exit(1);
                        }
                    };
                }
                "--unique" => match args
                    .get(i + 1)
                    .ok_or("--unique must provide a string")?
                    .as_str()
                {
                    "players" => filters.append(Filter::UniquePlayers),
                    "names" => filters.append(Filter::UniquePlayers),
                    "name" => filters.append(Filter::UniquePlayers),
                    "player" => filters.append(Filter::UniquePlayers),
                    "language" => filters.append(Filter::UniqueLanguages),
                    "languages" => filters.append(Filter::UniqueLanguages),
                    "binary" => filters.append(Filter::UniqueBinaries),
                    "binaries" => filters.append(Filter::UniqueBinaries),
                    "bin" => filters.append(Filter::UniqueBinaries),
                    "command" => filters.append(Filter::UniqueBinaries),
                    _ => {}
                },
                "--player" => {
                    let player = args
                        .get(i + 1)
                        .ok_or("--player must provide a string")?
                        .to_string();
                    filters.append(Filter::Player(vec![player]));
                }
                "--language" => {
                    let language = args
                        .get(i + 1)
                        .ok_or("--language must provide a string")?
                        .to_string();
                    filters.append(Filter::Language(vec![language]));
                }
                "--binary" => {
                    let binary = args
                        .get(i + 1)
                        .ok_or("--binary must provide a string")?
                        .to_string();
                    filters.append(Filter::Binary(vec![binary]));
                }
                "--sort" => {
                    let sort = SortColumn::from_str(
                        args.get(i + 1)
                            .ok_or("--sort must provide a string")?
                            .as_str(),
                    )?;
                    filters.append(Filter::Sort(sort));
                }
                _ => {}
            }
        }

        let config = ReadConfig {
            challenge: challenge.clone(),
            filters,
        };

        debug!("Read Config: {:?}", config);
        Ok(config)
    }
}

#[derive(Debug)]
pub struct WriteConfig {
    pub name: String,
    pub command: String,
    pub publish: bool,
    pub test_mode: bool,
    pub challenge: Challenge,
    pub language: String,
}

impl Default for WriteConfig {
    fn default() -> Self {
        let challenges = Challenges::new();
        let challenge = challenges.get_challenge("2331").expect("FIX ME!");
        WriteConfig {
            name: whoami::username(),
            command: String::new(),
            publish: false,
            test_mode: false,
            challenge: challenge.clone(),
            language: String::new(),
        }
    }
}

impl WriteConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut name: Option<String> = Some(whoami::username());
        let mut command: Option<String> = None;
        let mut publish = true;
        let mut test_mode = false;
        let challenges = Challenges::new();
        let mut challenge = challenges.get_challenge("2331").expect("FIX ME!");
        let mut language: Option<String> = None;

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-C" => {
                    let c = args
                        .get(i + 1)
                        .ok_or("-C must provide a string")?
                        .to_string();
                    challenge = match challenges.get_challenge(&c) {
                        Some(c) => c,
                        None => {
                            error!("Challenge {} not found", c);
                            println!("Available challenges:");
                            println!("{}", challenges);
                            std::process::exit(1);
                        }
                    };
                }
                "-n" => {
                    let user = whoami::username();
                    if user != "root" {
                        return Err("-n has been deprecated. Do not use anymore.".into());
                    }
                    name = Some(
                        args.get(i + 1)
                            .ok_or("-n must provide a string")?
                            .to_string(),
                    );
                }
                "-c" => {
                    // TODO - there's a bug here where if C doesn't have an argument, it will try run the next flag
                    // Applies to all the other ones as well.
                    command = Some(
                        args.get(i + 1)
                            .ok_or("-c must provide a string")?
                            .to_string(),
                    );
                }
                "-t" => {
                    test_mode = true;
                }
                "-q" => {
                    publish = false;
                }

                "-L" => {
                    language = Some(
                        args.get(i + 1)
                            .ok_or("-L must provide a string")?
                            .to_string()
                            .to_lowercase(),
                    );
                }
                _ => {}
            }
        }

        if test_mode {
            publish = false;
        }

        let config = WriteConfig {
            name: name.ok_or("-n must be provided")?,
            command: command.ok_or("-c must be provided")?,
            publish,
            test_mode,
            challenge: challenge.clone(),
            language: language.ok_or("-L must be provided")?,
        };

        debug!("Write Config: {:?}", config);
        Ok(config)
    }
}

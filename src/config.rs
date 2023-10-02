use crate::generator::challenges::{Challenge, Challenges};
use log::{debug, error, info};

#[derive(Debug)]
pub enum RunMode {
    Update(WriteConfig),
    Read(ReadConfig),
    Wipe,
}

impl RunMode {
    fn print_help() {
        info!("Usage: judge_2331 [options]");
        info!("-h: Print this help message");
        info!("-C <challenge>: Which challenge are you competing in. '?' to print all available challenges");
        info!("-c <command>: Make an attemp with your program supplied as <command>");
        info!("-n <name>: [DEPRECATED] the name to put on the scoreboard");
        info!("-L <language>: OPTIONAL the language you are using.");
        info!("-p: Print the scoreboard");
        info!("-q: Run in stealth mode (don't publish to the channel)");
        info!("-t: Run in test mode. No results will be published to the scoreboard or channel");
        info!("-a: Print all entries in the scoreboard");
        info!("-l <limit>: Print the top <limit> entries in the scoreboard");
        info!("-v <level>: Set the log level to <level>");
        info!("-o <output>: Set the log output to <output>");
        info!("-w Wipe the scoreboard");
        info!("--version Print the version");
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
            info!("judge_2331 {}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }

        let mode = if args.contains(&String::from("-c")) {
            RunMode::Update(WriteConfig::from_args(args)?)
        } else if args.contains(&String::from("-p")) {
            RunMode::Read(ReadConfig::from_args(args)?)
        } else if args.contains(&String::from("-w")) {
            if "root" != user {
                error!("You need to be root to wipe the DB");
                return Err("Not root".into());
            }
            RunMode::Wipe
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
    pub all: bool,
    pub limit: Option<usize>,
    pub challenge: Challenge,
}

impl ReadConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut all = false;
        let mut limit = None;
        let challenges = Challenges::new();
        let mut challenge = challenges.get_challenge("2331").expect("FIX ME!");

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-a" => {
                    all = true;
                }
                "-l" => {
                    limit = args
                        .get(i + 1)
                        .ok_or("-l must provide a string")?
                        .parse::<usize>()
                        .ok();
                }
                "-C" => {
                    let c = args
                        .get(i + 1)
                        .ok_or("-C must provide a string")?
                        .to_string();
                    if c == "?" {
                        info!("Available challenges:");
                        info!("{}", challenges);
                        std::process::exit(0);
                    }
                    challenge = match challenges.get_challenge(&c) {
                        Some(c) => c,
                        None => {
                            error!("Challenge {} not found", c);
                            info!("Available challenges:");
                            info!("{}", challenges);
                            std::process::exit(1);
                        }
                    };
                }
                _ => {}
            }
        }

        let config = ReadConfig {
            all,
            limit,
            challenge: challenge.clone(),
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
    pub language: Option<String>,
}

impl WriteConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut name: Option<String> = Some(whoami::username());
        let mut command: Option<String> = None;
        let mut publish = true;
        let mut test_mode = false;
        let challenges = Challenges::new();
        let mut challenge = challenges.get_challenge("2331").expect("FIX ME!");
        let mut language = None;

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
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
                "-C" => {
                    let c = args
                        .get(i + 1)
                        .ok_or("-C must provide a string")?
                        .to_string();
                    if c == "?" {
                        info!("Available challenges:");
                        info!("{}", challenges);
                        std::process::exit(0);
                    }
                    challenge = match challenges.get_challenge(&c) {
                        Some(c) => c,
                        None => {
                            error!("Challenge {} not found", c);
                            info!("Available challenges:");
                            info!("{}", challenges);
                            std::process::exit(1);
                        }
                    };
                }
                "-L" => {
                    language = Some(
                        args.get(i + 1)
                            .ok_or("-L must provide a string")?
                            .to_string(),
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
            language,
        };

        debug!("Write Config: {:?}", config);
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config() {
        let args = vec![String::from("-a"), String::from("-l"), String::from("1")];
        let config = ReadConfig::from_args(&args).expect("Error parsing args");
        assert!(config.all);
        assert_eq!(config.limit, Some(1));
    }

    #[test]
    fn test_write_config() {
        let args = vec![String::from("-c"), String::from("command")];
        let config = WriteConfig::from_args(&args).expect("Error parsing args");
        assert_eq!(config.command, "command");
    }
}

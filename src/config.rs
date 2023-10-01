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
        info!("-c <command>: Make an attemp with your program supplied as <command>");
        info!("-p: Print the scoreboard");
        info!("-q: Run in stealth mode (don't publish to the channel)");
        info!("-a: Print all entries in the scoreboard");
        info!("-l <limit>: Print the top <limit> entries in the scoreboard");
        info!("-v <level>: Set the log level to <level>");
        info!("-o <output>: Set the log output to <output>");
        info!("-w Wipe the scorebaord");
    }

    pub fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let user = whoami::username();

        if args.contains(&String::from("-h")) {
            Self::print_help();
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
}

impl ReadConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut all = false;
        let mut limit = None;

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
                _ => {}
            }
        }

        let config = ReadConfig { all, limit };

        debug!("Read Config: {:?}", config);
        Ok(config)
    }
}

#[derive(Debug)]
pub struct WriteConfig {
    pub name: String,
    pub command: String,
    pub publish: bool,
}

impl WriteConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut name: Option<String> = Some(whoami::realname());
        let mut command: Option<String> = None;
        let mut publish = true;

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
                "-q" => {
                    publish = false;
                }
                _ => {}
            }
        }

        let config = WriteConfig {
            name: name.ok_or("-n must be provided")?,
            command: command.ok_or("-c must be provided")?,
            publish,
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

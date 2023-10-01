use log::{debug, error, info};

#[derive(Debug)]
pub enum RunMode {
    Update(WriteConfig),
    Read(ReadConfig),
    Wipe,
}

impl RunMode {
    pub fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        if args.contains(&String::from("-h")) {
            info!("Usage: judge_2331 [options]");
            info!("-h: Print this help message");
            info!("-n <name> -c <command>: Update the scoreboard with the result of running <command> as <name>");
            info!("-p: Print the scoreboard");
            info!("-q: Run in stealth mode (don't publish to the channel)");
            info!("-a: Print all entries in the scoreboard");
            info!("-l <limit>: Print the top <limit> entries in the scoreboard");
            info!("-v <level>: Set the log level to <level>");
            info!("-o <output>: Set the log output to <output>");
            std::process::exit(0);
        }

        let mode = if args.contains(&String::from("-n")) {
            RunMode::Update(WriteConfig::from_args(args)?)
        } else if args.contains(&String::from("-p")) {
            RunMode::Read(ReadConfig::from_args(args)?)
        } else if args.contains(&String::from("-w")) {
            RunMode::Wipe
        } else {
            error!("You need to provide arguments");
            info!("Usage: judge_2331 [options]");
            info!("-h: Print this help message");
            info!("-n <name> -c <command>: Update the scoreboard with the result of running <command> as <name>");
            info!("-p: Print the scoreboard");
            info!("-a: Print all entries in the scoreboard");
            info!("-l <limit>: Print the top <limit> entries in the scoreboard");
            info!("-v <level>: Set the log level to <level>");
            info!("-o <output>: Set the log output to <output>");
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
        let mut name: Option<String> = None;
        let mut command: Option<String> = None;
        let mut publish = true;

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-n" => {
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
        let args = vec![
            String::from("-n"),
            String::from("name"),
            String::from("-c"),
            String::from("command"),
        ];
        let config = WriteConfig::from_args(&args).expect("Error parsing args");
        assert_eq!(config.name, "name");
        assert_eq!(config.command, "command");
    }
}

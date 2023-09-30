use log::debug;

#[derive(Debug)]
pub enum RunMode {
    Update(WriteConfig),
    Read(ReadConfig),
    Wipe,
}

impl RunMode {
    pub fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mode = if args.contains(&String::from("-n")) {
            RunMode::Update(WriteConfig::from_args(args)?)
        } else if args.contains(&String::from("-p")) {
            RunMode::Read(ReadConfig::from_args(args)?)
        } else if args.contains(&String::from("-w")) {
            RunMode::Wipe
        } else {
            return Err("No mode provided".into());
        };
        Ok(mode)
    }
}

#[derive(Debug)]
pub struct ReadConfig {
    pub sort: String,
    pub limit: Option<usize>,
}

impl ReadConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut sort = String::from("time_ns");
        let mut limit = None;

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-s" => {
                    sort = args
                        .get(i + 1)
                        .ok_or("-s must provide a string")?
                        .to_string();
                }
                "-n" => {
                    limit = args
                        .get(i + 1)
                        .ok_or("-l must provide a string")?
                        .parse::<usize>()
                        .ok();
                }
                _ => {}
            }
        }

        let config = ReadConfig { sort, limit };

        debug!("Read Config: {:?}", config);
        Ok(config)
    }
}

#[derive(Debug)]
pub struct WriteConfig {
    pub name: String,
    pub command: String,
}

impl WriteConfig {
    fn from_args(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut name: Option<String> = None;
        let mut command: Option<String> = None;

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
                _ => {}
            }
        }

        let config = WriteConfig {
            name: name.ok_or("-n must be provided")?,
            command: command.ok_or("-c must be provided")?,
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
        let args = vec![
            String::from("-s"),
            String::from("sort"),
            String::from("-n"),
            String::from("1"),
        ];
        let config = ReadConfig::from_args(&args).expect("Error parsing args");
        assert_eq!(config.sort, "sort");
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

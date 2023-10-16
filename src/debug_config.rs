use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::{fmt, str::FromStr};
use systemd_journal_logger::JournalLog;

#[derive(PartialEq, Debug, Hash)]
enum DebugOut {
    Stdout,
    Syslog,
}

impl FromStr for DebugOut {
    type Err = ();

    fn from_str(input: &str) -> Result<DebugOut, Self::Err> {
        let input = String::from(input).to_lowercase();
        match input.as_str() {
            "stdout" => Ok(DebugOut::Stdout),
            "syslog" => Ok(DebugOut::Syslog),
            _ => Err(()),
        }
    }
}

impl fmt::Display for DebugOut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DebugOut::Stdout => String::from("stdout"),
            DebugOut::Syslog => String::from("syslog"),
        };
        write!(f, "{}", s)
    }
}

fn get_level(args: &[String]) -> LevelFilter {
    for (i, arg) in args.iter().enumerate() {
        if arg == "-v" {
            if let Ok(level) =
                LevelFilter::from_str(args.get(i + 1).expect("-v must provide a string"))
            {
                return level;
            }
        }
    }
    LevelFilter::Warn
}

fn get_output(args: &[String]) -> DebugOut {
    for (i, arg) in args.iter().enumerate() {
        if arg == "-o" {
            if let Ok(output) =
                DebugOut::from_str(args.get(i + 1).expect("-o must provide a string"))
            {
                return output;
            }
        }
    }
    DebugOut::Stdout
}

pub fn init_debug(args: &[String]) {
    let level = get_level(args);
    match get_output(args) {
        DebugOut::Stdout => {
            SimpleLogger::new()
                .with_level(level)
                .init()
                .expect("Failed to initialize logger");
        }
        DebugOut::Syslog => {
            JournalLog::new()
                .expect("Failed to initialize logger")
                .install()
                .expect("Failed to initialize logger");
            log::set_max_level(level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_level() {
        let args = vec![
            String::from("program"),
            String::from("-v"),
            String::from("debug"),
        ];
        assert_eq!(get_level(&args), LevelFilter::Debug);
    }

    #[test]
    fn test_get_output() {
        let args = vec![
            String::from("program"),
            String::from("-o"),
            String::from("stdout"),
        ];
        assert_eq!(get_output(&args), DebugOut::Stdout);
    }

    #[test]
    fn test_get_output_default() {
        let args = vec![String::from("program")];
        assert_eq!(get_output(&args), DebugOut::Stdout);
    }

    #[test]
    fn test_get_output_bad() {
        let args = vec![
            String::from("program"),
            String::from("-o"),
            String::from("bad"),
        ];
        assert_eq!(get_output(&args), DebugOut::Stdout);
    }
}

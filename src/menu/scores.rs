use crate::config::{ReadConfig, RunMode};
use crate::menu::common::challenge_selection;

pub fn run() -> Result<RunMode, Box<dyn std::error::Error>> {
    let challenge = challenge_selection()?;

    Ok(RunMode::Read(ReadConfig {
        challenge,
        ..Default::default()
    }))
}

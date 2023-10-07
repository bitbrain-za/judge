use crate::config::RunMode;
use crate::menu::common::challenge_selection;
use cliclack::{intro, outro};

pub fn run() -> Result<Option<RunMode>, Box<dyn std::error::Error>> {
    intro("Help")?;

    let mode = cliclack::select("Please select one")
        .initial_value("submit")
        .item("judge", "About the Judge", "")
        .item("challenges", "Challenge Info", "")
        .item("exit", "Exit", "")
        .interact()?;

    match mode {
        "judge" => {
            termimad::print_text(include_str!("../../README.md"));
        }
        "challenges" => {
            let challenge = challenge_selection()?;
            outro(format!("You selected {}", challenge.command))?;
            challenge.print();
        }
        _ => {}
    }

    Ok(None)
}

use crate::config::RunMode;
use crate::generator::challenges::{Challenge, Challenges};
use cliclack::{input, intro, outro};

pub fn run() -> Result<RunMode, Box<dyn std::error::Error>> {
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
            challenge_selection()?;
        }
        _ => {}
    }

    Ok(RunMode::Other)
}

fn challenge_selection() -> Result<RunMode, Box<dyn std::error::Error>> {
    print!("\x1B[2J\x1B[1;1H");
    let mut choice = cliclack::select("Please select one").initial_value("submit");

    let mut challenges = Challenges::new();

    for challenge in challenges.challenges.iter_mut() {
        choice = choice.item(&challenge.command, &challenge.name, "");
    }
    let command = choice.interact()?;

    let challenges = Challenges::new();
    let challenge = match challenges.get_challenge(command) {
        Some(c) => c,
        None => {
            println!("Invalid challenge");
            return Ok(RunMode::Other);
        }
    };
    challenge.print();

    Ok(RunMode::Other)
}

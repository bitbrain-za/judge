use crate::generator::challenges::{Challenge, Challenges};

pub fn challenge_selection() -> Result<Challenge, Box<dyn std::error::Error>> {
    let mut choice = cliclack::select("Please select a challenge").initial_value("submit");

    let mut challenges = Challenges::new();

    for challenge in challenges.challenges.iter_mut() {
        choice = choice.item(&challenge.command, &challenge.name, "");
    }
    let command = choice.interact()?;

    let challenges = Challenges::new();
    match challenges.get_challenge(command) {
        Some(c) => Ok(c.clone()),
        None => {
            println!("Invalid challenge");
            Err("Invalid challenge".into())
        }
    }
}

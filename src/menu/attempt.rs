use crate::config::{RunMode, WriteConfig};
use crate::menu::common::challenge_selection;
use cliclack::{input, intro, outro, select};
use log::debug;

pub fn run() -> Result<Option<RunMode>, Box<dyn std::error::Error>> {
    intro("Help")?;

    let challenge = challenge_selection()?;

    let command: String = input("How do I run your program?")
        .placeholder("./echo \"Hello, World!\"")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()?;

    let language: String = input("What language did you write this in?")
        .placeholder("Malbolge")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()?;

    let test_mode: bool = select("Are you just testing?")
        .item(false, "No!", "Testing is boring")
        .item(true, "Yes", "We'll do the real thing later")
        .interact()?;

    let publish: bool = select("Would you like to publish this?")
        .item(false, "Yes!", "Good for you")
        .item(true, "No", "That's a bit dissapointing")
        .interact()?;

    outro("All set, let's go!")?;

    let config = WriteConfig {
        command,
        language,
        test_mode,
        publish,
        challenge: challenge.clone(),
        ..Default::default()
    };

    debug!("Config: {:?}", config);

    Ok(Some(RunMode::Update(config)))
}

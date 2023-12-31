use crate::config::{RunMode, WriteConfig};
use crate::menu::common::challenge_selection;
use cliclack::{input, intro, outro, select};
use log::debug;

pub fn run(allow_quiet_mode: Option<bool>) -> Result<RunMode, Box<dyn std::error::Error>> {
    intro("Help")?;
    let allow_quiet_mode = allow_quiet_mode.unwrap_or(true);

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
    let language = language.to_lowercase();

    let test_mode: bool = select("Are you just testing?")
        .item(false, "No!", "Testing is boring")
        .item(true, "Yes", "We'll do the real thing later")
        .interact()?;

    let publish: bool = if allow_quiet_mode {
        if !test_mode {
            select("Would you like to publish this?")
                .item(true, "Yes!", "Good for you")
                .item(false, "No", "That's a bit dissapointing")
                .interact()?
        } else {
            false
        }
    } else {
        true
    };

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

    Ok(RunMode::Update(config))
}

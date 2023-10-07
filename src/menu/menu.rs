use cliclack::{input, intro, outro};

use crate::config::RunMode;
use crate::menu::docs::run as docs;

pub struct Menu {}

impl Menu {
    pub fn run() -> Result<RunMode, Box<dyn std::error::Error>> {
        intro("The Judge!")?;

        let _run_mode = cliclack::select("Please select one")
            .initial_value("submit")
            .item("run", "Submit a run", "")
            .item("read", "Check the Scoreboard", "")
            .item("docs", "View the docs", "")
            .interact()?;

        let mode = match _run_mode {
            "run" => todo!(),
            "read" => todo!(),
            "docs" => docs()?,
            _ => return Err("Invalid selection".into()),
        };

        Ok(mode)
    }
}

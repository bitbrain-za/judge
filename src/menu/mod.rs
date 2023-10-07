mod attempt;
mod common;
mod docs;
mod menu;

pub use menu::Menu;

use crate::config::RunMode;

pub trait JudgeMenu {
    fn run() -> Result<RunMode, Box<dyn std::error::Error>>;
}

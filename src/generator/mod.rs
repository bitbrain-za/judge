mod g2331;
pub use g2331::G2331;
mod g2332;
pub use g2332::G2332;
mod g2333;
pub use g2333::G2333;
mod g2334;
pub use g2334::G2334;
mod g2411;
pub use g2411::G2411;

pub mod challenges;
mod test_result;

pub use self::test_result::TestResult;
pub trait Generator {
    fn save_to_file(&self, filename: &str) -> std::io::Result<()>;
    fn check_answer(&self, data: &str) -> Result<bool, Box<dyn std::error::Error>>;
    fn get_test_cases(&self) -> String;
    fn regenerate(&mut self);
    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn run(&self, test: TestResult) -> Result<TestResult, Box<dyn std::error::Error>>;
}

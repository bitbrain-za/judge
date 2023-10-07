mod g2331;
pub use g2331::G2331;
mod g2332;
pub use g2332::G2332;

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

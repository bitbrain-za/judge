mod g2331;
pub use g2331::G2331;
pub mod challenges;

pub trait Generator {
    fn new(count: usize) -> Self;
    fn save_to_file(&self, filename: &str) -> std::io::Result<()>;
    fn check_answer(&self, data: &str) -> Result<bool, Box<dyn std::error::Error>>;
    fn get_test_cases(&self) -> String;
    fn regenerate(&mut self);
}

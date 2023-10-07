use std::fmt::Display;

use crate::generator::Generator;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::process::Command;
use std::time::Instant;

use super::TestResult;

pub struct G2331 {
    pub count: usize,
    pub test_cases: Vec<Vec<i32>>,
    pub answer: Vec<i32>,
}

impl Display for G2331 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(&self.test_cases).unwrap();
        write!(f, "{}", json)
    }
}

const PATH: &str = "test.json";

impl G2331 {
    pub fn new(count: usize) -> Self {
        let mut s = Self {
            count,
            test_cases: Vec::new(),
            answer: Vec::new(),
        };
        s.regenerate();
        s
    }
}

impl Generator for G2331 {
    fn get_test_cases(&self) -> String {
        serde_json::to_string(&self.test_cases).unwrap()
    }

    fn regenerate(&mut self) {
        let mut rng = thread_rng();
        self.test_cases = Vec::new();

        while self.test_cases.len() < self.count {
            let mut test_case: Vec<i32> = Vec::new();
            let random_size: u8 = rng.gen_range(1..100);
            let mut random = 0;
            for _ in 0..random_size {
                random = rng.gen_range(1..100);
                test_case.push(random);
                test_case.push(random);
            }
            test_case.push(random);
            self.answer.push(random);
            test_case.shuffle(&mut thread_rng());

            self.test_cases.push(test_case);
        }
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::create(filename)?;
        file.write_all(self.to_string().as_bytes())?;
        Ok(())
    }

    fn check_answer(&self, data: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let result: Vec<i32> = serde_json::from_str(data).expect("JSON was not well-formatted");
        Ok(self.answer == result)
    }

    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.save_to_file(PATH)?;
        Ok(())
    }

    fn run(&self, test: TestResult) -> Result<TestResult, Box<dyn std::error::Error>> {
        if let TestResult::Success(score) = test {
            let mut score = score;
            // run the test
            let ex = format!("{} {}", score.command, PATH);
            let mut uut = Command::new("sh");
            uut.arg("-c").arg(ex);

            let start = Instant::now();
            let output = uut
                .output()
                .map_err(|e| format!("Failed to run your program: {}", e))?;

            let elapsed = start.elapsed().as_nanos();
            let result = if elapsed > u64::MAX as u128 {
                TestResult::Fail(String::from("Your program is way too slow"))
            } else {
                score.time_ns = elapsed as f64;
                match output.status.success() {
                    false => TestResult::Fail(format!(
                        "Your program exited with a non-zero status code: {}",
                        String::from_utf8(output.stderr)?
                    )),
                    true => {
                        let out = String::from_utf8(output.stdout)?;
                        match self.check_answer(&out)? {
                            false => TestResult::Fail(String::from(
                                "Your program did not produce the correct result",
                            )),
                            true => TestResult::Success(score),
                        }
                    }
                }
            };
            Ok(result)
        } else {
            Err("Please start with a success variant".into())
        }
    }
}

impl Drop for G2331 {
    fn drop(&mut self) {
        std::fs::remove_file(PATH).expect("could not remove file");
    }
}

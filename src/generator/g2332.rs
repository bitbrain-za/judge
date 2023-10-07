use std::fmt::Display;

use crate::generator::Generator;
use log::debug;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::process::Command;
use std::time::Instant;

use super::TestResult;

const MIN_SIZE: usize = 500;
const MAX_SIZE: usize = 1000;
const TEST_SAMPLES: usize = 100;
const ATTEMPT_SAMPLES: usize = 10_000;

pub struct G2332 {
    pub count: usize,
    pub test_cases: Vec<Vec<u8>>,
    pub answer: Vec<u8>,
}

impl Display for G2332 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(&self.test_cases).unwrap();
        write!(f, "{}", json)
    }
}

impl G2332 {
    pub fn new(test: bool) -> Self {
        let count = match test {
            true => TEST_SAMPLES,
            false => ATTEMPT_SAMPLES,
        };
        let mut s = Self {
            count,
            test_cases: Vec::new(),
            answer: Vec::new(),
        };
        s.regenerate();
        s
    }

    fn print_test_case(test_case: &Vec<u8>) -> String {
        let mut s = String::new();
        for (i, n) in test_case.iter().enumerate() {
            if i == test_case.len() - 1 {
                s.push_str(&format!("{}", n));
            } else {
                s.push_str(&format!("{},", n));
            }
        }
        s
    }
}

impl Generator for G2332 {
    fn get_test_cases(&self) -> String {
        serde_json::to_string(&self.test_cases).unwrap()
    }

    fn regenerate(&mut self) {
        let mut rng = thread_rng();
        self.test_cases = Vec::new();

        while self.test_cases.len() < self.count {
            let mut test_case: Vec<u8> = Vec::new();
            let random_size: usize = rng.gen_range(MIN_SIZE..MAX_SIZE);
            let mut random = 0;
            for _ in 0..random_size {
                random = rng.gen_range(0..255);
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
        let result: Vec<u8> = serde_json::from_str(data).expect("JSON was not well-formatted");
        Ok(self.answer == result)
    }

    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn run(&self, test: TestResult) -> Result<TestResult, Box<dyn std::error::Error>> {
        if let TestResult::Success(score) = test {
            let mut score = score;
            let mut spinner = cliclack::spinner();
            let mut commands: Vec<Command> = Vec::new();

            spinner.start("Creating commands");

            /* prepare all the commands in advance */
            for test_case in &self.test_cases {
                let ex = format!("{} {}", score.command, Self::print_test_case(test_case));
                let mut uut = Command::new("sh");
                uut.arg("-c").arg(ex);

                commands.push(uut);
            }
            let ex = format!("{} 0,1,1", score.command);
            let mut base = Command::new("sh");
            base.arg("-c").arg(ex);

            /* Run the test */
            spinner.stop("All set...");
            let mut spinner = cliclack::spinner();

            let mut answers: Vec<String> = Vec::new();
            let mut baseline: u128 = 0;
            let mut elapsed: u128 = 0;

            for uut in commands.iter_mut() {
                spinner.start(format!(
                    "Running tests {} of {}",
                    answers.len(),
                    self.test_cases.len()
                ));

                let start = Instant::now();
                let output = base
                    .output()
                    .map_err(|e| format!("Failed to run your program: {}", e))?;
                baseline += start.elapsed().as_nanos();
                if !output.status.success() {
                    return Ok(TestResult::Fail(format!(
                        "Your program exited with a non-zero status code: {}",
                        String::from_utf8(output.stderr)?
                    )));
                }
                let out = String::from_utf8(output.stdout)?;
                if "0" != out.trim() {
                    return Ok(TestResult::Fail(format!(
                        "Your program did not produce the correct result: {}",
                        out
                    )));
                }

                let start = Instant::now();
                let output = uut
                    .output()
                    .map_err(|e| format!("Failed to run your program: {}", e))?;
                elapsed += start.elapsed().as_nanos();
                if !output.status.success() {
                    return Ok(TestResult::Fail(format!(
                        "Your program exited with a non-zero status code: {}",
                        String::from_utf8(output.stderr)?
                    )));
                }
                let out = String::from_utf8(output.stdout)?;
                answers.push(out);
            }
            spinner.stop("Tests Complete");
            /* End of test run */
            debug!(
                "Elapsed: {}, Baseline: {}",
                scoreboard_db::NiceTime::new(elapsed as f64),
                scoreboard_db::NiceTime::new(baseline as f64)
            );
            if baseline > elapsed {
                return Err("Unable to get a good baseline, please try again".into());
            }
            elapsed -= baseline;
            debug!(
                "Elapsed: {} after baseline",
                scoreboard_db::NiceTime::new(elapsed as f64)
            );

            let result = if elapsed > u64::MAX as u128 {
                TestResult::Fail(String::from("Your program is way too slow"))
            } else {
                score.time_ns = elapsed as f64;

                let mut answers_json = String::from("[");
                for (i, answer) in answers.iter().enumerate() {
                    answers_json.push_str(answer);
                    if i != answers.len() - 1 {
                        answers_json.push_str(", ");
                    }
                }
                answers_json.push(']');

                match self.check_answer(&answers_json)? {
                    false => TestResult::Fail(String::from(
                        "Your program did not produce the correct result",
                    )),
                    true => TestResult::Success(score),
                }
            };
            Ok(result)
        } else {
            Err("Please start with a success variant".into())
        }
    }
}

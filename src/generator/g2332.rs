use std::fmt::Display;

use crate::generator::Generator;
use log::error;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

use super::TestResult;

const TEST_SIZE: usize = 1_000;
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
                s.push_str(&format!("{}\n", n));
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
            let mut random = 0;
            for _ in 0..TEST_SIZE {
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
            spinner.start("Preparing tests");

            let mut answers: Vec<String> = Vec::new();

            /* prep the test cases */
            let mut tests: Vec<Vec<u8>> = Vec::new();
            for test in &self.test_cases {
                let case = Self::print_test_case(test).clone().as_bytes().to_vec();
                tests.push(case);
            }

            /* start the child process */
            let mut child = Command::new(&score.command)
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn()
                .unwrap();

            let mut stdin = child.stdin.take().unwrap();
            let mut stdout = child.stdout.take().unwrap();

            spinner.stop("Running tests...");
            /* Run the test */
            let start = Instant::now();
            for test in tests {
                let _ = stdin.write_all(&test);
                let mut buf = [0u8; 5];
                let rx_count = stdout.read(&mut buf).unwrap();

                if 2 > rx_count {
                    return Ok(TestResult::Fail(String::from(
                        "Your program sent back too few characters. Did you forget the newline?",
                    )));
                }

                let buf = String::from_utf8(buf.to_vec())?;
                let buf = buf.trim_matches(char::from(0)).trim();
                match buf.parse::<u8>() {
                    Ok(n) => {
                        answers.push(n.to_string());
                    }
                    Err(e) => {
                        error!("Error parsing output: {}", buf);
                        return Ok(TestResult::Fail(format!(
                            "Your program did not produce the correct result: {e}"
                        )));
                    }
                }
            }
            let elapsed = start.elapsed().as_nanos();
            let _ = stdin.write_all("q\n".as_bytes());
            spinner.stop("Tests Complete");
            /* End of test run */

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

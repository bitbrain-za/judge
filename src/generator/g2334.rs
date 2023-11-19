use std::fmt::Display;

use crate::generator::Generator;
use log::debug;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

use super::TestResult;

const TEST_SIZE: usize = 1_000;
const TEST_SAMPLES: usize = 10;
const ATTEMPT_SAMPLES: usize = 100_000;

pub struct G2334 {
    pub count: usize,
    pub test_cases: Vec<String>,
    pub answer: String,
}

impl Display for G2334 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(&self.test_cases).unwrap();
        write!(f, "{}", json)
    }
}

impl G2334 {
    pub fn new(test: bool) -> Self {
        let count = match test {
            true => TEST_SAMPLES,
            false => ATTEMPT_SAMPLES,
        };
        let mut s = Self {
            count,
            test_cases: Vec::new(),
            answer: String::new(),
        };
        s.regenerate();
        s
    }
}

impl Generator for G2334 {
    fn get_test_cases(&self) -> String {
        serde_json::to_string(&self.test_cases).unwrap()
    }

    fn regenerate(&mut self) {
        let mut rng = thread_rng();
        self.test_cases = Vec::new();
        self.answer = String::new();
        let bad_chars = [
            '"', '+', '?', '|', '*', ' ', '(', ')', '[', ']', '{', '}', '\\', '/',
        ];

        while self.test_cases.len() < self.count {
            let mut test_case: String = std::iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(TEST_SIZE)
                .collect();
            let answer: char = bad_chars[rng.gen_range(0..bad_chars.len())];

            if self.count == TEST_SAMPLES {
                let position = rng.gen_range(0..TEST_SIZE);
                test_case.insert(position, answer);
            } else {
                /* will anyone look at the code? Or analyze their output? */
                test_case.push(answer);
            }

            self.answer.push(answer);
            self.test_cases.push(test_case);
        }
        debug!("answer: {:?}", self.answer);
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::create(filename)?;
        file.write_all(self.to_string().as_bytes())?;
        Ok(())
    }

    fn check_answer(&self, data: &str) -> Result<bool, Box<dyn std::error::Error>> {
        if self.answer == data {
            Ok(true)
        } else {
            debug!("expected: {:?}", self.answer);
            debug!("received: {:?}", data);
            Ok(false)
        }
    }

    fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn run(&self, test: TestResult) -> Result<TestResult, Box<dyn std::error::Error>> {
        if let TestResult::Success(score) = test {
            let mut score = score;
            let mut spinner = cliclack::spinner();
            spinner.start("Preparing tests");

            let mut answers: String = String::new();

            /* prep the test cases */
            let mut tests: Vec<Vec<u8>> = Vec::new();
            for test in &self.test_cases {
                let case = test.as_bytes().to_vec();
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
                answers.push(buf[0] as char);
            }
            let elapsed = start.elapsed().as_nanos();
            let _ = stdin.write_all("q\n".as_bytes());
            spinner.stop("Tests Complete");
            /* End of test run */

            let result = if elapsed > u64::MAX as u128 {
                TestResult::Fail(String::from("Your program is way too slow"))
            } else {
                score.time_ns = elapsed as f64;

                match self.check_answer(&answers)? {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g2334() {
        let mut g = G2334::new(true);
        let possible_answers = [
            '"', '+', '?', '|', '*', ' ', '(', ')', '[', ']', '{', '}', '\\', '/',
        ];

        assert_eq!(g.test_cases.len(), TEST_SAMPLES);
        assert_eq!(g.answer.len(), TEST_SAMPLES);

        g.regenerate();

        let mut answers = String::new();
        for test in g.test_cases {
            assert_eq!(test.len(), TEST_SIZE + 1);

            let answer = test.chars().find(|c| possible_answers.contains(c));
            assert!(answer.is_some());
            answers.push(answer.unwrap());
        }
        assert_eq!(answers, g.answer);
    }
}

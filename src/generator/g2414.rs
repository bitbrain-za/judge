use std::fmt::Display;

use crate::generator::Generator;
use cipher_crypt::{Caesar, Cipher};
use log::{debug, error};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

use super::TestResult;

const TEST_SIZE: usize = 1_000;
const TEST_SAMPLES: usize = 10;
const ATTEMPT_SAMPLES: usize = 100_000;

type TestCase = (String, usize);
type Answer = String;

pub struct G2414 {
    pub count: usize,
    pub test_cases: Vec<TestCase>,
    pub answer: Vec<Answer>,
}

impl Display for G2414 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(&self.test_cases).unwrap();
        write!(f, "{}", json)
    }
}

impl G2414 {
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

    fn print_test_case(test_case: &TestCase) -> String {
        let (s, n) = test_case;
        format!("{}\n{}\n", s, n)
    }
}

impl Generator for G2414 {
    fn get_test_cases(&self) -> String {
        serde_json::to_string(&self.test_cases).unwrap()
    }

    fn regenerate(&mut self) {
        let mut rng = thread_rng();
        self.test_cases = Vec::new();
        self.answer = Vec::new();

        while self.test_cases.len() < self.count {
            let test_case: String = std::iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(TEST_SIZE)
                .collect();
            let shift = rng.gen_range(0..255);
            let test_case = (test_case, shift);
            let cipher = Caesar::new(test_case.1 % 26);
            let cipher_text = cipher.encrypt(&test_case.0).unwrap();

            self.answer.push(cipher_text);
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
        let result: Vec<Answer> = serde_json::from_str(data).expect("JSON was not well-formatted");
        if self.answer == result {
            Ok(true)
        } else {
            debug!("expected: {:?}", self.answer);
            debug!("received: {:?}", result);
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
                match buf.parse::<u32>() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g2414() {
        let c1 = ("Super secret message", 10);
        let a1 = "Cezob combod wocckqo";
        let s2 = ("Can you read this", 1);
        let a2 = "Dbo zpv sfbe uijt";
        let s3 = ("NO", 5);
        let a3 = "ST";
        let s4 = ("Edge case", 40);
        let a4 = "Srus qogs";

        let cipher = Caesar::new(c1.1);
        let cipher_text = cipher.encrypt(c1.0).unwrap();
        assert_eq!(cipher_text, a1);

        let cipher = Caesar::new(s2.1);
        let cipher_text = cipher.encrypt(s2.0).unwrap();
        assert_eq!(cipher_text, a2);

        let cipher = Caesar::new(s3.1);
        let cipher_text = cipher.encrypt(s3.0).unwrap();
        assert_eq!(cipher_text, a3);

        let cipher = Caesar::new(s4.1 % 26);
        let cipher_text = cipher.encrypt(s4.0).unwrap();
        assert_eq!(cipher_text, a4);
    }
}

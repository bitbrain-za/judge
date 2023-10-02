use std::fmt::Display;

use crate::generator::Generator;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

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

impl Generator for G2331 {
    fn new(count: usize) -> Self {
        let mut s = Self {
            count,
            test_cases: Vec::new(),
            answer: Vec::new(),
        };
        s.regenerate();
        s
    }

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
}

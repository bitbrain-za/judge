use super::{Generator, G2331, G2332};
use log::debug;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Challenge {
    pub name: String,
    pub command: String,
    pub doc_path: String,
    pub table: String,
}

impl Challenge {
    pub fn print(&self) {
        termimad::print_text(include_str!("2331.md"));
    }
}

pub struct Challenges {
    pub challenges: Vec<Challenge>,
}

impl Challenges {
    pub fn new() -> Self {
        let c2331 = Challenge {
            name: String::from("Find the odd one out"),
            command: String::from("2331"),
            doc_path: String::from(
                "/home/philip/code_challenges/judge_23_3_1/src/generator/2331.md",
            ),
            table: String::from("23_3_1"),
        };

        let c2332 = Challenge {
            name: String::from("Find the odd one out two"),
            command: String::from("2332"),
            doc_path: String::from(
                "/home/philip/code_challenges/judge_23_3_1/src/generator/2332.md",
            ),
            table: String::from("23_3_2"),
        };

        Self {
            challenges: vec![c2331, c2332],
        }
    }

    pub fn get_challenge(&self, challenge: &str) -> Option<&Challenge> {
        self.challenges.iter().find(|c| c.command == challenge)
    }

    pub fn make_generator(&self, challenge: &str, count: usize) -> Option<Box<dyn Generator>> {
        debug!("make_generator: {}", challenge);
        match challenge {
            "2331" => Some(Box::new(G2331::new(count))),
            "2332" => Some(Box::new(G2332::new(count))),
            _ => None,
        }
    }
}

impl Display for Challenges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for challenge in &self.challenges {
            s.push_str(&format!("{} - {}\n", challenge.command, challenge.name));
        }

        write!(f, "{}", s)
    }
}

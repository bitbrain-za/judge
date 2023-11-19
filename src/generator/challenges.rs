use super::{Generator, G2331, G2332, G2333, G2334};
use log::debug;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Challenge {
    pub name: String,
    pub command: String,
    pub table: String,
    doc: String,
}

impl Challenge {
    pub fn print(&self) {
        termimad::print_text(&self.doc);
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
            table: String::from("23_3_1"),
            doc: include_str!("2331.md").to_string(),
        };

        let c2332 = Challenge {
            name: String::from("Find the odd one out two"),
            command: String::from("2332"),
            table: String::from("23_3_2"),
            doc: include_str!("2332.md").to_string(),
        };

        let c2333 = Challenge {
            name: String::from("How big?"),
            command: String::from("2333"),
            table: String::from("23_3_3"),
            doc: include_str!("2333.md").to_string(),
        };

        let c2334 = Challenge {
            name: String::from("Input validation"),
            command: String::from("2334"),
            table: String::from("23_3_4"),
            doc: include_str!("2334.md").to_string(),
        };

        Self {
            challenges: vec![c2331, c2332, c2333, c2334],
        }
    }

    pub fn get_challenge(&self, challenge: &str) -> Option<&Challenge> {
        self.challenges.iter().find(|c| c.command == challenge)
    }

    pub fn make_generator(&self, challenge: &str, test: bool) -> Option<Box<dyn Generator>> {
        debug!("make_generator: {}", challenge);
        match challenge {
            "2331" => Some(Box::new(G2331::new(test))),
            "2332" => Some(Box::new(G2332::new(test))),
            "2333" => Some(Box::new(G2333::new(test))),
            "2334" => Some(Box::new(G2334::new(test))),
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

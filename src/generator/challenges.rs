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
            doc_path: String::from("/home/philip/code_challenges/judge_23_3_1/README.md"),
            table: String::from("23_3_1"),
        };

        Self {
            challenges: vec![c2331],
        }
    }

    pub fn get_challenge(&self, challenge: &str) -> Option<&Challenge> {
        self.challenges.iter().find(|c| c.command == challenge)
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

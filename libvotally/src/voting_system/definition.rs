use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Describe the ballot form
pub enum BallotForm {
    Uninominal,
}

/// Type for a ballot box
pub enum Ballots {
    Uninominal(HashMap<String, i32>),
}

impl Ballots {
    pub(crate) fn new(ballot_form: BallotForm, choices: impl Iterator<Item = String>) -> Self {
        match ballot_form {
            BallotForm::Uninominal => {
                let mut choices_hashmap: HashMap<String, i32> = HashMap::new();

                choices.map(|s| String::from(s)).for_each(|c| {
                    choices_hashmap.insert(c, 0);
                });
                Ballots::Uninominal(choices_hashmap)
            }
        }
    }

    pub(crate) fn choices(&self) -> impl Iterator<Item = &String> {
        match &self {
            Ballots::Uninominal(c) => c.keys(),
        }
    }

    pub(crate) fn ballot_form(&self) -> BallotForm {
        match self {
            Ballots::Uninominal(_) => BallotForm::Uninominal,
        }
    }
}

/// Contain all the information needed to an election
pub struct VotingSystemInfo {
    /// The name of the voting system
    name: String,
    /// All ballots of the voting system
    choices: Vec<String>,
}

impl VotingSystemInfo {
    pub(crate) fn new<'a, I>(name: &str, choices: I) -> Self
    where
        I: Iterator<Item = &'a String>,
    {
        let choices: Vec<String> = choices.map(|s| s.clone()).collect();

        Self {
            name: name.to_owned(),
            choices,
        }
    }
}

/// Type for algorithm finding the result of the election from all ballots
pub type ResultAlgorithm = Box<dyn Fn(&Ballots) -> Option<String>>;

// Used to describe election
// pub struct VotingSystem {
//     /// The name of the voting system
//     name: String,
//     // The ballot form of the voting system
//     // ballot_form: BallotForm,
//     /// All ballots of the voting system
//     ballot_box: Ballots,
//     /// Calculate the election's result
//     result_algorithm: ResultAlgorithm,
//     /// Total number of ballots
//     count: usize,
// }

pub trait VotingSystem {
    /// Create a new election
    fn new(
        choices: impl Iterator<Item = String>,
    ) -> Self;

    /// Get the name of the voting system
    fn get_name(&self) -> &str;

    /// Get the ballot box
    fn get_ballot_box(&mut self) -> &mut Ballots;

    /// Get all choices
    fn get_choices(&self) -> impl Iterator<Item = &String>;

    /// Get the ballot form
    fn get_ballot_form(&self) -> BallotForm;

    /// Increase number of ballot
    fn add_ballot(&mut self);

    /// Get the total number of ballots
    fn get_count(&self) -> usize;

    /// Get all the information about this election
    fn get_info(&self) -> VotingSystemInfo {
        VotingSystemInfo::new(self.get_name(), self.get_choices())
    }

    /// Just vote
    fn vote(&mut self, ballot: &str) -> Result<(), InvalidBallot> {
        match &mut self.get_ballot_box() {
            Ballots::Uninominal(c) => {
                c.get(ballot)
                    .ok_or(InvalidBallot(format!("unknown candidate {}", ballot)))?;
                c.entry(String::from(ballot))
                    .and_modify(|count| *count += 1);
            }
        }

        self.add_ballot();
        Ok(())
    }

    /// Calculate the election's result
    fn result(&self) -> Option<String>;
}

/// Error  for invalid ballot
#[derive(Debug)]
pub struct InvalidBallot(String);

impl fmt::Display for InvalidBallot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid ballot: {}", self.0)
    }
}

impl Error for InvalidBallot {}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     #[should_panic]
//     fn invalid_ballot() {
//         let mut p = VotingSystem::new(
//             "test",
//             BallotForm::Uninominal,
//             vec![String::from("A")].into_iter(),
//             Box::new(|_choices: &Ballots| Some(String::from("A"))),
//         );
//
//         p.vote("C").unwrap();
//     }
// }

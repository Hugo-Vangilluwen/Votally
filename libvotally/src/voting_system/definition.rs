use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

/// Describe the ballot's form
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum BallotForm {
    Uninominal,
    Approved,
}

impl fmt::Display for BallotForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BallotForm::Uninominal => "Uninominal",
                BallotForm::Approved => "Approved",
            }
        )
    }
}

/// Type for a signle ballot
#[derive(Serialize, Deserialize)]
pub enum SingleBallot {
    Uninominal(String),
    Approved(Vec<String>),
}

/// Type for a ballot box
pub enum Ballots {
    Rates(HashMap<String, i32>),
}

impl Ballots {
    fn new(ballot_form: BallotForm, choices: Vec<String>) -> Self {
        match ballot_form {
            BallotForm::Uninominal | BallotForm::Approved => {
                let mut choices_hashmap: HashMap<String, i32> = HashMap::new();

                choices.iter().map(|s| String::from(s)).for_each(|c| {
                    choices_hashmap.insert(c, 0);
                });
                Ballots::Rates(choices_hashmap)
            }
        }
    }

    fn choices(&self) -> impl Iterator<Item = &String> {
        match &self {
            Ballots::Rates(c) => c.keys(),
        }
    }
}

/// Describe minimal information need to an election
#[derive(Clone, Serialize, Deserialize)]
pub struct MinimalVotingSystemInfo {
    /// The name of the voting system
    name: String,
    /// Differents choices
    choices: Vec<String>,
    /// The ballots' form
    ballot_form: BallotForm,
}

impl MinimalVotingSystemInfo {
    pub fn new(name: &str, ballot_form: BallotForm, choices: Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            choices,
            ballot_form,
        }
    }

    pub fn get_choices(&self) -> Vec<String> {
        self.choices.clone()
    }

    pub fn get_ballot_form(&self) -> BallotForm {
        self.ballot_form
    }

    pub fn correct_ballot(&self, ballot: &SingleBallot) -> bool {
        match (self.ballot_form, ballot) {
            (BallotForm::Uninominal, SingleBallot::Uninominal(b)) => self.choices.contains(&b),
            (BallotForm::Approved, SingleBallot::Approved(vec_b)) => {
                vec_b.iter().all(|b| self.choices.contains(&b))
                // TODO: Check all choices are different
                && vec_b.iter().collect::<HashSet<_>>().len() == vec_b.len()
            }
            _ => false,
        }
    }
}

impl fmt::Display for MinimalVotingSystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Vote {}", self.name)?;

        let mut choices_iter = self.choices.iter();

        write!(f, "Different choices are {}", choices_iter.next().unwrap())?;
        for c in choices_iter {
            write!(f, ", {}", c)?;
        }
        writeln!(f)?;

        write!(f, "Type of ballots: {}", self.ballot_form)
    }
}

/// Contain all the information needed to an election
pub struct VotingSystemInfo {
    /// The name of the voting system
    name: String,
    /// The ballots' form
    ballot_form: BallotForm,
    /// All ballots of the voting system
    ballot_box: Ballots,
    /// Total number of ballots
    count: usize,
}

impl VotingSystemInfo {
    pub(crate) fn new(name: &str, ballot_form: BallotForm, choices: Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            ballot_form,
            ballot_box: Ballots::new(ballot_form, choices),
            count: 0,
        }
    }

    /// Get the name of the voting system
    pub fn get_name(&self) -> &str {
        &self.name[..]
    }

    /// Get all choices
    pub fn get_choices(&self) -> impl Iterator<Item = &String> {
        self.ballot_box.choices()
    }

    /// Get the ballot form
    pub fn get_ballot_form(&self) -> BallotForm {
        self.ballot_form
    }

    /// Get the ballot box
    pub fn get_ballot_box(&self) -> &Ballots {
        &self.ballot_box
    }

    /// Get the total number of ballots
    pub fn get_count(&self) -> usize {
        self.count
    }

    /// Just vote
    pub fn vote(&mut self, ballot: SingleBallot) -> Result<(), InvalidBallot> {
        match (&mut self.ballot_box, ballot) {
            (Ballots::Rates(c), SingleBallot::Uninominal(b)) => {
                c.get(&b)
                    .ok_or(InvalidBallot(format!("unknown candidate {}", b)))?;
                c.entry(b).and_modify(|count| *count += 1);
            }
            (Ballots::Rates(c), SingleBallot::Approved(vec_approved)) => {
                for b in vec_approved {
                    c.get(&b)
                        .ok_or(InvalidBallot(format!("unknown candidate {}", b)))?;
                    c.entry(b).and_modify(|count| *count += 1);
                }
            } // _ => Err(InvalidBallot("Incompatible ballot form".to_string()))?
        }

        self.count += 1;
        Ok(())
    }
}

pub trait VotingSystem {
    /// Create a new election
    // fn new(choices: Vec<String>) -> Self;

    /// Algorithm finding the result of the election from all ballots
    // fn result_algorithm(ballots: &Ballots) -> String;
    fn result(&self) -> String;

    /// Get all information about this election
    fn get_info(&self) -> &VotingSystemInfo;

    /// Get all mutable information about this election
    fn get_mut_info(&mut self) -> &mut VotingSystemInfo;

    fn vote(&mut self, ballot: SingleBallot) -> Result<(), InvalidBallot> {
        self.get_mut_info().vote(ballot)
    }

    // Calculate the election's result
    // fn result(&mut self) -> String {
    //     Self::result_algorithm(&self.get_info().ballot_box)
    // }
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

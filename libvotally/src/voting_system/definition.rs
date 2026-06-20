use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

/// Describe the ballot's form
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BallotForm {
    Uninominal,
    Approved,
    Ranked,
}

impl fmt::Display for BallotForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BallotForm::Uninominal => "Uninominal",
                BallotForm::Approved => "Approved",
                BallotForm::Ranked => "Ranked",
            }
        )
    }
}

/// Type for a signle ballot
#[derive(Serialize, Deserialize, Debug)]
pub enum SingleBallot {
    Uninominal(String),
    Approved(Vec<String>),
    Ranked(Vec<String>),
}

impl SingleBallot {
    /// Return the form of this ballot
    fn ballot_form(&self) -> BallotForm {
        match self {
            SingleBallot::Uninominal(_) => BallotForm::Uninominal,
            SingleBallot::Approved(_) => BallotForm::Approved,
            SingleBallot::Ranked(_) => BallotForm::Ranked,
        }
    }
}

/// Trait for ballots boxes
pub trait Ballots: Sized {
    /// Create a new ballots box
    fn build(ballot_form: BallotForm, choices: Vec<String>) -> Result<Self, InvalidBallot>;

    /// Get all available choices
    fn choices(&self) -> impl Iterator<Item = &String>;

    /// Cast a vote in the ballots box
    fn vote(&mut self, ballot: SingleBallot) -> Result<(), InvalidBallot>;
}

/// Type for ballot boxes where each candidate has points
pub struct PointBallots(pub HashMap<String, i32>);

impl Ballots for PointBallots {
    fn build(ballot_form: BallotForm, choices: Vec<String>) -> Result<Self, InvalidBallot> {
        match ballot_form {
            BallotForm::Uninominal | BallotForm::Approved | BallotForm::Ranked => {
                let mut choices_hashmap: HashMap<String, i32> = HashMap::new();

                choices.iter().map(|s| String::from(s)).for_each(|c| {
                    choices_hashmap.insert(c, 0);
                });
                Ok(Self(choices_hashmap))
            }
        }
    }

    fn choices(&self) -> impl Iterator<Item = &String> {
        let PointBallots(c) = self;
        c.keys()
    }

    fn vote(&mut self, ballot: SingleBallot) -> Result<(), InvalidBallot> {
        let PointBallots(c) = self;

        match ballot {
            SingleBallot::Uninominal(b) => {
                c.get(&b)
                    .ok_or(InvalidBallot(format!("unknown candidate {}", b)))?;
                c.entry(b).and_modify(|count| *count += 1);
            }
            SingleBallot::Approved(vec_approved) => {
                for b in vec_approved {
                    c.get(&b)
                        .ok_or(InvalidBallot(format!("unknown candidate {}", b)))?;
                    c.entry(b).and_modify(|count| *count += 1);
                }
            }
            SingleBallot::Ranked(vec_ranked) => {
                let mut rank = 1;
                for b in vec_ranked {
                    c.get(&b)
                        .ok_or(InvalidBallot(format!("unknown candidate {}", b)))?;
                    c.entry(b).and_modify(|count| *count += rank);
                    rank += 1;
                }
            } // _ => Err(InvalidBallot("Incompatible ballot form".to_string()))?
        }

        Ok(())
    }
}

/// Type for ballot boxes where each candidate is in a kind of battle
/// with each other
pub struct BattleBallots(pub HashMap<(String, String), i32>);

impl Ballots for BattleBallots {
    fn build(ballot_form: BallotForm, choices: Vec<String>) -> Result<Self, InvalidBallot> {
        match ballot_form {
            BallotForm::Uninominal | BallotForm::Approved => Err(InvalidBallot(format!(
                "Incompatible ballot form {}",
                ballot_form
            ))),
            BallotForm::Ranked => {
                let mut choices_hashmap: HashMap<(String, String), i32> = HashMap::new();

                choices.iter().for_each(|c1| {
                    choices.iter().for_each(|c2| {
                        choices_hashmap.insert((String::from(c1), String::from(c2)), 0);
                    });
                });
                Ok(Self(choices_hashmap))
            }
        }
    }

    fn choices(&self) -> impl Iterator<Item = &String> {
        let BattleBallots(c) = self;
        HashSet::<&String>::from_iter(c.keys().map(|(a, _)| a)).into_iter()
    }

    fn vote(&mut self, ballot: SingleBallot) -> Result<(), InvalidBallot> {
        let BattleBallots(c) = self;

        match ballot {
            SingleBallot::Ranked(vec_ranked) => {
                let mut b_previous = None;
                let choices = HashSet::<String>::from_iter(c.keys().cloned().map(|(a, _)| a));

                for b in vec_ranked {
                    // Check if b is an available choice
                    if !choices.contains(&b) {
                        Err(InvalidBallot(format!("unknown candidate {}", b)))?
                    }

                    b_previous.map(|bp| {
                        c.entry((b.clone(), bp)).and_modify(|count| *count += 1)
                    });

                    b_previous = Some(b)
                }
            }
            _ => Err(InvalidBallot("Incompatible ballot form".to_string()))?,
        }

        Ok(())
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
    /// Create a new  minimal voting system info
    pub fn new(name: &str, ballot_form: BallotForm, choices: Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            choices,
            ballot_form,
        }
    }

    /// Get all available choices
    pub fn get_choices(&self) -> Vec<String> {
        self.choices.clone()
    }

    /// Get ballot form
    pub fn get_ballot_form(&self) -> BallotForm {
        self.ballot_form
    }

    /// Check if ballot could be cast or not
    pub fn check_ballot(&self, ballot: &SingleBallot) -> Result<(), InvalidBallot> {
        match (self.ballot_form, ballot) {
            (BallotForm::Uninominal, SingleBallot::Uninominal(b)) => {
                if self.choices.contains(&b) {
                    Ok(())
                } else {
                    Err(InvalidBallot(format!(
                        "Ballot didn't contain a available choice"
                    )))
                }
            }
            (BallotForm::Approved, SingleBallot::Approved(vec_b)) => {
                let mut uniques = HashSet::new();
                if vec_b
                    .iter()
                    .all(|b| self.choices.contains(&b) && uniques.insert(b.clone()))
                {
                    Ok(())
                } else {
                    Err(InvalidBallot(format!(
                        "Ballot contains an unavailable choice"
                    )))
                }
            }
            (BallotForm::Ranked, SingleBallot::Ranked(vec_b)) => {
                let mut uniques = HashSet::new();
                if vec_b
                    .iter()
                    .all(|b| self.choices.contains(&b) && uniques.insert(b.clone()))
                    && uniques.len() == self.choices.len()
                {
                    Ok(())
                } else {
                    Err(InvalidBallot(format!(
                        "Ballot contains an unavailable choice or few or too choices: {:?}",
                        ballot
                    )))
                }
            }
            _ => Err(InvalidBallot(format!(
                "invalid ballot form : {} instead of {}",
                ballot.ballot_form(),
                self.ballot_form
            ))),
        }
    }
}

impl fmt::Display for MinimalVotingSystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name)?;

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
pub struct VotingSystemInfo<B: Ballots> {
    /// The name of the voting system
    name: String,
    /// The ballots' form
    ballot_form: BallotForm,
    /// All ballots of the voting system
    ballot_box: B,
    /// Total number of ballots
    count: usize,
}

impl<B: Ballots> VotingSystemInfo<B> {
    /// Create a new voting system info
    pub(crate) fn build(
        name: &str,
        ballot_form: BallotForm,
        choices: Vec<String>,
    ) -> Result<Self, InvalidBallot> {
        Ok(Self {
            name: name.to_owned(),
            ballot_form,
            ballot_box: B::build(ballot_form, choices)?,
            count: 0,
        })
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
    pub fn get_ballot_box(&self) -> &B {
        &self.ballot_box
    }

    /// Get the total number of ballots
    pub fn get_count(&self) -> usize {
        self.count
    }

    /// Get the MinimalVotingSystemInfo representing this VotingSystemInfo
    pub fn get_minimal_info(&self) -> MinimalVotingSystemInfo {
        MinimalVotingSystemInfo::new(
            &self.name,
            self.ballot_form,
            self.get_choices().map(|s| s.to_owned()).collect(),
        )
    }

    /// Just vote
    pub fn vote(&mut self, ballot: SingleBallot) -> Result<(), InvalidBallot> {
        self.get_minimal_info().check_ballot(&ballot)?;

        self.ballot_box.vote(ballot)?;

        self.count += 1;
        Ok(())
    }
}

pub trait VotingSystem {
    type B: Ballots;

    const NAME: &str;
    const LONG_NAME: &str;

    /// Create a new election
    fn new(choices: Vec<String>) -> Self;

    /// Algorithm finding the result of the election from all ballots
    // fn result_algorithm(ballots: &Ballots) -> String;
    fn result(&self) -> String;

    /// Get all information about this election
    fn get_info(&self) -> &VotingSystemInfo<Self::B>;

    /// Get all mutable information about this election
    fn get_mut_info(&mut self) -> &mut VotingSystemInfo<Self::B>;

    /// Cast ballot
    fn vote(&mut self, ballot: SingleBallot) -> Result<(), InvalidBallot> {
        self.get_mut_info().vote(ballot)
    }

    fn get_minimal_info(&self) -> MinimalVotingSystemInfo {
        self.get_info().get_minimal_info()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ballot_trait() {
        let p = PointBallots::build(
            BallotForm::Uninominal,
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
        )
        .unwrap();

        assert_eq!(
            HashSet::<&String>::from_iter(p.choices()),
            HashSet::from_iter(
                vec![&"A".to_string(), &"B".to_string(), &"C".to_string()].into_iter()
            )
        );

        let b = BattleBallots::build(
            BallotForm::Ranked,
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
        )
        .unwrap();

        assert_eq!(
            HashSet::<&String>::from_iter(b.choices()),
            HashSet::from_iter(
                vec![&"A".to_string(), &"B".to_string(), &"C".to_string()].into_iter()
            )
        );

        assert!(BattleBallots::build(BallotForm::Uninominal, vec!["A".to_string()]).is_err());
    }

    #[test]
    fn ballot_uninominal() {
        let mvsi = MinimalVotingSystemInfo::new(
            "test",
            BallotForm::Uninominal,
            vec![String::from("A"), String::from("B"), String::from("C")],
        );

        assert!(
            mvsi.check_ballot(&SingleBallot::Uninominal("A".to_string()))
                .is_ok()
        );
        assert!(
            mvsi.check_ballot(&SingleBallot::Approved(vec![
                "A".to_string(),
                "C".to_string()
            ]))
            .is_err()
        );
    }

    #[test]
    fn ballot_approved() {
        let mvsi = MinimalVotingSystemInfo::new(
            "test",
            BallotForm::Approved,
            vec![String::from("A"), String::from("B"), String::from("C")],
        );

        assert!(
            mvsi.check_ballot(&SingleBallot::Approved(vec![
                "A".to_string(),
                "C".to_string()
            ]))
            .is_ok()
        );
        assert!(
            mvsi.check_ballot(&SingleBallot::Uninominal("A".to_string()))
                .is_err()
        );
    }

    #[test]
    fn ballot_ranked() {
        let mvsi = MinimalVotingSystemInfo::new(
            "test",
            BallotForm::Ranked,
            vec![String::from("A"), String::from("B"), String::from("C")],
        );

        assert!(
            mvsi.check_ballot(&SingleBallot::Ranked(vec![
                "A".to_string(),
                "C".to_string(),
                "B".to_string()
            ]))
            .is_ok()
        );
        assert!(
            mvsi.check_ballot(&SingleBallot::Ranked(vec!["A".to_string()]))
                .is_err()
        );
        assert!(
            mvsi.check_ballot(&SingleBallot::Uninominal("A".to_string()))
                .is_err()
        );
    }
}

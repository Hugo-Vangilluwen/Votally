use std::error::Error;
use std::fmt;

mod definition;

pub use self::definition::{BallotForm, MinimalVotingSystemInfo, SingleBallot, VotingSystem};

mod plurality;
pub use self::plurality::Plurality;

/// Error for unknown voting system
#[derive(Debug)]
pub struct UnknownVotingSystem(String);

impl fmt::Display for UnknownVotingSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown voting system: {}", self.0)
    }
}

impl Error for UnknownVotingSystem {}

/// Try to find the voting system which is associated to name.
/// Return a voting system if found
/// and return a UnknownVotingSystem error else.
pub fn find_voting_system(
    name: &str,
    choices: Vec<String>,
) -> Result<impl definition::VotingSystem, UnknownVotingSystem> {
    match name {
        self::plurality::NAME => Ok(Plurality::new(choices)),
        _ => Err(UnknownVotingSystem(format!("{}", name))),
    }
}

/// Try to find the voting system which is associated to name.
/// Return minimal information of the voting system if found
/// and return a UnknownVotingSystem error else.
pub fn find_info_voting_system(
    name: &str,
    choices: Vec<String>,
) -> Result<MinimalVotingSystemInfo, UnknownVotingSystem> {
    match name {
        self::plurality::NAME => Ok(MinimalVotingSystemInfo::new(
            name,
            BallotForm::Uninominal,
            choices,
        )),
        _ => Err(UnknownVotingSystem(format!("{}", name))),
    }
}

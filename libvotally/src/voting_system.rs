use std::error::Error;
use std::fmt;

mod definition;

use crate::voting_system::definition::InvalidBallot;

pub use self::definition::{BallotForm, MinimalVotingSystemInfo, SingleBallot, VotingSystem};

mod plurality;
pub use self::plurality::Plurality;

mod approval;
pub use self::approval::Approval;

mod borda_count;
pub use self::borda_count::BordaCount;

/// Error for unknown voting system
#[derive(Debug)]
pub struct UnknownVotingSystem(String);

impl fmt::Display for UnknownVotingSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown voting system: {}", self.0)
    }
}

impl Error for UnknownVotingSystem {}

pub enum VotingSystemEnum {
    Plurality(Plurality),
    Approval(Approval),
    Borda(BordaCount),
}

impl VotingSystemEnum {
    pub fn result(&self) -> String {
        match self {
            VotingSystemEnum::Plurality(p) => p.result(),
            VotingSystemEnum::Approval(a) => a.result(),
            VotingSystemEnum::Borda(b) => b.result(),
        }
    }

    pub fn get_minimal_info(&self) -> MinimalVotingSystemInfo {
        match self {
            VotingSystemEnum::Plurality(p) => p.get_minimal_info(),
            VotingSystemEnum::Approval(a) => a.get_minimal_info(),
            VotingSystemEnum::Borda(b) => b.get_minimal_info(),
        }
    }

    pub fn vote(&mut self, ballot: SingleBallot) -> Result<(), InvalidBallot> {
        match self {
            VotingSystemEnum::Plurality(p) => p.vote(ballot),
            VotingSystemEnum::Approval(a) => a.vote(ballot),
            VotingSystemEnum::Borda(b) => b.vote(ballot),
        }
    }
}

/// Try to find the voting system which is associated to name.
/// Return a voting system if found
/// and return a UnknownVotingSystem error else.
pub fn find_voting_system(
    name: &str,
    choices: Vec<String>,
) -> Result<VotingSystemEnum, UnknownVotingSystem> {
    match name {
        Plurality::NAME => Ok(VotingSystemEnum::Plurality(Plurality::new(choices))),
        Approval::NAME => Ok(VotingSystemEnum::Approval(Approval::new(choices))),
        BordaCount::NAME => Ok(VotingSystemEnum::Borda(BordaCount::new(choices))),
        _ => Err(UnknownVotingSystem(format!("{}", name))),
    }
}

/// Return Ok(()) if name_vote is known and Err(UnknownVotingSystem) else
/// Current known voting system: plurality
pub fn correct_voting_system(name_vote: &str) -> Result<(), UnknownVotingSystem> {
    if vec![Plurality::NAME, Approval::NAME, BordaCount::NAME].contains(&name_vote) {
        Ok(())
    } else {
        Err(UnknownVotingSystem(format!("{}", name_vote)))
    }
}

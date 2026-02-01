use std::error::Error;
use std::fmt;

mod definition;

use crate::voting_system::definition::VotingSystemInfo;

pub use self::definition::{BallotForm, MinimalVotingSystemInfo, SingleBallot, VotingSystem};

mod plurality;
pub use self::plurality::Plurality;

mod approval;
pub use self::approval::Approval;

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
}

impl VotingSystem for VotingSystemEnum {
    fn result(&self) -> String {
        match self {
            VotingSystemEnum::Plurality(p) => p.result(),
            VotingSystemEnum::Approval(a) => a.result(),
        }
    }

    fn get_info(&self) -> &VotingSystemInfo {
        match self {
            VotingSystemEnum::Plurality(p) => p.get_info(),
            VotingSystemEnum::Approval(a) => a.get_info(),
        }
    }

    fn get_mut_info(&mut self) -> &mut VotingSystemInfo {
        match self {
            VotingSystemEnum::Plurality(p) => p.get_mut_info(),
            VotingSystemEnum::Approval(a) => a.get_mut_info(),
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
        self::plurality::NAME => Ok(VotingSystemEnum::Plurality(Plurality::new(choices))),
        self::approval::NAME => Ok(VotingSystemEnum::Approval(Approval::new(choices))),
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
        self::approval::NAME => Ok(MinimalVotingSystemInfo::new(
            name,
            BallotForm::Approved,
            choices,
        )),
        _ => Err(UnknownVotingSystem(format!("{}", name))),
    }
}

/// Return Ok(()) if name_vote is known and Err(UnknownVotingSystem) else
/// Current known voting system: plurality
pub fn correct_voting_system(name_vote: &str) -> Result<(), UnknownVotingSystem> {
    match name_vote {
        self::plurality::NAME | self::approval::NAME => Ok(()),
        _ => Err(UnknownVotingSystem(format!("{}", name_vote))),
    }
}

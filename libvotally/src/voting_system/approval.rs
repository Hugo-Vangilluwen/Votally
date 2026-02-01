use crate::voting_system::definition::*;

pub(crate) const NAME: &str = "approval";

/// # Approval voting
///
/// Here an exemple :
/// ```rust
/// use libvotally::voting_system::{Approval, VotingSystem, SingleBallot};
///
/// let mut p = Approval::new(vec![
///     String::from("A"),
///     String::from("B"),
///     String::from("C")
/// ]);
///
/// p.vote(SingleBallot::Approved(vec!["A".to_string(), "B".to_string()]));
/// p.vote(SingleBallot::Approved(vec!["B".to_string()]));
///
/// assert_eq!("B", p.result());
/// ```
pub struct Approval {
    info: VotingSystemInfo,
}

impl Approval {
    pub fn new(choices: Vec<String>) -> Self {
        Self {
            info: VotingSystemInfo::new(NAME, BallotForm::Uninominal, choices),
        }
    }
}

impl VotingSystem for Approval {
    fn get_info(&self) -> &VotingSystemInfo {
        &self.info
    }

    fn get_mut_info(&mut self) -> &mut VotingSystemInfo {
        &mut self.info
    }

    fn result(&self) -> String {
        match self.info.get_ballot_box() {
            Ballots::Rates(c) => c
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _v)| k)
                .cloned()
                .unwrap(),
            // _ => unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approval_voting() {
        let mut p = Approval::new(vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
        ]);

        for v in vec!["A", "B", "A", "C", "B", "A"] {
            p.vote(SingleBallot::Approved(vec![v.to_string()])).unwrap();
        }

        assert_eq!("A", p.result());
    }
}

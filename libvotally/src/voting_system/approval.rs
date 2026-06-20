use crate::voting_system::definition::*;

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
    info: VotingSystemInfo<PointBallots>,
}

impl VotingSystem for Approval {
    type B = PointBallots;

    const NAME: &str = "approval";
    const LONG_NAME: &str = "Approval voting";

    fn new(choices: Vec<String>) -> Self {
        Self {
            info: VotingSystemInfo::new(Self::LONG_NAME, BallotForm::Approved, choices),
        }
    }

    fn get_info(&self) -> &VotingSystemInfo<PointBallots> {
        &self.info
    }

    fn get_mut_info(&mut self) -> &mut VotingSystemInfo<PointBallots> {
        &mut self.info
    }

    fn result(&self) -> String {
        let PointBallots(c) = self.info.get_ballot_box();
        c.iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .cloned()
            .unwrap()
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

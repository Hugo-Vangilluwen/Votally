use crate::voting_system::definition::*;

/// # Approval voting
///
/// Here an exemple :
/// ```rust
/// use libvotally::voting_system::{Approval, VotingSystem, SingleBallot};
///
/// let mut p = Approval::new(&vec!["A", "B", "C"]);
///
/// p.vote(SingleBallot::Approved(vec![
///     "A".to_string(),
///     "B".to_string()
/// ])).unwrap();
/// p.vote(SingleBallot::Approved(vec!["B".to_string()])).unwrap();
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

    fn new(choices: &Vec<&str>) -> Self {
        Self {
            info: VotingSystemInfo::build(Self::LONG_NAME, BallotForm::Approved, choices).unwrap(),
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
        let mut p = Approval::new(&vec!["A", "B", "C"]);

        for v in vec!["A", "B", "A", "C", "B", "A"] {
            p.vote(SingleBallot::Approved(vec![v.to_string()])).unwrap();
        }

        assert_eq!("A", p.result());
    }
}

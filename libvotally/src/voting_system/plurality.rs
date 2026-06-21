use crate::voting_system::definition::*;

/// # First-past-the-post voting
///
/// Here an exemple :
/// ```rust
/// use libvotally::voting_system::{Plurality, VotingSystem, SingleBallot};
///
/// let mut p = Plurality::new(&vec!["A", "B", "C"]);
///
/// p.vote(SingleBallot::Uninominal("A".to_string()));
/// p.vote(SingleBallot::Uninominal("B".to_string()));
/// p.vote(SingleBallot::Uninominal("C".to_string()));
/// p.vote(SingleBallot::Uninominal("A".to_string()));
///
/// assert_eq!("A", p.result());
/// ```
pub struct Plurality {
    info: VotingSystemInfo<PointBallots>,
}

impl VotingSystem for Plurality {
    type B = PointBallots;

    const NAME: &str = "plurality";
    const LONG_NAME: &str = "Plurality voting";

    fn new(choices: &Vec<&str>) -> Self {
        Self {
            info: VotingSystemInfo::build(Self::LONG_NAME, BallotForm::Uninominal, choices)
                .unwrap(),
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
    fn plurality_voting() {
        let mut p = Plurality::new(&vec!["A", "B", "C"]);

        for v in vec!["A", "B", "A", "C", "B", "A"] {
            p.vote(SingleBallot::Uninominal(v.to_string())).unwrap();
        }

        assert_eq!("A", p.result());
    }
}

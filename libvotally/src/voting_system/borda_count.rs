use crate::voting_system::definition::*;

/// # Borda count
///
/// Here an exemple :
/// ```rust
/// use libvotally::voting_system::{BordaCount, VotingSystem, SingleBallot};
///
/// let mut p = BordaCount::new(vec![
///     String::from("A"),
///     String::from("B"),
///     String::from("C"),
/// ]);
///
/// p.vote(SingleBallot::Ranked(vec![
///     "A".to_string(),
///     "B".to_string(),
///     "C".to_string(),
/// ])).unwrap();
/// p.vote(SingleBallot::Ranked(vec![
///     "C".to_string(),
///     "A".to_string(),
///     "B".to_string(),
/// ])).unwrap();
///
/// assert_eq!("A", p.result());
/// ```
pub struct BordaCount {
    info: VotingSystemInfo<PointBallots>,
}

impl VotingSystem for BordaCount {
    type B = PointBallots;

    const NAME: &str = "borda";
    const LONG_NAME: &str = "Borda count";

    fn new(choices: Vec<String>) -> Self {
        Self {
            info: VotingSystemInfo::new(Self::LONG_NAME, BallotForm::Ranked, choices),
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
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .cloned()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borda_voting() {
        let mut p = BordaCount::new(vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
        ]);

        for v in vec![vec!["A", "B", "C"], vec!["C", "A", "B"]] {
            p.vote(SingleBallot::Ranked(
                v.into_iter().map(|s| s.to_string()).collect(),
            ))
            .unwrap();
        }

        assert_eq!("A", p.result());
    }
}

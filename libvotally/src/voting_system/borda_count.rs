use crate::voting_system::definition::*;

pub(crate) const NAME: &str = "borda";
pub(crate) const LONG_NAME: &str = "Borda count";

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
    info: VotingSystemInfo,
}

impl BordaCount {
    pub fn new(choices: Vec<String>) -> Self {
        Self {
            info: VotingSystemInfo::new(NAME, BallotForm::Ranked, choices),
        }
    }
}

impl VotingSystem for BordaCount {
    fn get_info(&self) -> &VotingSystemInfo {
        &self.info
    }

    fn get_mut_info(&mut self) -> &mut VotingSystemInfo {
        &mut self.info
    }

    fn result(&self) -> String {
        match self.info.get_ballot_box() {
            Ballots::Points(c) => c
                .iter()
                .min_by(|a, b| a.1.cmp(&b.1))
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

    #[test]
    #[should_panic]
    fn invalid_ballot() {
        let mut p = BordaCount::new(vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
        ]);

        p.vote(SingleBallot::Ranked(vec!["A".to_string()])).unwrap();
    }

    #[test]
    #[should_panic]
    fn not_ranked_ballot() {
        let mut p = BordaCount::new(vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
        ]);

        p.vote(SingleBallot::Uninominal("A".to_string())).unwrap();
    }
}

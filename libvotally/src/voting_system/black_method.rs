use crate::voting_system::definition::*;

impl Ballots for (PointBallots, BattleBallots) {
    fn new(choices: &Vec<&str>) -> Self {
        (PointBallots::new(choices), BattleBallots::new(choices))
    }

    fn choices(&self) -> impl Iterator<Item = &String> {
        let (p, _) = self;
        p.choices()
    }

    fn vote(&mut self, ballot: SingleBallot) -> Result<(), InvalidBallot> {
        let (p, b) = self;
        p.vote(ballot.clone())?;
        b.vote(ballot)
    }
}

/// # Black's method
/// This method was proposed by Duncan Black in 1958
/// as a comprised between Condorcet method and Borda count.
/// See [Black_wikipedia].
///
/// Here an exemple :
/// ```rust
/// use libvotally::voting_system::{BlackMethod, VotingSystem, SingleBallot};
///
/// let mut b = BlackMethod::new(&vec!["A", "B", "C"]);
///
/// b.vote(SingleBallot::Ranked(vec![
///     "A".to_string(),
///     "B".to_string(),
///     "C".to_string(),
/// ])).unwrap();
/// b.vote(SingleBallot::Ranked(vec![
///     "C".to_string(),
///     "A".to_string(),
///     "B".to_string(),
/// ])).unwrap();
///
/// assert_eq!("A", b.result());
/// ```
///
/// [Black_wikipedia]: https://en.wikipedia.org/wiki/Black's_method
pub struct BlackMethod(VotingSystemInfo<(PointBallots, BattleBallots)>);

impl VotingSystem for BlackMethod {
    type B = (PointBallots, BattleBallots);

    const NAME: &str = "black";
    const LONG_NAME: &str = "Black's method";

    fn new(choices: &Vec<&str>) -> Self {
        Self(VotingSystemInfo::new(
            Self::LONG_NAME,
            BallotForm::Ranked,
            choices,
        ))
    }

    fn get_info(&self) -> &VotingSystemInfo<Self::B> {
        &self.0
    }

    fn get_mut_info(&mut self) -> &mut VotingSystemInfo<Self::B> {
        &mut self.0
    }

    fn result(&self) -> String {
        let (PointBallots(p), BattleBallots(b)) = self.0.get_ballot_box();
        let ch = p.keys();

        // Condorcet winner ?
        match ch
            .clone()
            .filter(|c1| {
                ch.clone().all(|c2| {
                    (c1.to_string() == c2.to_string())
                        || (*b.get(&(c1.to_string(), c2.to_string())).unwrap()
                            > *b.get(&(c2.to_string(), c1.to_string())).unwrap())
                })
            })
            .next()
        {
            Some(c) => c.to_string(),
            None =>
            // Borda count
            {
                println!("Borda");
                p.iter()
                    .min_by(|a, b| a.1.cmp(&b.1))
                    .map(|(k, _v)| k)
                    .cloned()
                    .unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_method() {
        let mut black1 = BlackMethod::new(&vec!["A", "B", "C"]);

        for v in vec![
            vec!["A", "B", "C"],
            vec!["A", "B", "C"],
            vec!["A", "B", "C"],
            vec!["B", "C", "A"],
            vec!["B", "C", "A"],
        ] {
            black1
                .vote(SingleBallot::Ranked(
                    v.into_iter().map(|s| s.to_string()).collect(),
                ))
                .unwrap();
        }

        assert_eq!("A", black1.result());

        let mut black2 = BlackMethod::new(&vec!["A", "B", "C"]);

        for v in vec![
            vec!["A", "B", "C"],
            vec!["A", "B", "C"],
            vec!["B", "C", "A"],
            vec!["B", "C", "A"],
        ] {
            black2
                .vote(SingleBallot::Ranked(
                    v.into_iter().map(|s| s.to_string()).collect(),
                ))
                .unwrap();
        }

        assert_eq!("B", black2.result());
    }
}

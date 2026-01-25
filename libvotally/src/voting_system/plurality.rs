use crate::voting_system::definition::*;

pub(crate) const NAME: &str = "plurality";

/// # First-past-the-post voting
///
/// Here an exemple :
/// ```rust
/// use libvotally::voting_system::{Plurality, VotingSystem};
///
/// let mut p = Plurality::new(vec![
///     String::from("A"),
///     String::from("B"),
///     String::from("C")
/// ]);
///
/// p.vote("A");
/// p.vote("B");
/// p.vote("C");
/// p.vote("A");
///
/// assert_eq!("A", p.result().unwrap());
/// ```
// pub fn plurality(choices: impl Iterator<Item = String>) -> VotingSystem {
//     VotingSystem::new(
//         NAME,
//         BallotForm::Uninominal,
//         choices,
//         Box::new(|choices: &Ballots| {
//             match choices {
//                 Ballots::Uninominal(c) => c
//                     .iter()
//                     .max_by(|a, b| a.1.cmp(&b.1))
//                     .map(|(k, _v)| k)
//                     .cloned(),
//                 // _ => unimplemented!()
//             }
//         }),
//     )
// }

pub struct Plurality {
    info: VotingSystemInfo,
}

impl VotingSystem for Plurality {
    // choices: impl Iterator<Item = String>,
    // result_algorithm: ResultAlgorithm,
    fn new(choices: Vec<String>) -> Self {
        Self {
            info: VotingSystemInfo::new("plurality", BallotForm::Uninominal, choices),
        }
    }

    fn get_info(&self) -> &VotingSystemInfo {
        &self.info
    }

    fn get_mut_info(&mut self) -> &mut VotingSystemInfo {
        &mut self.info
    }

    fn result_algorithm(ballots: &Ballots) -> Option<String> {
        match ballots {
            Ballots::Uninominal(c) => c
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _v)| k)
                .cloned(),
            // _ => unimplemented!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plurality_voting() {
        let mut p = Plurality::new(vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
        ]);

        for v in vec!["A", "B", "A", "C", "B", "A"] {
            p.vote(v).unwrap();
        }

        assert_eq!("A", p.result().unwrap());
    }
}

use crate::voting_system::definition::*;

pub(crate) const NAME: &str = "plurality";

/// # First-past-the-post voting
///
/// Here an exemple :
/// ```rust
/// use libvotal::voting_system::plurality;
///
/// let mut p =
///     plurality(vec![String::from("A"), String::from("B"), String::from("C")].into_iter());
///
/// p.vote("A");
/// p.vote("B");
/// p.vote("C");
/// p.vote("A");
///
/// assert_eq!("A", p.result());
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
//     /// The name of the voting system
//     name: String,
//     // The ballot form of the voting system
//     // ballot_form: BallotForm,
    /// All ballots of the voting system
    pub ballot_box: Ballots,
//     /// Calculate the election's result
//     result_algorithm: ResultAlgorithm,
    /// Total number of ballots
    count: usize,
}

impl VotingSystem for Plurality {
    /// Create a new election
    fn new(
        choices: impl Iterator<Item = String>,
    ) -> Plurality {
        Self {
            // ballot_form,
            ballot_box: Ballots::new(BallotForm::Uninominal, choices),
            count: 0,
        }
    }

    /// Get the name of the voting system
    fn get_name(&self) -> &str {
        NAME
    }

    fn get_ballot_box(&mut self) -> &mut Ballots {
        &mut (self.ballot_box)
    }

    fn get_choices(&self) -> impl Iterator<Item = &String> {
        self.ballot_box.choices()
    }

    fn get_ballot_form(&self) -> BallotForm {
        self.ballot_box.ballot_form()
    }

    fn add_ballot(&mut self) {
        self.count += 1;
    }

    /// Get the total number of ballots
    fn get_count(&self) -> usize {
        self.count
    }

    /// Calculate the election's result
    fn result(&self) -> Option<String> {
        match &self.ballot_box {
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
        let mut p =
            Plurality::new(vec![String::from("A"), String::from("B"), String::from("C")].into_iter());

        for v in vec!["A", "B", "A", "C", "B", "A"] {
            p.vote(v).unwrap();
        }

        assert_eq!(Some("A"), p.result().as_deref());
    }
}

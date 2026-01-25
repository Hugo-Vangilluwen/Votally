use tokio::io::{self, AsyncBufReadExt};

// use libvotally::voting_system::*;

/// Announce in the console the different choices
pub fn announce_choices(choices: &Vec<String>) {
    let mut choices_iter = choices.iter();

    print!("Different choices are {}", choices_iter.next().unwrap());
    for c in choices_iter {
        print!(", {}", c);
    }
    println!();

    // println!("Enter your vote: ");
}

/// Read a vote
pub async fn read_vote() -> String {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);
    let mut ballot = String::new();

    println!("Enter your choice:");
    reader.read_line(&mut ballot).await.unwrap();

    ballot.to_string().trim().to_owned()
}

// pub fn read_vote<VS: VotingSystem>(vote: &mut VS) {
// annouce_choices(vote.get_info().get_choices());

// match vote.get_info().get_ballot_form() {
//     BallotForm::Uninominal => {
//         let mut ballot = String::new();
//
//         io::stdin()
//             .read_line(&mut ballot)
//             .expect("Failed to read line");
//
//         match vote.vote(ballot.trim()) {
//             Ok(_) => println!("Vote cast!"),
//             Err(e) => eprintln!("{}", e),
//         }
//     }
// }
// }

/// Wait for the user to press enter
pub fn press_enter(message: &str) {
    println!("Press enter to {}", message);

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
}

use libvotally::voting_system::{BallotForm, SingleBallot};
use tokio::io::{self, AsyncBufReadExt};

/// Read a vote
pub async fn read_vote(ballot_form: &BallotForm) -> SingleBallot {
    match ballot_form {
        BallotForm::Uninominal => {
            let stdin = io::stdin();
            let mut reader = io::BufReader::new(stdin);
            let mut ballot = String::new();

            println!("Enter your choice:");
            reader.read_line(&mut ballot).await.unwrap();

            SingleBallot::Uninominal(ballot.to_string().trim().to_owned())
        }
    }
}

/// Wait for the user to press enter
pub fn press_enter(message: &str) {
    println!("Press enter to {}", message);

    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
}

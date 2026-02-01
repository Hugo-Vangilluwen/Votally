use libvotally::voting_system::{BallotForm, SingleBallot};
use tokio::io::{self, AsyncBufReadExt};

/// Read a vote
pub async fn read_vote(ballot_form: &BallotForm) -> io::Result<SingleBallot> {
    let stdin = io::stdin();
    let mut stdin_reader = io::BufReader::new(stdin);

    match ballot_form {
        BallotForm::Uninominal => {
            let mut buffer = String::new();

            println!("Enter your choice:");
            stdin_reader.read_line(&mut buffer).await?;

            let ballot = buffer.to_string().trim().to_owned();
            Ok(SingleBallot::Uninominal(ballot))
        }
        BallotForm::Approved => {
            let mut buffer = String::new();

            println!("Enter your approved choices separated with comma:");
            stdin_reader.read_line(&mut buffer).await?;

            let ballot = buffer.split(',').map(|s| s.trim().to_owned()).collect();
            Ok(SingleBallot::Approved(ballot))
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

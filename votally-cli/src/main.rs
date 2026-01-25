use std::process;

use clap::Parser;

use libvotally::network::{VotallyClient, VotallyServer};

use votally_cli::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Create a server for holding a vote
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    server: bool,

    /// Name of the used voting system among plurality
    #[arg(short, long, default_value = "plurality")]
    voting_system: String,

    /// List of choices for a server
    // #[arg(short, long)]
    choices: Vec<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.server {
        if cli.choices.len() <= 1 {
            eprintln!("There is not enough choice.");
            process::exit(1);
        }

        let mut server = VotallyServer::new("localhost", cli.voting_system, cli.choices).await;

        press_enter("start ballot");

        server.start_ballot().await.unwrap();

        press_enter("end vote");

        server.end_poll().await;

        server.calculate_result().await;

        println!("Winner: {}", server.result());
    } else {
        let mut client = VotallyClient::new("localhost").await;
        println!("Client started !");

        let choices = client.get_info().await;
        announce_choices(&choices);

        let mut ballot = read_vote().await;
        while !choices.contains(&ballot) {
            println!("Enter your choice:");
            ballot = read_vote().await;
        }

        client.send_vote(&ballot).await;
        println!("Vote cast !");

        println!("Winner: {}", client.result().await);
    }
}

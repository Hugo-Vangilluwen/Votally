use std::process;

use tokio::io::{self, AsyncBufReadExt};

use clap::Parser;

use libvotally::network::{VotallyClient, VotallyServer};
use libvotally::voting_system::UnknownVotingSystem;

use votally_cli::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Create a server for holding a vote
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    server: bool,

    /// Name of the used voting system among approval, plurality, borda, black
    #[arg(short, long, default_value = "approval")]
    voting_system: String,

    /// List of choices for a server
    // #[arg(short, long)]
    choices: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), UnknownVotingSystem> {
    let cli = Cli::parse();

    if cli.server {
        if cli.choices.len() <= 1 {
            eprintln!("There is not enough choice.");
            process::exit(1);
        }

        let server_address = local_ip_address::local_ip().unwrap();
        println!("Server IP: {}", server_address);
        let mut server =
            VotallyServer::build(server_address.to_string(), cli.voting_system, &cli.choices.iter().map(|c| c as &str).collect())
                .await?;

        press_enter("start ballot");

        server.start_ballot().await.unwrap();

        press_enter("end vote");

        server.end_poll().await;

        server.calculate_result().await;

        println!("Winner: {}", server.result());
    } else {
        let mut server_address = String::new();

        {
            let mut stdin_reader = io::BufReader::new(io::stdin());

            println!("Enter your server IP:");
            stdin_reader.read_line(&mut server_address).await.unwrap();
            server_address = server_address.trim().to_owned();
        }

        let mut client = VotallyClient::new(server_address).await;
        println!("Client started !");

        let info = client.get_info().await;
        println!("{}", info);
        let ballot_form = info.get_ballot_form();

        let mut ballot = read_vote(&ballot_form).await.unwrap();
        while info
            .check_ballot(&ballot)
            .inspect_err(|e| println!("{e}"))
            .is_err()
        {
            ballot = read_vote(&ballot_form).await.unwrap();
        }
        println!("Valid ballot");

        client.send_vote(&ballot).await;
        println!("Vote cast !");

        println!("Winner: {}", client.result().await);
    }

    Ok(())
}

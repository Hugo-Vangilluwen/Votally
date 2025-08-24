// use std::env;
use std::process;

use clap::Parser;

use libvotally::voting_system::find_voting_system;
// use libvotally::network::VotallyServer;
use libvotally::network::{VotallyClient, VotallyServer};
// use votally_cli::read_vote;

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

fn main() {
    // let (config_server, vote) = ConfigServer::build(env::args()).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    let cli = Cli::parse();

    if cli.server {
        if cli.choices.len() <= 1 {
            eprintln!("There is not enough choice.");
            process::exit(1);
        }

        let vote = find_voting_system(&cli.voting_system).unwrap_or_else(
            |err| {
                eprintln!("{}", err);
                process::exit(1);
            },
        )(cli.choices.into_iter());

        let server = VotallyServer::new("localhost", vote);

        server.answer_many(1);

        // read_vote(&mut vote);

        // println!("The winner is {}", vote.result());
    } else {
        println!("I'm a client.");

        let mut client = VotallyClient::new("localhost");

        println!("{}", client.get_info());
    }
}

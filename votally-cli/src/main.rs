use std::io::BufRead;
// use std::env;
use std::{io::stdin, process};

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

        // let vote = find_voting_system(&cli.voting_system).unwrap_or_else(
        //     |err| {
        //         eprintln!("{}", err);
        //         process::exit(1);
        //     },
        // )(cli.choices.into_iter());

        let mut server =
            VotallyServer::new("localhost", cli.voting_system, cli.choices.into_iter());

        server.answer_many(1);

        println!("Winner: {}", server.result());

        // read_vote(&mut vote);

        // println!("The winner is {}", vote.result());
    } else {
        println!("I'm a client.");

        let mut client = VotallyClient::new("localhost");

        let info = client.get_info();

        println!("Choices are: ");

        let mut info_iter = info.iter();
        let last_info = info_iter.next_back().unwrap();
        for i in info_iter {
            print!("{}, ", i);
        }
        println!("{}", last_info);

        let mut choice = String::new();
        {
            let mut stdin = stdin().lock();

            while !info.contains(&choice) {
                println!("Enter your choice:");
                stdin.read_line(&mut choice).unwrap();
                choice = choice.to_string().trim().to_owned();
            }
        }

        client.send_vote(&choice);
    }
}

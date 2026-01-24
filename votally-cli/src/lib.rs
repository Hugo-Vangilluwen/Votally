use std::io;

use libvotally::voting_system::*;

// pub struct ConfigServer {
//     pub name: String,
// }

// impl ConfigServer {
//     pub fn build(mut args: impl Iterator<Item = String>) -> Result<(ConfigServer, VotingSystem), &'static str> {
//         // program's name
//         args.next();
//
//         let server_name = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a server name"),
//         };
//
//         let vote = match args.next() {
//             Some(arg) => find_voting_system(&arg),
//             None => return Err("Didn't get a voting system name"),
//         };
//
//         let mut choices: Vec<String> = vec![];
//
//         for c in args {
//             choices.push(c);
//         }
//
//         Ok((ConfigServer{name: server_name}, vote(choices)))
//     }
// }

/// Announce in the console the different choices
fn annouce_choices<VS: VotingSystem>(vote: &VS) {
    let mut choices = vote.get_info().get_choices();
    print!("Different choices are {}", choices.next().unwrap());
    for c in choices {
        print!(", {}", c);
    }
    println!();
    println!("Enter your vote: ");
}

/// Read a vote
pub fn read_vote<VS: VotingSystem>(vote: &mut VS) {
    annouce_choices(vote);

    match vote.get_info().get_ballot_form() {
        BallotForm::Uninominal => {
            let mut ballot = String::new();

            io::stdin()
                .read_line(&mut ballot)
                .expect("Failed to read line");

            match vote.vote(ballot.trim()) {
                Ok(_) => println!("Vote cast!"),
                Err(e) => eprintln!("{}", e),
            }
        }
    }
}

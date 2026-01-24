use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, watch};

use crate::voting_system::{VotingSystem, find_voting_system};

async fn answer_votally_client(
    mut socket: TcpStream,
    mut end_accept_voter_rx: watch::Receiver<()>,
    ballots_tx: mpsc::Sender<String>,
    choices: String,
) -> io::Result<()> {
    socket.write_all(choices.as_bytes()).await?;

    end_accept_voter_rx.changed().await.unwrap();

    let mut reader = BufReader::new(socket);
    let mut ballot = String::new();
    reader.read_line(&mut ballot).await?;

    ballots_tx.send(ballot).await.unwrap();

    Ok(())
}

pub struct VotallyServer {
    end_accept_voter_tx: watch::Sender<()>,
}

impl VotallyServer {
    pub const PORT: &str = "50001";

    pub async fn new<T>(address: &str, name_vote: String, choices: Vec<String>) -> Self
    where
        T: Iterator<Item = String> + Copy + 'static,
    {
        let address = address.to_owned();
        let (end_accept_voter_tx, mut end_accept_voter_rx) = watch::channel(());
        let (ballots_tx, mut ballots_rx) = mpsc::channel(100);

        let response_choices = choices
            .iter()
            .fold(String::new(), |acc, c| acc + c.as_str() + ",");

        // accept voter
        tokio::spawn(async move {
            let listener_tcp = TcpListener::bind(address + ":" + Self::PORT).await.unwrap();

            let end_rx_clone = end_accept_voter_rx.clone();
            tokio::select! {
                _ = async {
                    loop {
                        let (socket, _) = listener_tcp.accept().await?;

                        tokio::spawn(answer_votally_client(socket, end_rx_clone.clone(), ballots_tx.clone(), response_choices.clone()));
                    }

                    Ok::<(), io::Error>(())
                } => {}
                _ = end_accept_voter_rx.changed() => {}
            }
        });

        // make the poll
        // let vote = Arc::new(Mutex::new((find_voting_system(&name_vote[..]).unwrap())(
        // choices.iter().map(|s| s.to_string()),
        // )));

        tokio::spawn(async move {
            let choices = choices.iter().map(|s| s.to_string()).collect();

            let mut vote = find_voting_system(&name_vote[..], choices).unwrap();

            loop {
                let message_vote = ballots_rx.recv().await.unwrap();

                // remove the newline at the end
                let mut message_vote = message_vote.chars();
                message_vote.next_back();
                let message_vote = message_vote.as_str();

                vote.vote(message_vote)
                    .unwrap_or_else(|err| eprintln!("{}", err));

                break;
            }
        });

        VotallyServer {
            end_accept_voter_tx,
        }
    }

    pub async fn start_ballot(self) -> Result<(), watch::error::SendError<()>> {
        self.end_accept_voter_tx.send(())
    }
}

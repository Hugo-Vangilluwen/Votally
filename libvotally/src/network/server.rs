use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot, watch},
    task::JoinHandle,
};

use crate::voting_system::{VotingSystem, find_voting_system};

/// Answer to one votally client
/// Give information then receive ballot
async fn answer_votally_client(
    mut socket: TcpStream,
    mut end_accept_voter_rx: watch::Receiver<()>,
    ballots_tx: mpsc::Sender<String>,
    choices: String,
    mut result_rx: watch::Receiver<String>
) -> io::Result<()> {
    let (socket_rd, mut socket_wr) = socket.split();

    socket_wr.write_all(choices.as_bytes()).await?;

    // begin accept ballot
    end_accept_voter_rx.changed().await.unwrap();
    socket_wr.write_all("\n".as_bytes()).await?;

    let mut reader = BufReader::new(socket_rd);
    let mut ballot = String::new();
    reader.read_line(&mut ballot).await?;

    ballots_tx.send(ballot).await.unwrap();

    // wait the result
    result_rx.changed().await.unwrap();

    let r = result_rx.borrow().clone();
    let r = r + "\n";
    socket_wr.write_all(r.as_bytes()).await?;

    Ok(())
}

pub struct VotallyServer {
    end_accept_voter_tx: watch::Sender<()>,
    vote_handle: Option<JoinHandle<String>>,
    end_accept_ballot_tx: Option<oneshot::Sender<()>>,
    vote_result: Option<String>,
    result_tx: watch::Sender<String>
}

impl VotallyServer {
    pub const PORT: &str = "50001";

    /// Create a new VotallyServer
    /// Initialise process accepting client's connection
    pub async fn new(address: &str, name_vote: String, choices: Vec<String>) -> Self {
        let address = address.to_owned();
        let (end_accept_voter_tx, mut end_accept_voter_rx) = watch::channel(());
        let (ballots_tx, mut ballots_rx) = mpsc::channel(100);
        let (end_accept_ballot_tx, end_accept_ballot_rx) = oneshot::channel();
        let (result_tx, result_rx) = watch::channel(String::new());


        let response_choices = choices
            .iter()
            .fold(String::new(), |acc, c| acc + c.as_str() + ",");
        let response_choices = response_choices.to_owned() + "\n";

        // accept voter
        tokio::spawn(async move {
            let listener_tcp = TcpListener::bind(address + ":" + Self::PORT).await.unwrap();

            let end_rx_clone = end_accept_voter_rx.clone();
            tokio::select! {
            _ = async {
                loop {
                    match listener_tcp.accept().await {
                    Ok((socket, _)) => {
                        tokio::spawn(answer_votally_client(
                            socket,
                            end_rx_clone.clone(),
                            ballots_tx.clone(),
                            response_choices.clone(),
                                                           result_rx.clone()
                        ));
                    },
                    Err(_) => {}
                    }
                }
            } => {}
            _ = end_accept_voter_rx.changed() => {}
            }
        });

        // make the poll
        let vote_handle = tokio::spawn(async move {
            let choices = choices.iter().map(|s| s.to_string()).collect();

            let mut vote = find_voting_system(&name_vote[..], choices).unwrap();

            tokio::select! {
            _ = async {
                loop {
                    match ballots_rx.recv().await {
                        Some(message_vote) => {

                    // remove the newline at the end
                    let mut message_vote = message_vote.chars();
                    message_vote.next_back();
                    let message_vote = message_vote.as_str();

                    vote.vote(message_vote)
                        .unwrap_or_else(|err| eprintln!("{}", err));
                        },
                        None => {
                            break
                        }
                    }
                }
            }=> {},
            _ = end_accept_ballot_rx => {}
            };

            vote.result()
        });

        VotallyServer {
            end_accept_voter_tx,
            vote_handle: Some(vote_handle),
            end_accept_ballot_tx: Some(end_accept_ballot_tx),
            vote_result: None,
            result_tx
        }
    }

    /// End accepting new connection and start the poll
    pub async fn start_ballot(&self) -> Result<(), watch::error::SendError<()>> {
        self.end_accept_voter_tx.send(())
    }

    /// End the poll
    pub async fn end_poll(&mut self) {
        match self.end_accept_ballot_tx.take() {
            Some(s) => {
                let _ = s.send(()); // return Err if ballots_rx is yet closed
                self.end_accept_ballot_tx = None;
            }
            None => {}
        }
    }

    pub async fn calculate_result(&mut self) {
        match self.vote_handle.take() {
            Some(v) => self.vote_result = v.await.ok(),
            None => {}
        }
        self.vote_handle = None;

        self.result_tx.send(self.vote_result.clone().unwrap()).unwrap();
        self.result_tx.closed().await;
    }


    pub fn result(&self) -> String {
        self.vote_result.clone().unwrap()
    }
}

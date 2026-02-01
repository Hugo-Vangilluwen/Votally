use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::voting_system::SingleBallot;

use crate::{network::server::VotallyServer, voting_system::MinimalVotingSystemInfo};

pub struct VotallyClient {
    stream: TcpStream,
}

impl VotallyClient {
    /// Create a new VotalClient
    pub async fn new(address: &str) -> Self {
        let stream = TcpStream::connect(address.to_owned() + ":" + VotallyServer::PORT)
            .await
            .unwrap();

        Self { stream }
    }

    /// Write message in TcpStream
    async fn write_stream(&mut self, message: String) {
        let message = format!("{message}\n");

        self.stream.write_all(message.as_bytes()).await.unwrap();
    }

    /// Read one line of TcpStream
    async fn read_stream(&mut self) -> String {
        let mut buffer = String::new();
        let mut reader = BufReader::new(&mut self.stream);
        reader.read_line(&mut buffer).await.unwrap();

        buffer
    }

    /// Get all the information from server
    // Normaly return VotingSystemInfo
    pub async fn get_info(&mut self) -> MinimalVotingSystemInfo {
        ron::de::from_str(&(self.read_stream().await)).unwrap()
    }

    /// Send the vote to the server
    pub async fn send_vote(&mut self, ballot: &SingleBallot) {
        let _ = self.read_stream().await; // expect a blank line

        let ser_singleballot = ron::ser::to_string(&ballot).unwrap() + "\n";
        self.write_stream(ser_singleballot).await;
    }

    /// Get the result
    pub async fn result(&mut self) -> String {
        let res = self.read_stream().await;

        // remove the newline at the end
        let mut res = res.chars();
        res.next_back();
        res.as_str().to_owned()
    }
}

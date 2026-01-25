use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

// use crate::voting_system::VotingSystemInfo;

use crate::network::server::VotallyServer;

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
    async fn write_stream(&mut self, message: &str) {
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
    pub async fn get_info(&mut self) -> Vec<String> {
        println!("Client started !");

        let info_iter = self.read_stream().await;
        let mut info_iter = info_iter.split(',');
        info_iter.next_back();

        info_iter.map(|s| String::from(s.trim())).collect()
    }

    /// Send the vote to the server
    pub async fn send_vote(&mut self, vote: &str) {
        let _ = self.read_stream().await; // expect a blank line

        self.write_stream(vote).await;
    }
}

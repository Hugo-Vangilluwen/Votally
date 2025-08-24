// use std::io::Read;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

// use crate::voting_system::VotingSystemInfo;

use crate::network::server::VotallyServer;

pub struct VotallyClient {
    stream: TcpStream,
}

impl VotallyClient {
    /// Create a new VotalClient
    pub fn new(address: &str) -> Self {
        let stream = TcpStream::connect(address.to_owned() + ":" + VotallyServer::PORT).unwrap();

        Self { stream }
    }

    /// Get all the information from server
    // Normaly return VotingSystemInfo
    pub fn get_info(&mut self) -> String {
        let message = "INFO\n";

        self.stream.write_all(message.as_bytes()).unwrap();

        println!("Send !");

        let mut buffer = String::new();
        let reader = self.stream.try_clone().unwrap();
        let mut reader = BufReader::new(reader);
        reader.read_line(&mut buffer).unwrap();

        buffer
    }

    /// Send the vote to the server
    pub fn send_vote(&mut self, vote: &str) {}
}

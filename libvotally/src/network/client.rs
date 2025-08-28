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

    fn write_stream(&mut self, message: &str) {
        let message = format!("{message}\n");

        self.stream.write_all(message.as_bytes()).unwrap();
    }

    fn read_stream(&mut self) -> String {
        let mut buffer = String::new();
        let reader = self.stream.try_clone().unwrap();
        let mut reader = BufReader::new(reader);
        reader.read_line(&mut buffer).unwrap();

        buffer
    }

    /// Get all the information from server
    // Normaly return VotingSystemInfo
    pub fn get_info(&mut self) -> Vec<String> {
        self.write_stream("INFO");

        println!("Send !");

        let info_iter = self.read_stream();
        let mut info_iter = info_iter.split(',');
        info_iter.next_back();

        info_iter.map(|s| String::from(s.trim())).collect()
    }

    /// Send the vote to the server
    pub fn send_vote(&mut self, vote: &str) {
        self.write_stream(vote);
    }
}

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread,
};

use crate::voting_system::VotingSystem;

pub struct VotallyServer {
    address: String,
    vote: VotingSystem,
    listener: Option<TcpListener>,
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Mutex<TcpStream>>>,
}

impl VotallyServer {
    pub const PORT: &str = "50001";
    const MAX_CONNECTION: usize = 4;

    /// Create a new VotalServer
    pub fn new(address: &str, vote: VotingSystem) -> Self {
        let listener = TcpListener::bind(address.to_owned() + ":" + Self::PORT).unwrap();

        let mut workers = Vec::with_capacity(Self::MAX_CONNECTION);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..Self::MAX_CONNECTION {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                vote.get_choices().map(|s| s.to_string()).collect(),
            ))
        }

        Self {
            address: String::from(address),
            vote,
            listener: Some(listener),
            workers,
            sender: Some(sender),
        }
    }

    pub fn answer_many(&self, n: usize) {
        for stream in self.listener.as_ref().unwrap().incoming().take(n) {
            self.answer(stream.unwrap());
        }
    }

    pub fn answer(&self, stream: TcpStream) {
        let mutex_stream = Mutex::new(stream);

        self.sender.as_ref().unwrap().send(mutex_stream).unwrap();
    }
}

impl Drop for VotallyServer {
    fn drop(&mut self) {
        drop(self.listener.take());
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            // println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    // id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Mutex<TcpStream>>>>,
        choices: Vec<String>,
    ) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(stream) => {
                        let response = format!("{:?}", choices);

                        stream
                            .lock()
                            .unwrap()
                            .write_all(response.as_bytes())
                            .unwrap();

                        let mut buffer = String::new();
                        let reader = stream.lock().unwrap().try_clone().unwrap();
                        let mut reader = BufReader::new(reader);
                        reader.read_line(&mut buffer).unwrap();
                        print!("Received: {}", buffer);

                        println!("Connnection ended, id:{id}");
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { thread } // id,
    }
}

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::voting_system::{find_voting_system, VotingSystem};

pub struct VotallyServer {
    address: String,
    // vote: Arc<Mutex<VotingSystem>>,
    // vote: VotingSystem,
    listener: Option<TcpListener>,
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Mutex<TcpStream>>>,
    thread_vote: Option<JoinHandle<String>>,
}

impl VotallyServer {
    pub const PORT: &str = "50001";
    const MAX_CONNECTION: usize = 4;

    /// Create a new VotalServer
    pub fn new<T>(address: &str, name_vote: String, choices: T)
    -> Self
    where
        T: Iterator<Item = String>,
    {
        let listener = TcpListener::bind(address.to_owned() + ":" + Self::PORT).unwrap();

        let mut workers = Vec::with_capacity(Self::MAX_CONNECTION);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let (sender_vote, receiver_vote) = mpsc::channel();

        // let choices = vote.get_choices().map(|s| s.to_string()).collect();
        let choices_worker: Vec<String> = choices.collect();

        for id in 0..Self::MAX_CONNECTION {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                Mutex::new(sender_vote.clone()),
                choices_worker.clone(),
            ))
        }

        // let choices: Vec<String> = choices.map(|s| s.to_string()).collect();
        let choices: Vec<String> = choices_worker;

        let thread_vote = thread::spawn(move || {
            let mut vote = (find_voting_system(&name_vote[..]).unwrap())(choices.iter().map(|s| s.to_string()));

            loop {
                let message_vote = receiver_vote.recv().unwrap();

                // remove the newline at the end
                let mut message_vote = message_vote.chars();
                message_vote.next_back();
                let message_vote = message_vote.as_str();

                vote.vote(message_vote);

                break;
            }

            vote.result()
        });

        Self {
            address: String::from(address),
            // vote,
            listener: Some(listener),
            workers,
            sender: Some(sender),
            thread_vote: Some(thread_vote),
        }
    }

    pub fn answer(&self, stream: TcpStream) {
        let mutex_stream = Mutex::new(stream);

        self.sender.as_ref().unwrap().send(mutex_stream).unwrap();
    }

    pub fn answer_many(&self, n: usize) {
        for stream in self.listener.as_ref().unwrap().incoming().take(n) {
            self.answer(stream.unwrap());
        }
    }

    pub fn result(&mut self) -> String {
        // self.vote.lock().unwrap().result()
        // String::from("Toto")
        self.thread_vote.take().unwrap().join().unwrap()
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
        sender_vote: Mutex<mpsc::Sender<String>>,
        choices: Vec<String>,
    ) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(stream) => {
                        let response = choices.iter().fold(String::new(), |acc, c| acc + c + ",");

                        stream
                            .lock()
                            .unwrap()
                            .write_all(response.as_bytes())
                            .unwrap();

                        let mut buffer = String::new();
                        let reader = stream.lock().unwrap().try_clone().unwrap();
                        let mut reader = BufReader::new(reader);
                        reader.read_line(&mut buffer).unwrap();

                        sender_vote.lock().unwrap().send(buffer).unwrap();

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

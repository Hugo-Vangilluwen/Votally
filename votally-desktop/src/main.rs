use iced;
use iced::widget::{Column, button, checkbox, container, radio, scrollable, text, text_input};
use iced::{Element, Length, Task};
use std::sync::Arc;
use tokio::sync::Mutex;

use libvotally::network::VotallyClient;
use libvotally::voting_system::{BallotForm, MinimalVotingSystemInfo, SingleBallot};

#[derive(Clone, Debug)]
enum Message {
    ChangeServerIP(String),
    SubmitServerIP,
    SetClient(Arc<VotallyClient>),
    GetInfo(MinimalVotingSystemInfo),
    ChangeBallotUninominal(usize),
    ChangeBallotApproved(String, bool),
    SubmitBallot,
    WaitWinner,
    ShowWinner(String),
}

#[derive(Default)]
struct VotallyApp {
    server_ip: String,
    client: Option<Arc<Mutex<VotallyClient>>>,
    info: Option<MinimalVotingSystemInfo>,
    ballot: Option<SingleBallot>,
    sending_vote: bool,
    winner: Option<String>,
}

fn container_centered<'a, W>(widget: W) -> iced::widget::Container<'a, Message>
where
    W: Into<Element<'a, Message>>,
{
    container(widget).center(Length::Fill).padding(10)
}

fn view_choices<'a>(
    choices: Vec<String>,
    ballot_form: BallotForm,
    ballot: &'a Option<SingleBallot>,
) -> Vec<Element<'a, Message>> {
    match ballot_form {
        BallotForm::Uninominal => choices
            .into_iter()
            .enumerate()
            .map(move |(k, choice)| {
                let selected_choice = ballot.clone().map(|b| {
                    k + if b == SingleBallot::Uninominal(choice.clone()) {
                        0
                    } else {
                        1
                    }
                });

                radio(choice, k, selected_choice, Message::ChangeBallotUninominal).into()
            })
            .collect(),
        BallotForm::Approved => {
            let approved_choices = ballot
                .clone()
                .map(|b| b.approved_choices())
                .unwrap_or_default();
            choices
                .into_iter()
                .map(move |choice| {
                    checkbox(approved_choices.contains(&choice))
                        .label(choice.clone())
                        .on_toggle(move |checked| {
                            Message::ChangeBallotApproved(choice.clone(), checked)
                        })
                        .into()
                })
                .collect()
        }
        _ => unimplemented!(),
    }
}

impl VotallyApp {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangeServerIP(s) => {
                self.server_ip = s;
                Task::none()
            }
            Message::SubmitServerIP => {
                let server_ip = self.server_ip.clone();
                Task::perform(VotallyClient::new(server_ip), |client| {
                    Message::SetClient(Arc::new(client))
                })
            }
            Message::SetClient(c) => {
                self.client = Arc::into_inner(c).map(Mutex::new).map(Arc::new);

                let client_clone = self.client.as_ref().map(Arc::clone);

                Task::perform(
                    async move { client_clone.unwrap().lock().await.get_info().await },
                    |info| Message::GetInfo(info),
                )
            }
            Message::GetInfo(info) => {
                self.info = Some(info);
                Task::none()
            }
            Message::ChangeBallotUninominal(k) => {
                self.ballot = Some(SingleBallot::Uninominal(
                    self.info.as_ref().unwrap().get_choices()[k].clone(),
                ));
                Task::none()
            }
            Message::ChangeBallotApproved(choice, checked) => {
                self.ballot = Some(self.ballot.take().map_or(
                    SingleBallot::Approved(vec![choice.clone()]),
                    |b| {
                        let mut approved_choices = b.approved_choices();
                        if checked {
                            approved_choices.insert(choice);
                        } else {
                            approved_choices.remove(&choice);
                        }
                        SingleBallot::Approved(Vec::from_iter(approved_choices))
                    },
                ));
                Task::none()
            }
            Message::SubmitBallot => {
                let client_clone = self.client.as_ref().map(Arc::clone);
                let ballot_clone = self.ballot.clone();
                self.info = None;
                self.sending_vote = true;

                Task::perform(
                    async move {
                        client_clone
                            .unwrap()
                            .lock()
                            .await
                            .send_vote(&ballot_clone.unwrap())
                            .await
                    },
                    |_| Message::WaitWinner,
                )
            }
            Message::WaitWinner => {
                let client_clone = self.client.as_ref().map(Arc::clone);

                Task::perform(
                    async move { client_clone.unwrap().lock().await.result().await },
                    |winner| Message::ShowWinner(winner),
                )
            }
            Message::ShowWinner(winner) => {
                self.winner = Some(winner);
                self.client = None;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match self.client {
            None => match &self.winner {
                None => container_centered(
                    text_input("Enter server's IP", &self.server_ip)
                        .padding(5)
                        .on_input(Message::ChangeServerIP)
                        .on_submit(Message::SubmitServerIP),
                ),
                Some(w) => container_centered(text(format!("Winner: {}", w))),
            }
            .into(),
            Some(_) => match &self.info {
                None => if self.sending_vote {
                    container_centered(text("Sending ballot ..."))
                } else {
                    container_centered(text("Waiting info"))
                }
                .into(),
                Some(i) => container(
                    Column::from_vec(vec![
                        text(i.get_name()).into(),
                        container(
                            scrollable(Column::from_vec(view_choices(
                                i.get_choices(),
                                i.get_ballot_form(),
                                &self.ballot,
                            )))
                            .spacing(20.),
                        )
                        .max_height(100.)
                        .into(),
                        self.ballot
                            .as_ref()
                            .map(|b| match i.check_ballot(&b) {
                                Ok(()) => button("Vote !").on_press(Message::SubmitBallot).into(),
                                Err(e) => text(format!("{}", e)).into(),
                            })
                            .unwrap_or_else(|| text("No ballot Yet").into()),
                    ])
                    .spacing(10.),
                )
                .center(Length::Fill)
                .padding(10.)
                .into(),
            },
        }
    }
}

fn main() -> iced::Result {
    iced::application(VotallyApp::default, VotallyApp::update, VotallyApp::view)
        .window_size(iced::Size::new(250., 200.))
        .run()
}

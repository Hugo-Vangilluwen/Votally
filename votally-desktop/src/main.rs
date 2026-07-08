use iced;
use iced::widget::{Column, checkbox, container, radio, text, text_input};
use iced::{Element, Task};
use std::iter::{chain, once};
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
}

#[derive(Default)]
struct VotallyApp {
    server_ip: String,
    client: Option<Arc<Mutex<VotallyClient>>>,
    info: Option<MinimalVotingSystemInfo>,
    ballot: Option<SingleBallot>,
}

fn container_centered<'a, W>(widget: W) -> Element<'a, Message>
where
    W: Into<Element<'a, Message>>,
{
    container(widget)
        .center(iced::Length::Fill)
        .padding(10)
        .into()
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
            _ => unimplemented!(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match self.client {
            None => container_centered(
                text_input("Enter server's IP", &self.server_ip)
                    .padding(5)
                    .on_input(Message::ChangeServerIP)
                    .on_submit(Message::SubmitServerIP),
            ),
            Some(_) => match &self.info {
                None => container_centered(text("Waiting info")),
                Some(i) => Column::with_children(chain(
                    once(text(i.get_name()).into()),
                    view_choices(i.get_choices(), i.get_ballot_form(), &self.ballot).into_iter(),
                ))
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

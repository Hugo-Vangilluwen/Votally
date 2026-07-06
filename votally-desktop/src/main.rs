use iced;
use iced::widget::text_input;
use iced::{Element, Task};
use std::sync::Arc;

use tokio::sync::Mutex;

use libvotally::network::VotallyClient;
use libvotally::voting_system::MinimalVotingSystemInfo;

#[derive(Clone, Debug)]
enum Message {
    ChangeServerIP(String),
    SubmitServerIP,
    SetClient(Arc<VotallyClient>),
    GetInfo(MinimalVotingSystemInfo),
    ChangeBallot(String),
    SubmitBallot,
}

#[derive(Default)]
struct VotallyApp {
    server_ip: String,
    client: Option<Arc<Mutex<VotallyClient>>>,
    info: Option<MinimalVotingSystemInfo>,
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
            _ => unimplemented!(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match self.client {
            None => text_input("Enter server's IP", &self.server_ip)
                .on_input(Message::ChangeServerIP)
                .on_submit(Message::SubmitServerIP)
                .into(),
            Some(_) => unimplemented!(),
        }
    }
}

fn main() -> iced::Result {
    iced::application(VotallyApp::default, VotallyApp::update, VotallyApp::view)
        .window_size(iced::Size::new(200., 50.))
        .run()
}

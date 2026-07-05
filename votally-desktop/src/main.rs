use iced;
use iced::widget::text_input;
use iced::{Element, Task};
use std::sync::Arc;

use libvotally::network::VotallyClient;

#[derive(Clone, Debug)]
enum Message {
    ChangeServerIP(String),
    SubmitServerIP,
    SetClient(Arc<VotallyClient>),
}

#[derive(Default)]
struct VotallyApp {
    server_ip: String,
    client: Option<VotallyClient>,
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
                Task::perform(VotallyClient::new(server_ip), |output| {
                    Message::SetClient(Arc::new(output))
                })
            }
            Message::SetClient(c) => {
                self.client = Arc::into_inner(c);
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

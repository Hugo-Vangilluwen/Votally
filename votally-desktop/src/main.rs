use votally_desktop::VotallyApp;

fn main() -> iced::Result {
    iced::application(VotallyApp::default, VotallyApp::update, VotallyApp::view)
        .window_size(iced::Size::new(250., 200.))
        .run()
}

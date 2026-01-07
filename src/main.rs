use iced::{
    executor, Alignment, Application, Command, Element,
    Settings,
    widget::{Text, TextInput, Button, Column, Row},
};

pub fn main() -> iced::Result {
    FormApp::run(Settings::default())
}

enum Page{
    Login,
    Encoder,
}

struct FormApp {

    page: Page,

    input_value: String,
    submitted: Option<String>,

    codec_value: String,
    result_value: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {

    InputChanged(String),
    SubmitPressed,

    CodecChanged(String),
    EncodePressed,
    DecodePressed,
}

impl Application for FormApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self{
                page: Page::Login,

                input_value: String::new(),
                submitted: None,

                codec_value: String::new(),
                result_value: None,
            },
            Command::none(),
        )
    }


    fn title(&self) -> String {
        "Simple form - Iced".into()
    }


    fn update(&mut self, message: Message) -> Command<Message> {
        match message{
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::SubmitPressed => {
                let secretkey = "secr3tk3y";

                if self.input_value == secretkey{
                    self.page = Page::Encoder;
                } else{
                    self.submitted = Some("Bad key!".into());
                }
            }
        

        Message::CodecChanged(v) => {
            self.codec_value = v;
        }

        Message::EncodePressed => {
            let encoded = base64::encode(&self.codec_value);
            self.result_value = Some(encoded.clone());
            self.codec_value = encoded;
        }

        Message::DecodePressed => {
            let decoded = base64::decode(&self.codec_value).ok().and_then(|bytes| String::from_utf8(bytes).ok());
            if let Some(text) = decoded.clone(){
                self.codec_value = text;
            }
            self.result_value = decoded;
        }

        }
        Command::none()
    }



    fn view(&self) -> Element<Message>{
        match self.page{
            Page::Login => self.view_login(),
            Page::Encoder => self.view_encoder(),
        }
    }
}






impl FormApp{
    fn view_login(&self) -> Element<Message> {
        let input = TextInput::new("Enter the secret key", &self.input_value).on_input(Message::InputChanged).padding(10);

        let button = Button::new("Apply")
            .padding(10)
            .on_press(Message::SubmitPressed);

        let mut content = Column::new()
            .spacing(20)
            .padding(20)
            .align_items(Alignment::Center)
            .push(Row::new().spacing(10).push(input).push(button));


        if let Some(msg) = &self.submitted {
            content = content.push(Text::new(msg.clone()));
        }

        content.into()
    }


    fn view_encoder(&self) -> Element<Message> {
        let input = TextInput::new("Text to encode/decode", &self.codec_value).on_input(Message::CodecChanged).padding(10);

        let encode_btn = Button::new("Encode Base64").padding(10).on_press(Message::EncodePressed);

        let decode_btn = Button::new("Decode base64").padding(10).on_press(Message::DecodePressed);

        let mut content = Column::new().padding(20).spacing(20).align_items(Alignment::Center).push(input).push(Row::new().spacing(10).push(encode_btn).push(decode_btn));

        

        content.into()
    }
}



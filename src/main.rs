#![windows_subsystem = "windows"]
use iced::widget::{text_input::TextInput, Column, Container, Row, Text};
use iced::{Alignment, Element, Sandbox, Settings, Theme};

#[derive(Default)]
struct App {
    input1: String,
    input2: String,
    input_to_int: f32,
    error_msg: String,
    tax_percent: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Calculate,
    UpdateTax,
    TextInputChanged1(String),
    TextInputChanged2(String),
}

pub fn tax_amount(x: f32) -> f32 {
    1.0 - (x / 100.0)
}

pub fn tax(x: f32, y: f32) -> f32 {
    x * y
}

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Tax Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Calculate => match self.input1.parse::<f32>() {
                Ok(parsed_value) => {
                    self.input_to_int = parsed_value;
                }
                Err(_) => {
                    self.error_msg = "Invalid Input".to_string();
                    self.input_to_int = 0.0;
                }
            },
            Message::UpdateTax => match self.input2.parse::<f32>() {
                Ok(parsed_value) => {
                    self.tax_percent = tax_amount(parsed_value);
                }
                Err(_) => {
                    self.error_msg = "Invalid Input".to_string();
                    self.tax_percent = 0.0;
                }
            },
            Message::TextInputChanged1(change1) => {
                self.input1 = change1;
                self.error_msg = "".to_string();
            }
            Message::TextInputChanged2(change2) => {
                self.input2 = change2;
                self.error_msg = "".to_string();
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let error_text = Text::new(&self.error_msg);

        let income_prompt = Text::new("Enter Gross Income");
        let input1 = TextInput::new("Enter quantity here...", &self.input1)
            .on_input(Message::TextInputChanged1)
            .on_submit(Message::Calculate)
            .size(20)
            .padding(10)
            .width(250);
        let output1 = Text::new(format!(
            "Your Net Income is: {} Dollars",
            tax(self.input_to_int, self.tax_percent)
        ));
        let col1 = Column::new()
            .push(income_prompt)
            .push(input1)
            .push(output1)
            .align_items(Alignment::Center)
            .spacing(15);

        let tax_prompt = Text::new("Enter Tax Percentage");
        let input2 = TextInput::new("Enter quantity here...", &self.input2)
            .on_input(Message::TextInputChanged2)
            .on_submit(Message::UpdateTax)
            .size(20)
            .padding(10)
            .width(250);
        let output2 = Text::new(format!("Leftover After Tax: {}", self.tax_percent));
        let col2 = Column::new()
            .push(tax_prompt)
            .push(input2)
            .push(output2)
            .align_items(Alignment::Center)
            .spacing(15);

        let row = Row::new().push(col2).push(col1).spacing(10);
        let col3 = Column::new()
            .push(row)
            .push(error_text)
            .align_items(Alignment::Center);

        let container = Container::new(col3)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into();

        container
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

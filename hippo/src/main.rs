use hippo::files::csv_loader;
use hippo::models::format::{Table, Message};

use iced::widget::{button, column, text, Button, Column, Text};
use iced::{Alignment, Command, Element, Length, Settings, Theme, Application, executor};
use native_dialog::FileDialog;
use csv::{Reader, StringRecord};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn main() -> iced::Result {
    DataViewer::run(Settings::default())
}

#[derive(Default)]
pub struct DataViewer {
    data: Option<Table>,
    error: Option<String>,
}

impl Application for DataViewer {

    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("CSV Viewer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadPressed => {
                let result = csv_loader::load_csv_from_dialog();
                match result {
                    Ok(data) => {
                        println!("CSV Loaded Successfully");
                        //TODO: table view 로직 구현하기
                        self.data = Some(data);
                        self.error = None;
                    }
                    Err(error) => {
                        println!("Error loading CSV: {}", error);
                        self.data = None;
                        self.error = Some(error);
                    }
                }
                Command::none()
            } 
            Message::CsvLoaded(_) => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let mut content = Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(Button::new(Text::new("Load CSV")).on_press(Message::LoadPressed));

        content = if let Some(data) = &self.data {
            content.push(Text::new(format!("Loaded Header Count {}", data.header.len())))
        } else if let Some(error) = &self.error {
            content.push(Text::new(format!("Error: {}", error)))
        } else {
            content.push(Text::new("No data loaded"))
        };

        content.into()
    }
}

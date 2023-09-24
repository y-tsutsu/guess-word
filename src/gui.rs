use iced::alignment::Horizontal;
use iced::text_input::TextInput;
use iced::{Application, Column, Command, Element, Length, Text};

use guess_word::*;

#[derive(Debug, Default)]
struct State {
    input: iced::text_input::State,
    input_value: String,
    announce: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Guess,
}

#[derive(Default)]
pub struct GuessWord {
    game: Game,
    state: State,
}

impl Application for GuessWord {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (GuessWord, Command<Self::Message>) {
        (GuessWord::default(), Command::none())
    }

    fn title(&self) -> String {
        "GuessWord".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => {
                self.state.input_value = value;
            }
            Message::Guess => {
                let (status, result) = self.game.guess(&self.state.input_value);
                match status {
                    GameStatus::Won => {
                        self.state.announce = "You Win!".to_string();
                    }
                    GameStatus::Lost => {
                        self.state.announce =
                            format!("You Lost! (answer:{})", self.game.get_answer().unwrap());
                    }
                    GameStatus::InProgress => match result {
                        GuessResult::DuplicateGuess => {
                            self.state.announce = "Duplicate Guess".to_string();
                        }
                        GuessResult::IncorrectLength => {
                            self.state.announce = "Incorrect Length".to_string();
                        }
                        GuessResult::NotInDictionary => {
                            self.state.announce = "Invalid word".to_string();
                        }
                        GuessResult::Valid => {
                            self.state.announce.clear();
                        }
                        _ => (),
                    },
                }
                self.state.input_value.clear();
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let title = Text::new("GuessWord")
            .width(Length::Fill)
            .size(60)
            .color([0.5; 3])
            .horizontal_alignment(Horizontal::Center);

        let announce = Text::new(&self.state.announce)
            .width(Length::Fill)
            .size(20)
            .color([0.0, 0.4, 1.0])
            .horizontal_alignment(Horizontal::Center);

        let input = TextInput::new(
            &mut self.state.input,
            "input word...",
            &self.state.input_value,
            Message::InputChanged,
        )
        .padding(15)
        .size(30)
        .on_submit(Message::Guess);

        let content = Column::new()
            .padding(40)
            .max_width(800)
            .spacing(20)
            .push(title)
            .push(announce)
            .push(input);

        content.into()
    }
}

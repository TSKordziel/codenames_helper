use iced::widget::{
    button, column, container, radio, responsive, row, text, Button, Column, Container,
};
use iced::Length::FillPortion;
use iced::{alignment, border, window, Background, Color, Element, Fill, Font, Size, Theme};

use rand::rng;
use rand::seq::SliceRandom;

const WORD_LIST: &'static str = include_str!("codenames.txt");
const BUTTON_SIZE_FACTOR: f32 = 0.4;
const CORNER_RAD: i32 = 15;

fn read_lines() -> Vec<String> {
    WORD_LIST.lines().map(|word| word.to_string()).collect()
}

fn button_helper(label: &str, message: Message) -> Button<'_, Message> {
    button(label)
        .on_press(message)
        .padding(10)
        .style(|theme: &Theme, status: button::Status| match status {
            button::Status::Active => button::Style {
                background: Some(theme.extended_palette().background.weak.color.into()),
                border: border::rounded(15),
                ..button::Style::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(theme.extended_palette().background.strong.color.into()),
                border: border::rounded(15),
                ..button::Style::default()
            },
            _ => button::Style {
                background: Some(Background::Color(Color::new(0.5, 0.5, 0.5, 1.0))),
                border: border::rounded(15),
                ..button::Style::default()
            },
        })
}

fn main() -> iced::Result {
    iced::application("CODENAMES HELPER", update, view)
        .theme(|_| Theme::Dark)
        .window(window::Settings {
            size: Size {
                width: 1600.0,
                height: 740.0,
            },
            position: window::Position::Centered,
            ..Default::default()
        })
        .run()
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadioSelected((usize, Choice)),
    Start,
    Draw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice {
    Red,
    Blue,
    Neutral,
    Death,
}

#[derive(Default, Clone)]
struct Card {
    text: String,
    index: usize,
    selection: Option<(usize, Choice)>,
}

impl Card {
    fn make_container(&self, max_length: usize) -> Container<'_, Message> {
        container(
            container(
                column![
                    container(responsive(move |size| {
                        container(
                            text(self.text.as_str())
                                .font(Font::MONOSPACE)
                                .size((size.width / max_length as f32) * 1.618)
                                .center(),
                        )
                        .center(Fill)
                        .into()
                    }))
                    .height(FillPortion(2))
                    .width(Fill),
                    container(responsive(|size| {
                        let red = radio(
                            "Red",
                            (self.index, Choice::Red),
                            self.selection,
                            Message::RadioSelected,
                        )
                        .size(size.height * BUTTON_SIZE_FACTOR)
                        .text_size(size.height * BUTTON_SIZE_FACTOR)
                        .spacing(3)
                        .width(FillPortion(4));

                        let blue = radio(
                            "Blue",
                            (self.index, Choice::Blue),
                            self.selection,
                            Message::RadioSelected,
                        )
                        .size(size.height * BUTTON_SIZE_FACTOR)
                        .text_size(size.height * BUTTON_SIZE_FACTOR)
                        .spacing(3)
                        .width(FillPortion(4));

                        let neutral = radio(
                            "Neutral",
                            (self.index, Choice::Neutral),
                            self.selection,
                            Message::RadioSelected,
                        )
                        .size(size.height * BUTTON_SIZE_FACTOR)
                        .text_size(size.height * BUTTON_SIZE_FACTOR)
                        .spacing(3)
                        .width(FillPortion(5));

                        let death = radio(
                            "Death",
                            (self.index, Choice::Death),
                            self.selection,
                            Message::RadioSelected,
                        )
                        .size(size.height * BUTTON_SIZE_FACTOR)
                        .text_size(size.height * BUTTON_SIZE_FACTOR)
                        .spacing(3)
                        .width(FillPortion(4));
                        row![red, blue, neutral, death]
                            .height(Fill)
                            .width(Fill)
                            .padding(5)
                            .into()
                    }))
                    .height(Fill)
                    .width(Fill),
                ]
                .height(Fill)
                .width(Fill),
            )
            .center(Fill)
            .style(|theme: &Theme| match self.selection {
                Some((_index, choice)) => match choice {
                    Choice::Red => container::Style {
                        background: Some(Background::Color(Color::new(1.0, 0.0, 0.0, 1.0))),
                        border: border::rounded(CORNER_RAD),
                        ..container::Style::default()
                    },
                    Choice::Blue => container::Style {
                        background: Some(Background::Color(Color::new(0.0, 0.0, 1.0, 1.0))),
                        border: border::rounded(CORNER_RAD),
                        ..container::Style::default()
                    },
                    Choice::Death => container::Style {
                        background: Some(Background::Color(Color::new(0.0, 0.0, 0.0, 0.5))),
                        border: border::rounded(CORNER_RAD),
                        ..container::Style::default()
                    },
                    Choice::Neutral => container::Style {
                        background: Some(Background::Color(Color::new(0.82, 0.71, 0.55, 1.0))),
                        border: border::rounded(CORNER_RAD),
                        ..container::Style::default()
                    },
                },
                None => container::Style {
                    background: Some(theme.extended_palette().background.weak.color.into()),
                    border: border::rounded(CORNER_RAD),
                    ..container::Style::default()
                },
            }),
        )
        .padding(5)
        .center(Fill)
    }
    fn update(&mut self, word: String, index: usize) {
        self.text = word;
        self.index = index;
        self.selection = None;
    }
}

#[derive(Default)]
struct Grid {
    cards: [Card; 25],
    words: Vec<String>,
    started: bool,
    max_word_len: usize,
}
impl Grid {
    fn generate(&mut self) {
        if self.words.len() < self.cards.len() {
            self.words = read_lines();
            self.shuffle_deck();
        }
        for (index, word) in self.words.drain(0..25).enumerate() {
            self.cards[index].update(word, index);
        }
    }
    fn view(&self) -> Container<'_, Message> {
        match self.started {
            true => {
                let length = self.max_word_len;
                let rows = self.cards.chunks(5).map(|chunk| {
                    row(chunk.iter().map(|card| card.make_container(length).into()))
                        .spacing(25)
                        .align_y(alignment::Vertical::Center)
                        .into()
                });
                let content = Column::from_iter(rows);

                container(
                    content
                        .push(button_helper("New Game!", Message::Draw))
                        .spacing(25)
                        .align_x(alignment::Horizontal::Center),
                )
                .padding(20)
                .center(Fill)
            }
            false => container(button_helper("Start!", Message::Start))
                .padding(20)
                .center(Fill),
        }
    }
    fn shuffle_deck(&mut self) {
        let mut rng = rng();
        self.words.shuffle(&mut rng);
    }
}

fn view(grid: &Grid) -> Element<Message> {
    grid.view().into()
}

fn update(grid: &mut Grid, message: Message) {
    match message {
        Message::RadioSelected((index, choice)) => {
            grid.cards[index].selection = Some((index, choice));
        }
        Message::Start => {
            if let Some(number) = WORD_LIST.lines().map(|word| word.len()).max() {
                grid.max_word_len = number;
            }
            grid.started = true;
            grid.generate();
        }
        Message::Draw => {
            grid.generate();
        }
    }
}

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, ListDirection, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};

use crate::db::models::Word;
use crate::api::get_meaning;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    words: Vec<Word>,
    input: String,
    translation: String
}

impl App {
    pub fn new(words: Vec<Word>) -> Self {
        Self { words, exit: false, counter: 0, input: String::new(), translation: String::new()}
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    async fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event).await
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Backspace =>  {
                self.input.pop();
            }
            KeyCode::Enter => self.update_meaning().await,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    async fn update_meaning(&mut self) {
        match get_meaning(&self.input).await {
            Ok(response) => {
                self.translation = String::new();
                for word in response.words {
                    self.translation.push_str(&word.reading.kanji);
                    self.translation.push_str("\n\n");
                    for sense in word.senses {
                        for gloss in sense.glosses {
                            self.translation.push_str(&gloss.to_string());
                            self.translation.push('\n');
                        }
                    }
                }
            }
            Err(error) => {
                self.translation = error.to_string();
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [title_area, body_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1)
        ])
        .areas(area);

        let [left_area, right_area] = Layout::horizontal([
            Constraint::Percentage(60),
            Constraint::Percentage(40)
        ])
        .areas(body_area);
        
        // title
        Paragraph::new("jp-cli")
            .centered()
            .block(Block::bordered())
            .render(title_area, buf);


        let japanese = self.words
            .iter()
            .map(|word| word.japanese.clone());
        let english = self.words
            .iter()
            .map(|word| word.english.clone());

        
        let block = Block::bordered().title(Line::from("Words").centered());
        let inner = block.inner(right_area);
        block.render(right_area, buf);

        let [jap_area, english_area] = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .flex(Flex::Center)
        .areas(inner);


        List::new(english)
            .render(english_area, buf);
        List::new(japanese)
            .render(jap_area, buf);

        let left_block = Block::bordered().title(Line::from("Search").centered());
        let left_block_inner = left_block.inner(left_area);
        left_block.render(left_area, buf);

        let [input_area, translation_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(left_block_inner);

        Paragraph::new(self.input.as_str())
            .block(Block::bordered().title("Input"))
            .render(input_area, buf);

        Paragraph::new(self.translation.as_str())
            .block(Block::bordered().title("Translation"))
            .wrap(Wrap { trim: false })
            .render(translation_area, buf);


        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
    }
}

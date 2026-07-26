use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, ListDirection, List},
    DefaultTerminal, Frame,
};

use crate::db::models::Word;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    words: Vec<Word>
}

impl App {
    pub fn new(words: Vec<Word>) -> Self {
        Self { words, exit: false, counter: 0}
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
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



        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
    }
}

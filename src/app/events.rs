use ratatui::style::Color;
use std::{io, sync::mpsc};

use crossterm::event::{KeyCode, KeyEventKind};
pub fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(key_event) => tx.send(Event::Key(key_event)).unwrap(),
            _ => {}
        }
    }
}

pub enum Event {
    Key(crossterm::event::KeyEvent),
    Title(String),
}

use crate::app::App;

impl App {
    pub fn handle_events(&mut self, rx: &mpsc::Receiver<Event>) -> io::Result<()> {
        match rx.recv().unwrap() {
            Event::Key(key_event) => self.handle_key_event(key_event)?,
            Event::Title(text) => self.title_text = text,
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        } else if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('c') {
            if self.progress_bar_color == Color::Green {
                self.progress_bar_color = Color::Yellow;
            } else {
                self.progress_bar_color = Color::Green;
            }
        }

        Ok(())
    }
}

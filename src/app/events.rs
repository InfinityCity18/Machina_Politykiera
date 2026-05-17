use crossterm::event::{KeyCode, KeyEventKind};
use std::{error::Error, io, result::Result, sync::mpsc};

pub fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match crossterm::event::read() {
            Ok(crossterm::event::Event::Key(key_event)) => match tx.send(Event::Key(key_event)) {
                Ok(_) => (),
                Err(_) => todo!("handle error bruh"),
            },
            Err(_) => todo!("handle error bruh"),
            _ => (),
        }
    }
}

pub enum Event {
    Key(crossterm::event::KeyEvent),
    Title(String),
    ProcessesRefresh,
}

#[derive(Clone)]
pub enum Focus {
    ProcessListWindow,
    MemoryListWindow,
    PinnedMemoryWindow,
    ValueInputField,
}
use crate::app::App;

impl App<'_> {
    pub fn handle_events(&mut self, rx: &mpsc::Receiver<Event>) -> Result<(), Box<dyn Error>> {
        match rx.recv() {
            Ok(Event::Key(key_event)) => self.handle_key_event(key_event)?,
            Ok(Event::Title(text)) => self.title_text = text,
            Ok(Event::ProcessesRefresh) => self.process_list.update(),
            Err(_) => todo!("handle error bruh"),
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            match (key_event.code, self.focus_window.clone()) {
                (KeyCode::Enter, _) => self.focus_window = Focus::ValueInputField,
                (KeyCode::Esc, _) => self.focus_window = Focus::ProcessListWindow,
                (KeyCode::Char(_), Focus::ValueInputField) => {
                    self.input_field.handle_key_event(key_event)
                }
                (KeyCode::Char('q'), _) => self.exit = true,
                (KeyCode::Char('p'), _) => self.focus_window = Focus::ProcessListWindow,
                (KeyCode::Char('m'), _) => self.focus_window = Focus::MemoryListWindow,
                (KeyCode::Char('n'), _) => todo!(), // next scan
                (KeyCode::Char('f'), _) => todo!(), // first scan
                (KeyCode::Char('o'), _) => self.focus_window = Focus::PinnedMemoryWindow, // override
                (_, _) => match self.focus_window {
                    Focus::ProcessListWindow => self.handle_process_list_key_event(key_event),
                    _ => (),
                },
            }
        }
        Ok(())
    }

    fn handle_process_list_key_event(&mut self, key_event: crossterm::event::KeyEvent) {
        match key_event.code {
            KeyCode::Up => self.process_list.widget_state.select_previous(),
            KeyCode::Down => self.process_list.widget_state.select_next(),
            KeyCode::Char('s') => todo!(), // search
            _ => (),
        }
    }
}

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
    MemoryEditorWindow,
    ValueInputField,
}

use self::Event::*;
use self::Focus::*;
use crate::app::App;

impl App<'_> {
    pub fn handle_events(&mut self, rx: &mpsc::Receiver<Event>) -> Result<(), Box<dyn Error>> {
        match rx.recv() {
            Ok(Key(key_event)) => self.handle_key_event(key_event)?,
            Ok(Title(text)) => self.title_text = text,
            Ok(ProcessesRefresh) => self.process_list.update(),
            Err(_) => todo!("handle error bruh"),
        }
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press {
            match (key_event.code, self.focus_window.clone()) {
                (KeyCode::Esc, _) => self.set_focus(ProcessListWindow),
                (_, ValueInputField) => self.scan_value_field.handle_key_event(key_event),
                (KeyCode::Char('q'), _) => self.exit = true,
                (KeyCode::Char('v'), _) => self.set_focus(ValueInputField),
                (KeyCode::Char('e'), _) => self.set_focus(MemoryEditorWindow),
                (KeyCode::Char('p'), _) => self.set_focus(ProcessListWindow),
                (KeyCode::Char('m'), _) => self.set_focus(MemoryListWindow),
                (KeyCode::Char('t'), _) => self.scan_type_selector.cycle_type(),
                (KeyCode::Char('n'), _) => todo!(), // next scan
                (KeyCode::Char('f'), _) => todo!(), // first scan
                (_, ProcessListWindow) => self.handle_process_list_key_event(key_event),
                _ => (),
            }
        }
        Ok(())
    }

    fn set_focus(&mut self, target: Focus) {
        // actions to perform after losing focus of a window
        match self.focus_window {
            ValueInputField => self.scan_value_field.selected = false,
            _ => (),
        }

        // actions to perform on gaining focus of a window
        match target {
            ValueInputField => self.scan_value_field.selected = true,
            _ => (),
        }

        self.focus_window = target;
    }

    fn handle_process_list_key_event(&mut self, key_event: crossterm::event::KeyEvent) {
        match key_event.code {
            KeyCode::Up => self.process_list.widget_state.select_previous(),
            KeyCode::Down => self.process_list.widget_state.select_next(),
            KeyCode::Enter => self.change_process_to_selected(),
            _ => (),
        }
    }
}

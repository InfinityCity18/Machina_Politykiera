use ratatui::{style::Color, DefaultTerminal, Frame};
use std::{io, sync::mpsc, thread, time::Duration};

pub mod events;
mod process_list;
mod widgets;
use events::Event;

pub struct App {
    pub exit: bool,
    pub progress_bar_color: Color,
    pub title_text: String,
    //selected_process -> Process
    //list_of_scanned_processes -> MemoryScan
    //list_of_pinned_processes -> PinnedProcesses
}

pub fn cycle_title(tx: mpsc::Sender<Event>) {
    let mut text = String::from("Machina Politykiera");
    loop {
        let mut m: Vec<char> = text.chars().collect();
        m.rotate_left(1);
        text = m.into_iter().collect();
        thread::sleep(Duration::from_millis(100));
        tx.send(Event::Title(text.clone())).unwrap();
    }
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rx: mpsc::Receiver<Event>,
    ) -> io::Result<()> {
        while !self.exit {
            let _ = self.handle_events(&rx);
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

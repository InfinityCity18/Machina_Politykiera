use ratatui::{style::Color, DefaultTerminal, Frame};
use std::{io, sync::mpsc, thread, time::Duration};

pub mod events;
mod widgets;
use events::Event;

pub struct App {
    pub exit: bool,
    pub progress_bar_color: Color,
    pub background_progress: f64,
}

pub fn run_background_thread(tx: mpsc::Sender<Event>) {
    let mut progress = 0_f64;
    let increment = 0.01_f64;
    loop {
        thread::sleep(Duration::from_millis(100));
        progress += increment;
        progress = progress.min(1_f64);
        tx.send(Event::Progress(progress)).unwrap();
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

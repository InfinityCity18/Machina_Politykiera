use ratatui::{DefaultTerminal, Frame};
use std::{io, sync::mpsc};

pub mod events;
mod memoryaddress;
mod memoryscanner;
pub mod processlist;
mod scansettings;
pub mod tui;
use events::Event;
use events::Focus;

use crate::app::memoryscanner::MemoryScanner;
use crate::app::processlist::ProcessList;

pub struct App<'a> {
    pub exit: bool,
    pub title_text: String,

    // selected_process: Process  <--- initialization problem, see below
    memory_scanner: MemoryScanner,
    process_list: ProcessList<'a>,
    //selected_process -> Process
    //list_of_scanned_processes -> MemoryScan <-- would get rid of
    //list_of_pinned_processes -> PinnedProcesses
    focus_window: Focus,
}

impl App<'_> {
    pub fn new() -> Self {
        let mut me = Self {
            exit: false,
            title_text: String::from("Machina Politykiera"),
            //selected_process: no idea how to initialize,
            memory_scanner: MemoryScanner::new(),
            process_list: ProcessList::new(),
            focus_window: Focus::ProcessListWindow,
        };

        me.process_list.update();
        me
    }

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

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

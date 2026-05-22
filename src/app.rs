use procfs::process::Process;
use ratatui::{DefaultTerminal, Frame};
use std::{io, sync::mpsc};

pub mod events;
mod inputfield;
mod memoryaddress;
mod memoryeditor;
mod memoryscanner;
pub mod processlist;
mod scansettings;
pub mod tui;

use events::Event;
use events::Focus;

use crate::app::memoryeditor::MemoryEditor;
use crate::app::memoryscanner::MemoryScanner;
use crate::app::processlist::ProcessList;

use inputfield::InputField;

pub struct App<'a> {
    pub exit: bool,
    pub title_text: String,

    selected_process: Option<Process>,
    memory_scanner: MemoryScanner<'a>,
    memory_editor: MemoryEditor<'a>,
    process_list: ProcessList<'a>,
    //selected_process -> Process
    //list_of_scanned_processes -> MemoryScan <-- would get rid of
    //list_of_pinned_processes -> PinnedProcesses
    focus_window: Focus,
    scan_value_field: InputField,
}

impl App<'_> {
    pub fn new() -> Self {
        let mut me = Self {
            exit: false,
            title_text: String::from(""), // this doesn't matter. Gets set by running title cycling
            selected_process: None,
            memory_scanner: MemoryScanner::new(),
            memory_editor: MemoryEditor::new(),
            process_list: ProcessList::new(),
            focus_window: Focus::ProcessListWindow,
            scan_value_field: InputField::new(" [V]alue ".to_string()),
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

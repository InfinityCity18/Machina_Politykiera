use procfs::process::Process;
use ratatui::{DefaultTerminal, Frame};
use std::{io, rc::Rc, sync::mpsc};

pub mod events;
mod inputfield;
mod logger;
mod memoryaddress;
mod memoryeditor;
mod memoryscanner;
pub mod processlist;
mod scansettings;
pub mod tui;
mod typeselector;

use events::Event;
use events::Focus;

use crate::app::logger::Logger;
use crate::app::memoryeditor::MemoryEditor;
use crate::app::memoryscanner::MemoryScanner;
use crate::app::processlist::ProcessList;
use crate::app::scansettings::ScanSettings;
use crate::app::scansettings::ScanValue;
use crate::app::typeselector::TypeSelector;

use inputfield::InputField;

pub struct App<'a> {
    pub exit: bool,
    pub title_text: String,

    selected_process: Option<Rc<Process>>,

    logger: Logger,
    memory_scanner: MemoryScanner<'a>,
    memory_editor: MemoryEditor<'a>,
    process_list: ProcessList<'a>,
    scan_value_field: InputField,
    scan_type_selector: TypeSelector,

    focus_window: Focus,
}

impl App<'_> {
    pub fn new() -> Self {
        let mut me = Self {
            exit: false,
            title_text: String::from(""), // this doesn't matter. Gets set by running title cycling
            selected_process: None,

            logger: Logger::new(),
            memory_scanner: MemoryScanner::new(),
            memory_editor: MemoryEditor::new(),
            process_list: ProcessList::new(),
            focus_window: Focus::ProcessListWindow,
            scan_value_field: InputField::new(" [V]alue ".to_string()),
            scan_type_selector: TypeSelector::new(),
        };

        me.process_list.update();
        me.logger.log("we are many".to_string());
        me.logger.log("we are many".to_string());
        me.logger.log("we are many".to_string());
        me.logger.log("we are many".to_string());
        me.logger.log("we are many".to_string());
        me.logger.log("we are many".to_string());
        me.logger.log("we are many".to_string());
        me.logger.log("we are many".to_string());
        me.logger.log("we are many".to_string());
        me.logger.log("we are many".to_string());
        me.logger.log("meow log 1".to_string());
        me.logger.log("woof log 2".to_string());
        me.logger.log("very long meowsers log 3 that probably takes up more than one line but i can never really be fully sure".to_string());
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

    pub fn change_process_to_selected(&mut self) {
        match self.process_list.get_selected() {
            None => (),
            Some(proc) => {
                if proc.is_alive() {
                    self.selected_process = Some(proc)
                }
            }
        }
    }

    pub fn perform_scan(&mut self, first: bool) {
        // error handling is to be improved. Probably by logging
        if !match self.selected_process.clone() {
            Some(proc) => proc.is_alive(),
            None => false,
        } {
            return;
        }

        let value = match ScanValue::from_user_input(
            self.scan_value_field.input.clone(),
            self.scan_type_selector.selected,
        ) {
            Ok(val) => Some(val),
            Err(_) => None,
        };

        if value == None {
            return;
        }

        let settings = ScanSettings::new(self.selected_process.clone().unwrap(), value.unwrap());

        match if first {
            self.memory_scanner.first_scan(settings)
        } else {
            self.memory_scanner.next_scan(settings)
        } {
            Ok(()) => (),
            Err(_) => (), // should be handled probs, or logged
        };
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

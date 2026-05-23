use log::{error, info};
use procfs::process::Process;
use ratatui::{DefaultTerminal, Frame};
use std::{io, rc::Rc, sync::mpsc};

pub mod events;
mod inputfield;
pub mod logger;
mod memoryaddress;
mod memoryeditor;
mod memoryscanner;
pub mod processlist;
mod scansettings;
pub mod tui;
mod typeselector;

use events::Event;
use events::Focus;

use crate::app::logger::{Logger, LogsMutex};
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

    new_value_field: InputField,

    focus_window: Focus,
}

impl App<'_> {
    pub fn new(logs_mutex: LogsMutex) -> Self {
        let mut me = Self {
            exit: false,
            title_text: String::from(""), // this doesn't matter. Gets set by running title cycling
            selected_process: None,

            logger: Logger::new(logs_mutex),
            memory_scanner: MemoryScanner::new(),
            memory_editor: MemoryEditor::new(),
            process_list: ProcessList::new(),
            focus_window: Focus::ProcessListWindow,
            scan_value_field: InputField::new(" [V]alue ".to_string()),
            scan_type_selector: TypeSelector::new(),

            new_value_field: InputField::new(" New Value ".to_string()),
        };

        me.process_list.update();
        // me.logger.log("we are many1".to_string());
        // me.logger.log("we are many2".to_string());
        // me.logger.log("we are many3".to_string());
        // me.logger.log("we are many4".to_string());
        // me.logger.log("we are many5".to_string());
        // me.logger.log("we are many6".to_string());
        // me.logger.log("we are many7".to_string());
        // me.logger.log("we are many8".to_string());
        // me.logger.log("we are many9".to_string());
        // me.logger.log("meow log 1".to_string());
        // me.logger.log("woof log 2".to_string());
        // me.logger.log("very long meowsers log 3 that probably takes up more than one line but i can never really be fully sure".to_string());
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
            error!("Couldn't scan - selected process is dead or invalid");
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
            error!("Couldn't scan - couldn't parse input value to the specified type");
            return;
        }

        let settings = ScanSettings::new(self.selected_process.clone().unwrap(), value.unwrap());

        match if first {
            self.memory_scanner.first_scan(settings)
        } else {
            self.memory_scanner.next_scan(settings)
        } {
            Ok(()) => info!(
                "Performed {} scan succesfully",
                if first { "first" } else { "next" }
            ),
            Err(e) => error!(
                "Couldn't perform {} scan. Error: {}",
                if first { "first" } else { "next" },
                e
            ),
        };
    }

    // replace wall of returns with logging
    pub fn edit_selected_value(&mut self) {
        let addr = match self.memory_editor.get_selected() {
            Some(ad) => ad,
            None => return,
        };

        let val =
            match ScanValue::from_user_input(self.new_value_field.input.clone(), addr.val_type) {
                Ok(v) => v,
                Err(_) => return,
            };

        match addr.set_value(val) {
            Ok(()) => return,
            Err(_) => return,
        };
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

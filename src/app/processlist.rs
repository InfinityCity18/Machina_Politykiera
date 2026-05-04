use crate::app::Event;
use procfs::process::{self, Process};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget, Widget},
};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// A struct for convenient holding of processes list
pub struct ProcessList {
    processes: Vec<Process>,
    pub widget_state: ListState,
}

pub fn update_processes_periodically(tx: mpsc::Sender<Event>, ms: u64) {
    loop {
        tx.send(Event::ProcessesRefresh).unwrap();
        thread::sleep(Duration::from_millis(ms));
    }
}

impl ProcessList {
    /// Creates new, empty instance of `ProcessList`
    pub fn new() -> Self {
        let mut me = Self {
            processes: vec![],
            widget_state: ListState::default(),
        };
        me.widget_state.select_last();
        me
    }

    /// Updates list of processes
    pub fn update(&mut self) {
        match process::all_processes() {
            Ok(proc_it) => self.processes = proc_it.filter_map(|res| res.ok()).collect(),
            Err(err) => {
                unimplemented!("Updating list of processes error handling is unimplemented")
            }
        }
    }

    /// Returns a reference to list of processes
    pub fn get(&self) -> &Vec<Process> {
        &self.processes
    }
}

impl Widget for &mut ProcessList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .processes
            .iter()
            .map(|p| {
                let name = p
                    .stat()
                    .map(|s| s.comm)
                    .unwrap_or_else(|_| "Couldn't parse".to_string());
                ListItem::new(format!("{:<8}{}", p.pid, name))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Process List ")
                    .borders(Borders::ALL),
            )
            .highlight_symbol(">> ");

        StatefulWidget::render(list, area, buf, &mut self.widget_state);
    }
}

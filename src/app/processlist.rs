use procfs::process::{self, Process};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{List, ListItem, ListState, StatefulWidget, Widget},
};
use std::rc::Rc;

/// A struct for convenient holding of processes list
pub struct ProcessList<'a> {
    processes: Vec<Rc<Process>>,

    pub widget_state: ListState,
    list_items: Vec<ListItem<'a>>,
}

impl ProcessList<'_> {
    /// Creates new, empty instance of `ProcessList`
    pub fn new() -> Self {
        let mut me = Self {
            processes: vec![],
            widget_state: ListState::default(),
            list_items: vec![],
        };
        me.widget_state.select_last();
        me
    }

    /// Updates list of processes
    pub fn update(&mut self) {
        match process::all_processes() {
            Ok(proc_it) => {
                self.processes.clear();
                self.processes = proc_it
                    .filter_map(|res| res.inspect_err(|e| log::warn!("problem with opening process")).ok().map(|p| Rc::new(p)))
                    .collect();
                self.list_items = self
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
            }
            Err(_) => {
                // nothing ever happens
            }
        };
    }

    /// Returns a reference to list of processes
    pub fn get(&self) -> &Vec<Rc<Process>> {
        &self.processes
    }

    /// Returns the currently selected process
    pub fn get_selected(&self) -> Option<Rc<Process>> {
        match self.widget_state.selected() {
            Some(i) => Some(self.processes[i].clone()),
            None => None,
        }
    }
}

impl Widget for &mut ProcessList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let list = List::new(self.list_items.clone()).highlight_symbol(">> ");

        StatefulWidget::render(list, area, buf, &mut self.widget_state);
    }
}

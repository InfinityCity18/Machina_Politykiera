use log::info;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::{
    Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
};

use crate::app::scansettings::ScanValue;
use crate::app::{
    inputfield::InputField, memoryaddress::MemoryAddress, scansettings::ScanSettings,
};

pub struct MemoryEditor<'a> {
    pinned_addresses: Vec<(MemoryAddress, Option<ScanValue>)>,

    pub widget_state: ListState,
    list_items: Vec<ListItem<'a>>,
}

impl MemoryEditor<'_> {
    /// Creates new instance of `MemoryEditor`
    pub fn new() -> Self {
        Self {
            pinned_addresses: vec![],
            widget_state: ListState::default(),
            list_items: vec![],
        }
    }

    pub fn update(&mut self) {
        if self.pinned_addresses.len() > 0 && self.widget_state.selected() == None {
            self.widget_state.select_first();
        }
        self.pinned_addresses = self
            .pinned_addresses
            .iter()
            .map(|(addr, _)| {
                (
                    addr.clone(),
                    match addr.read_value() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    },
                )
            })
            .collect();

        self.list_items = self
            .pinned_addresses
            .iter()
            .clone()
            .map(|(addr, val)| {
                ListItem::new(format!(
                    "{}{}",
                    addr.to_string(),
                    match val {
                        None => "None".to_string(),
                        Some(v) => v.to_string(),
                    }
                ))
            })
            .collect()
    }

    pub fn pin_address(&mut self, address: MemoryAddress) {
        if self.pinned_addresses.iter().find(|av| av.0 == address) == None {
            self.pinned_addresses.push((address, None));
        }
        self.update();
    }
    pub fn get_selected(&self) -> Option<MemoryAddress> {
        match self.widget_state.selected() {
            Some(i) => Some(self.pinned_addresses[i].0.clone()),
            None => None,
        }
    }

    pub fn unpin_selected(&mut self) {
        match self.widget_state.selected() {
            Some(i) => {
                self.pinned_addresses.remove(i);
            }
            None => info!("Nothing to unpin"),
        };
        self.update();
    }
}

impl Widget for &mut MemoryEditor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let list = List::new(self.list_items.clone()).highlight_symbol(">> ");

        StatefulWidget::render(list, area, buf, &mut self.widget_state);
    }
}

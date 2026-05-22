use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::{
    Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
};

use crate::app::{
    inputfield::InputField, memoryaddress::MemoryAddress, scansettings::ScanSettings,
};

pub struct MemoryEditor<'a> {
    pinned_addresses: Vec<MemoryAddress>,

    pub list_selected: bool,
    pub widget_state: ListState,
    list_items: Vec<ListItem<'a>>,

    pub value_edit_field: InputField,
}

impl MemoryEditor<'_> {
    /// Creates new instance of `MemoryEditor`
    pub fn new() -> Self {
        Self {
            pinned_addresses: vec![],
            widget_state: ListState::default(),
            list_selected: false,
            list_items: vec![],
            value_edit_field: InputField::new(" Edited Value ".to_string()),
        }
    }

    pub fn pin_address(&mut self, address: MemoryAddress) {
        if !self.pinned_addresses.contains(&address) {
            self.pinned_addresses.push(address);
        }
    }

    pub fn unpin_address(&mut self, address: MemoryAddress) {
        if self.pinned_addresses.contains(&address) {
            let index = self
                .pinned_addresses
                .iter()
                .position(|x| *x == address)
                .unwrap();
            self.pinned_addresses.remove(index);
        }
    }
}

impl Widget for &mut MemoryEditor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(5)])
            .split(area);

        let list = List::new(self.list_items.clone()).highlight_symbol(">> ");

        StatefulWidget::render(list, area, buf, &mut self.widget_state);

        self.value_edit_field.render(rows[1], buf);
    }
}

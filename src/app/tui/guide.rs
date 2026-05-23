use crate::app::events::Focus;
use crate::app::App;
use ratatui::layout::Direction;
use ratatui::macros::constraint;
use ratatui::widgets::{BorderType, Borders, Paragraph};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, Widget, Wrap},
};

pub fn render_guide(area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
    Paragraph::new(
        "Welcome to Machina Politykiera

This is a tool for viewing and editing of memory of other processes running on the same system.

On your right you can see 4 windows:

1. Processes => use arrow keys and enter to select which process's memory you want to scan through

2. Scan Options => search for a custom value with a correct type

3. Scanned Memory => look through the scanned memory values using arrow keys and then press enter to pin addresses of choice

4. Memory Editor => change the values inside the pinned addresses

5. Activity Logs => if anything unexpected happens, make sure to check them out 
        ",
    ).wrap(Wrap { trim: true })
    .render(area, buf);
}

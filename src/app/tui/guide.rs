use ratatui::widgets::Paragraph;
use ratatui::widgets::{Widget, Wrap};

pub fn render_guide(area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
    Paragraph::new(
        "Welcome to Machina Politykiera

This is a tool for viewing and editing of memory of other processes running on the same system.

On your right you can see 4 windows:

1. Processes => use arrow keys and enter to select which process's memory you want to scan through

2. Scan Options => search for a custom value with a correct type

3. Scanned Memory => look through the scanned memory values using arrow keys and then press enter to pin addresses of choice

4. Memory Editor => change the values inside the pinned addresses

If anything unexpected happens, reference the activity log below.
        ",
    ).wrap(Wrap { trim: true })
    .render(area, buf);
}

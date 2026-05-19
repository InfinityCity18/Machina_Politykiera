use crate::app::events::Focus;
use crate::app::App;
use ratatui::layout::Direction;
use ratatui::widgets::{Borders, Paragraph};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, Widget, Wrap},
};
pub mod title;

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(area);

        let left_cols = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(10), Constraint::Fill(1)])
            .split(columns[0]);

        let middle_cols = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(2, 5), Constraint::Ratio(3, 5)])
            .split(columns[1]);

        let right_cols = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(2, 5), Constraint::Ratio(3, 5)])
            .split(columns[2]);

        let title_block = Block::default().title(" Title ").borders(Borders::ALL);
        let guide_block = Block::default().title(" User Guide ").borders(Borders::ALL);
        let mut process_list_block = Block::default().title(" Processes ").borders(Borders::ALL);
        let mut memory_list_block = Block::default()
            .title(" Scanned memory ")
            .borders(Borders::ALL);
        let scan_options_block = Block::default()
            .title(" Scan options ")
            .borders(Borders::ALL);

        (&scan_options_block).render(right_cols[0], buf);
        let mut scan_options_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(scan_options_block.inner(right_cols[0]));

        let mut pinned_memory_block = Block::default()
            .title(" Pinned memory ")
            .borders(Borders::ALL);

        match self.focus_window {
            Focus::MemoryListWindow => {
                memory_list_block =
                    memory_list_block.border_style(Style::default().fg(Color::Green));
            }
            Focus::ProcessListWindow => {
                process_list_block =
                    process_list_block.border_style(Style::default().fg(Color::Green));
            }
            Focus::PinnedMemoryWindow => {
                pinned_memory_block =
                    pinned_memory_block.border_style(Style::default().fg(Color::Green));
            }
            _ => {}
        };

        // draw title
        let text = self.title_text.clone();
        let title_window = Paragraph::new(text)
            .block(title_block)
            .alignment(Alignment::Center);
        title_window.render(left_cols[0], buf);

        // draw guide
        guide_block.render(left_cols[1], buf);

        // draw process list
        (&process_list_block).render(middle_cols[0], buf);
        self.process_list
            .render(process_list_block.inner(middle_cols[0]), buf);

        // draw scan options
        self.input_field.render(scan_options_column[0], buf);

        let type_selection =
            Paragraph::new("Type select goes here").block(Block::default().borders(Borders::ALL));
        type_selection.render(scan_options_column[1], buf);

        let scan_info_text = Paragraph::new(
            "Type the [V]alue above and press enter to search for the associated [T]ype.",
        )
        .wrap(Wrap { trim: true });
        scan_info_text.render(scan_options_column[2], buf);

        // draw memory list
        memory_list_block.render(middle_cols[1], buf);
        pinned_memory_block.render(right_cols[1], buf);
    }
}

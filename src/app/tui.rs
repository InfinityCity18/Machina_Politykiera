use crate::app::events::Focus;
use crate::app::App;
use ratatui::layout::Direction;
use ratatui::widgets::{Borders, Paragraph};
use ratatui::{
    layout::{Alignment, Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, Widget},
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

        let left_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)])
            .split(columns[0]);

        let middle_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(2, 5), Constraint::Ratio(3, 5)])
            .split(columns[1]);

        let right_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Ratio(2, 5), Constraint::Ratio(3, 5)])
            .split(columns[2]);

        let block1 = Block::default().title(" Title ").borders(Borders::ALL);
        let block2 = Block::default().title(" User Guide ").borders(Borders::ALL);
        let mut block3 = Block::default().title(" Processes ").borders(Borders::ALL);

        let mut block4 = Block::default()
            .title(" Scanned memory ")
            .borders(Borders::ALL);

        let mut block5 = Block::default()
            .title(" Scan options ")
            .borders(Borders::ALL);

        let mut block6 = Block::default()
            .title(" Pinned memory ")
            .borders(Borders::ALL);

        let text = self.title_text.clone();

        let p = Paragraph::new(text)
            .block(block1)
            .alignment(Alignment::Center);

        match self.focus_window {
            Focus::MemoryListWindow => {
                block4 = block4.border_style(Style::default().fg(Color::Green));
            }
            Focus::ProcessListWindow => {
                block3 = block3.border_style(Style::default().fg(Color::Green));
            }
            Focus::PinnedMemoryWindow => {
                block6 = block6.border_style(Style::default().fg(Color::Green));
            }
        };

        p.render(left_rows[0], buf);
        block2.render(left_rows[1], buf);
        block4.render(middle_rows[1], buf);
        block5.render(right_rows[0], buf);
        block6.render(right_rows[1], buf);

        if matches!(self.focus_window, Focus::ProcessListWindow) {}

        (&block3).render(middle_rows[0], buf);

        self.process_list.render(block3.inner(middle_rows[0]), buf);
    }
}

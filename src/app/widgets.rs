use crate::app::App;
use ratatui::layout::Direction;
use ratatui::widgets::{Borders, Paragraph};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    widgets::{Block, Widget},
};

impl Widget for &App {
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

        // Render Blocks into each of the 6 sections to visualize the grid
        let block1 = Block::default().title(" Block 1 ").borders(Borders::ALL);
        let block2 = Block::default().title(" Block 2 ").borders(Borders::ALL);
        let block3 = Block::default().title(" Block 3 ").borders(Borders::ALL);

        let block4 = Block::default().title(" Block 4 ").borders(Borders::ALL);
        let block5 = Block::default().title(" Block 5 ").borders(Borders::ALL);
        let block6 = Block::default().title(" Block 6 ").borders(Borders::ALL);

        block1.render(left_rows[0], buf);
        block2.render(left_rows[1], buf);
        block3.render(middle_rows[0], buf);
        block4.render(middle_rows[1], buf);
        block5.render(right_rows[0], buf);
        block6.render(right_rows[1], buf);
    }
}

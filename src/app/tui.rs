use crate::app::events::Focus;
use crate::app::tui::guide::render_guide;
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
pub mod guide;
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
            .constraints([
                Constraint::Length(10),
                Constraint::Fill(1),
                Constraint::Length(10),
            ])
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
        let mut logger_block = Block::default()
            .title(" Activity [L]ogs ")
            .borders(Borders::ALL);

        let mut process_list_block = Block::default()
            .title(" [P]rocesses ")
            .borders(Borders::ALL);
        let mut memory_list_block = Block::default()
            .title(" Scanned [M]emory ")
            .borders(Borders::ALL);
        let scan_options_block = Block::default()
            .title(" Scan Options ")
            .borders(Borders::ALL);

        (&scan_options_block).render(right_cols[0], buf);
        let scan_options_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(2),
                Constraint::Fill(1),
            ])
            .split(scan_options_block.inner(right_cols[0]));

        let scan_info_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .split(scan_options_column[4]);

        let mut memory_editor_block = Block::default()
            .title(" Memory [E]ditor ")
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
            Focus::MemoryEditorWindow => {
                memory_editor_block =
                    memory_editor_block.border_style(Style::default().fg(Color::Green));
            }
            Focus::LoggerWindow => {
                logger_block = logger_block.border_style(Style::default().fg(Color::Green));
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
        (&guide_block).render(left_cols[1], buf);
        render_guide(guide_block.inner(left_cols[1]), buf);

        // draw logger
        (&logger_block).render(left_cols[2], buf);
        self.logger.render(logger_block.inner(left_cols[2]), buf);

        // draw process list
        (&process_list_block).render(middle_cols[0], buf);
        self.process_list
            .render(process_list_block.inner(middle_cols[0]), buf);

        // draw scan options
        let selected_process_text = match self.selected_process.clone() {
            Some(proc) => match proc.stat() {
                Ok(stat) => format!("{:<8} {}", stat.pid, stat.comm),
                Err(_err) => "Error - couldn't parse name or pid".to_string(),
            },
            None => "None".to_string(),
        };

        Paragraph::new(selected_process_text)
            .block(
                Block::default()
                    .title(" Selected Process ")
                    .borders(Borders::all()),
            )
            .render(scan_options_column[0], buf);

        self.scan_value_field.render(scan_options_column[1], buf);

        self.scan_type_selector.render(scan_options_column[2], buf);

        let scan_info_text =
            Paragraph::new("Type above a [V]alue of the selected [T]ype then perform:")
                .wrap(Wrap { trim: true });

        scan_info_text.render(scan_options_column[3], buf);
        let first_scan_info =
            Paragraph::new("[F]irst scan to find all memory adresses with matching values")
                .block(Block::default().borders(Borders::all()))
                .wrap(Wrap { trim: true });

        first_scan_info.render(scan_info_row[0], buf);
        let next_scan_info = Paragraph::new(
            "[N]ext scan to filter out found memory adresses that don't match the value",
        )
        .block(Block::default().borders(Borders::all()))
        .wrap(Wrap { trim: true });

        next_scan_info.render(scan_info_row[1], buf);

        // draw memory list
        (&memory_list_block).render(middle_cols[1], buf);
        self.memory_scanner
            .render(memory_list_block.inner(middle_cols[1]), buf);

        // draw memory editor
        (&memory_editor_block).render(right_cols[1], buf);
        let memory_editor_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Length(5),
            ])
            .split(memory_editor_block.inner(right_cols[1]));

        self.memory_editor.render(memory_editor_column[0], buf);
        self.new_value_field.render(memory_editor_column[1], buf);
        Paragraph::new("Type above the new value for the selected memory address. Make sure it matches the correct type and press enter to overwrite.")
            .wrap(Wrap { trim: true }).block(Block::default().borders(Borders::all())).render(memory_editor_column[2], buf);
    }
}

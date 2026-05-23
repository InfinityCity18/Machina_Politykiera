use ratatui::widgets::{Paragraph, Widget, Wrap};
use std::cmp::min;

pub struct Logger {
    logs: Vec<String>,
    offset: usize,
}

impl Logger {
    pub fn new() -> Logger {
        Self {
            logs: vec![],
            offset: 0,
        }
    }

    pub fn log(&mut self, lg: String) {
        self.logs.push(format!(">> {}", lg));
    }

    pub fn scroll_up(&mut self) {
        self.offset = self.offset.saturating_add(1);
    }

    pub fn scroll_down(&mut self) {
        self.offset = self.offset.saturating_sub(1);
    }
}

impl Widget for &mut Logger {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if self.logs.is_empty() || area.height == 0 || area.width == 0 {
            return;
        }

        let width = area.width as usize;
        let num_lines = area.height as usize;
        let total_logs = self.logs.len();

        let lines_for = |s: &str| s.chars().count().saturating_sub(1) / width + 1;

        let mut max_offset = 0;
        let mut visible = 0;
        for i in 0..total_logs {
            visible += lines_for(&self.logs[i]);
            if visible > num_lines {
                max_offset = total_logs.saturating_sub(i + 1);
                break;
            }
        }

        self.offset = min(self.offset, max_offset);

        let end_index = total_logs.saturating_sub(self.offset);
        let mut start_index = end_index;
        let mut current_height = 0;

        while start_index > 0 {
            let h = lines_for(&self.logs[start_index - 1]);

            if current_height + h > num_lines && start_index < end_index {
                break;
            }

            current_height += h;
            start_index -= 1;
        }

        let text = self.logs[start_index..end_index].join("\n");

        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

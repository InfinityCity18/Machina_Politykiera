use log::{Metadata, Record};
use ratatui::widgets::{Paragraph, Widget, Wrap};
use std::{cmp::min, sync::{Arc, Mutex}};

pub type LogsMutex = Arc<Mutex<Vec<String>>>;

pub struct LoggerGlobal {
    logs: LogsMutex
}

impl LoggerGlobal {
    pub fn new(logs_mutex: LogsMutex) -> LoggerGlobal {
        LoggerGlobal { logs: logs_mutex }
    }
}

impl log::Log for LoggerGlobal {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut logs_lock = self.logs.lock().expect("Logs mutex got poisoned, panicking...");
            logs_lock.push(format!("{} - {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {
        let mut logs_lock = self.logs.lock().expect("Logs mutex got poisoned, panicking...");
        logs_lock.clear();
    }
}

pub struct Logger {
    logs: LogsMutex,
    offset: usize,
}

impl Logger {
    pub fn new(logs_mutex: LogsMutex) -> Logger {
        Self {
            logs: logs_mutex,
            offset: 0,
        }
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
        let logs_lock = self.logs.lock().expect("Logs mutex got poisoned, panicking...");
        if logs_lock.is_empty() || area.height == 0 || area.width == 0 {
            return;
        }

        let width = area.width as usize;
        let num_lines = area.height as usize;
        let total_logs = logs_lock.len();

        let lines_for = |s: &str| s.chars().count().saturating_sub(1) / width + 1;

        let mut max_offset = 0;
        let mut visible = 0;
        for i in 0..total_logs {
            visible += lines_for(&logs_lock[i]);
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
            let h = lines_for(&logs_lock[start_index - 1]);

            if current_height + h > num_lines && start_index < end_index {
                break;
            }

            current_height += h;
            start_index -= 1;
        }

        let text = logs_lock[start_index..end_index].join("\n");

        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

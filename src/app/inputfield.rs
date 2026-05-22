use crossterm::event::KeyCode;
use ratatui::widgets::Widget;
use ratatui::widgets::{Borders, Paragraph};
use ratatui::{
    style::Color, style::Modifier, style::Style, text::Line, text::Span, widgets::Block,
};

pub struct InputField {
    title: String,
    pub input: String,
    index: usize,
    pub selected: bool,
}

impl InputField {
    pub fn new(title: String) -> InputField {
        Self {
            title: title,
            input: String::new(),
            index: 0,
            selected: false,
        }
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) {
        match key_event.code {
            KeyCode::Char(chr) => self.insert_char(chr),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            _ => (),
        }
    }

    pub fn clear(&mut self) {
        self.input = "".to_string();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn move_left(&mut self) {
        let moved = self.index.saturating_sub(1);
        self.index = self.clamp_cursor(moved)
    }

    fn move_right(&mut self) {
        let moved = self.index.saturating_add(1);
        self.index = self.clamp_cursor(moved)
    }

    fn insert_char(&mut self, new_char: char) {
        self.input.insert(self.index, new_char);
        self.move_right();
    }

    fn delete_char(&mut self) {
        if self.index != 0 {
            let current_index = self.index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_left();
        }
    }
}

impl Widget for &mut InputField {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL);

        let p = if self.selected {
            block = block
                .clone()
                .border_style(Style::default().fg(Color::Green));
            let text_before_cursor = &self.input[..self.index];

            let cursor_char = if self.index < self.input.len() {
                &self.input[self.index..self.index + 1]
            } else {
                " "
            };

            let text_after_cursor = if self.index < self.input.len() {
                &self.input[self.index + 1..]
            } else {
                ""
            };

            let line = Line::from(vec![
                Span::raw(text_before_cursor),
                Span::styled(
                    cursor_char,
                    Style::default().add_modifier(Modifier::REVERSED),
                ),
                Span::raw(text_after_cursor),
            ]);
            Paragraph::new(line).block(block)
        } else {
            Paragraph::new(self.input.clone()).block(block)
        };

        p.render(area, buf);
    }
}

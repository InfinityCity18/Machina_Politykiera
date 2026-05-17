use crossterm::event::KeyCode;
use ratatui::widgets::Widget;
use ratatui::widgets::{Borders, Paragraph};
use ratatui::{style::Modifier, style::Style, text::Line, text::Span, widgets::Block};

pub struct InputField {
    input: String,
    index: usize,
}

impl InputField {
    pub fn new() -> InputField {
        Self {
            input: String::new(),
            index: 0,
        }
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) {
        match key_event.code {
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Char(chr) => self.insert_char(chr),
            _ => (),
        }
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.index)
            .unwrap_or(self.input.len())
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
        let index = self.byte_index();
        self.input.insert(index, new_char);
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
        let block = Block::default().title(" Value ").borders(Borders::ALL);

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

        let p = Paragraph::new(line).block(block);

        p.render(area, buf);
    }
}

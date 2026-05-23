use ratatui::widgets::Widget;
use ratatui::widgets::{Borders, Paragraph};
use ratatui::widgets::Block;

use crate::app::scansettings::ScanValueType;
use crate::app::scansettings::ScanValueType::*;

pub struct TypeSelector {
    pub selected: ScanValueType,
}

impl TypeSelector {
    pub fn new() -> TypeSelector {
        Self { selected: Byte }
    }
    pub fn cycle_type(&mut self) {
        self.selected = match self.selected {
            Byte => Word,
            Word => DWord,
            DWord => QWord,
            QWord => Float,
            Float => Double,
            Double => String(0),
            String(_) => Byte,
        }
    }

    pub fn get_text(&self) -> &str {
        match self.selected {
            Byte => "Byte    | 1 byte  | 8  bits",
            Word => "Word    | 2 bytes | 16 bits",
            DWord => "DWord   | 4 bytes | 32 bits",
            QWord => "QWord   | 8 bytes | 64 bits",
            Float => "Float   | 4 bytes | 32 bits",
            Double => "Double  | 8 bytes | 64 bits",
            String(_) => "String  | any",
        }
    }
}

impl Widget for &mut TypeSelector {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::default().title(" [T]ype ").borders(Borders::ALL);

        let p = Paragraph::new([" ", self.get_text()].join("")).block(block);
        p.render(area, buf);
    }
}

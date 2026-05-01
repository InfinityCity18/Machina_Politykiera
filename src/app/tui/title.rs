use crate::app::Event;
use std::time::Duration;
use std::{sync::mpsc, thread};
pub fn cycle_title(tx: mpsc::Sender<Event>, mut text: String) {
    loop {
        let mut m: Vec<char> = text.chars().collect();
        m.rotate_left(1);
        text = m.into_iter().collect();
        thread::sleep(Duration::from_millis(100));
        tx.send(Event::Title(text.clone())).unwrap();
    }
}

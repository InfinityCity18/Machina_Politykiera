use crate::app::Event;
use std::time::Duration;
use std::{sync::mpsc, thread};

pub const TITLE: &str = r#"
                  __  __                   _         _                               ___             _       _      _       _  _    _        _                                 
    o O O        |  \/  |  __ _     __    | |_      (_)    _ _     __ _             | _ \   ___     | |     (_)    | |_    | || |  | |__    (_)     ___      _ _   __ _        
   o             | |\/| | / _` |   / _|   | ' \     | |   | ' \   / _` |            |  _/  / _ \    | |     | |    |  _|    \_, |  | / /    | |    / -_)    | '_| / _` |       
  TS__[O]        |_|__|_| \__,_|   \__|   |_||_|    |_|   |_||_|  \__,_|            |_|    \___/    |_|     |_|     \__|    |__/   |_\_\    |_|    \___|   _|_|_  \__,_|       
 {======|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|_| """ |_|"""""|_|"""""|_|"""""|_|"""""|_| """"|_|"""""|_|"""""|_|"""""|_|"""""|_|"""""|      
./o--000'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'"`-0-0-'      "#;

pub fn cycle_title(tx: mpsc::Sender<Event>, mut text: String) {
    loop {
        let m: String = text
            .lines()
            .map(|line| {
                let mut chars: Vec<char> = line.chars().collect();
                if !chars.is_empty() {
                    chars.rotate_left(1);
                }
                chars.into_iter().collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        text = m;
        thread::sleep(Duration::from_millis(100));
        tx.send(Event::Title(text.clone())).unwrap();
    }
}

use std::{io, sync::mpsc, thread};

use ratatui::style::Color;

mod app;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let (event_tx, event_rx) = mpsc::channel::<app::events::Event>();

    let tx_input_events = event_tx.clone();
    thread::spawn(move || {
        app::events::handle_input_events(tx_input_events);
    });

    let tx_title_anim = event_tx.clone();
    thread::spawn(move || {
        app::cycle_title(tx_title_anim);
    });

    let mut app = app::App {
        exit: false,
        progress_bar_color: Color::Green,
        title_text: String::from("Machina Politykiera"),
    };

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();
    app_result
}

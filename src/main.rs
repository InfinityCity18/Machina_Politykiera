use std::{io, sync::mpsc, thread};

use ratatui::style::Color;

mod app;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let (event_tx, event_rx) = mpsc::channel::<app::input::Event>();

    let tx_to_input_events = event_tx.clone();
    thread::spawn(move || {
        app::input::handle_input_events(tx_to_input_events);
    });

    let tx_to_background_progress_events = event_tx.clone();
    thread::spawn(move || {
        app::run_background_thread(tx_to_background_progress_events);
    });

    let mut app = app::App {
        exit: false,
        progress_bar_color: Color::Green,
        background_progress: 0_f64,
    };

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();
    app_result
}

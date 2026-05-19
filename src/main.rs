use std::{io, sync::mpsc, thread};

use crate::app::tui::title;

mod app;

fn launch_threads(event_tx: mpsc::Sender<app::events::Event>) {
    let tx_input_events = event_tx.clone();
    thread::spawn(move || {
        app::events::handle_input_events(tx_input_events);
    });

    let tx_title_anim = event_tx.clone();
    thread::spawn(move || {
        app::tui::title::cycle_title(tx_title_anim, title::TITLE.to_string());
    });

    let tx_process_update = event_tx.clone();
    thread::spawn(move || {
        app::processlist::update_processes_periodically(tx_process_update, 1);
    });
}
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let (event_tx, event_rx) = mpsc::channel::<app::events::Event>();

    launch_threads(event_tx);

    let mut app = app::App::new();

    let app_result = app.run(&mut terminal, event_rx);

    ratatui::restore();
    app_result
}

mod models;
mod app;
mod ui;

use crate::app::AppState;
use crate::models::{CurrentPage, TodoItem};
use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut state = AppState::default();
    setup_dummy_data(&mut state);

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &mut state);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal, state: &mut AppState) -> Result<()> {
    while state.running {
        terminal.draw(|f| ui::render(f, state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press { continue; }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => state.running = false,
                KeyCode::Char('1') => state.current_page = CurrentPage::Tasks,
                KeyCode::Char('2') => state.current_page = CurrentPage::Archives,
                KeyCode::Up | KeyCode::Char('k') => state.previous(),
                KeyCode::Down | KeyCode::Char('j') => state.next(),
                KeyCode::Char(' ') => state.toggle_status(),
                KeyCode::Enter => state.archive_current(),
                KeyCode::Char('x') | KeyCode::Delete => state.delete_current(),
                KeyCode::Char('K') => state.move_up(),
                KeyCode::Char('J') => state.move_down(),
                _ => {}
            }
        }
    }
    Ok(())
}

fn setup_dummy_data(state: &mut AppState) {
    for i in 1..=10 {
        state.items.push(TodoItem::new(&format!("Tâche importante numéro {}", i)));
    }
}
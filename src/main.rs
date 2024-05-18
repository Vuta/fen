use std::io::{self, stdout, Stdout};

use crossterm::{
    execute,
    event::{self, *},
    terminal::*,
};
use ratatui::prelude::*;

mod state;
mod parser;
mod view;

pub type Backend = CrosstermBackend<Stdout>;

fn main() {
    let mut terminal = init_terminal().unwrap();
    let mut state = state::App::new();

    loop {
        let _ = terminal.draw(|frame| view::draw(frame, &state));
        let _ = handle_events(&mut state);

        if state.exit {
            let _ = restore();

            return;
        }
    }
}

fn init_terminal() -> io::Result<Terminal<Backend>> {
    execute!(stdout(), EnterAlternateScreen)?;
    let _ = enable_raw_mode();
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    let _ = disable_raw_mode();

    Ok(())
}

fn handle_events(state: &mut state::App) -> io::Result<()> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Esc => { state.exit = true; },
            KeyCode::Enter => {
                match parser::run(&state.input) {
                    Ok(board) => {
                        state.board = board;
                        state.error = String::new();
                    },
                    Err(err) => { state.error = err.to_string(); },
                }
                state.input = String::new();
            },
            KeyCode::Char(c) => { state.input.push(c); },
            _ => {}
        }
    }

    Ok(())
}

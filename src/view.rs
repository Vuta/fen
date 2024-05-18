use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::style::Color::Rgb;

use crate::state::{App, Piece, Color};

pub fn draw(frame: &mut Frame, state: &App) {
    let rows = build_rows(state);
    let widths = [
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ];
    let table = Table::new(rows, widths).column_spacing(0);
    let block = Block::default().borders(Borders::ALL);

    frame.render_widget(table.block(block), frame.size());

    let rect = Rect::new(1, 10, 100, 1);
    let text = String::from("Enter FEN: ") + &state.input;

    frame.render_widget(Paragraph::new(text), rect);
}

fn build_rows(app: &App) -> Vec<Row> {
    let mut square_color = false;
    let mut rows: Vec<Row> = Vec::with_capacity(8);
    let state = app.board.state;

    for i in 0..=7 {
        let mut row: Vec<Cell> = Vec::with_capacity(8);

        for j in 0..=7 {
            let id = i * 8 + j;
            let cell = cell(state[id], square_color);

            row.push(cell);

            if (id + 1) % 8 != 0 { square_color = !square_color; } 
        }

        rows.push(Row::new(row));
    }

    rows
}

// TODO: Refactor
fn cell(p: Option<Piece>, square_c: bool) -> Cell<'static> {
    let style = match square_c {
        true => Style::new().on_green(),
        false => Style::new().bg(Rgb(222, 222, 222)),
    };

    let mut c = "";
    let mut text_color = ratatui::style::Color::White;

    match p {
        Some(p) => {
            c = p.to_string();

            match p.0 {
                Color::Black => {
                    text_color = ratatui::style::Color::Black;
                },
                _ => {},
            }
        },
        None => {},
    }

    Cell::from(
        Text::from(c)
            .centered()
            .style(Style::default().fg(text_color))
    ).style(style)
}

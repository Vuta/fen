use crate::state::*;

pub fn run(position: &str) -> Result<Board, &'static str> {
    let ranks: Vec<&str> = position.split("/").collect();
    if ranks.len() != 8 { return Err("Invalid"); }

    let mut board = Board { state: [None; 64] };

    for (r_id, rank) in ranks.iter().enumerate() {
        let mut count: usize = 0;
        let mut consecutive_num = false;

        for c in rank.chars() {
            if c >= '1' && c <= '8' {
                if consecutive_num == true { return Err("Invalid"); }

                count += c.to_digit(10).unwrap() as usize;
                consecutive_num = true;
            } else {
                match parse_piece(c) {
                    None => return Err("Invalid"),
                    Some(piece) => {
                        board.state[r_id * 8 + count] = Some(piece);
                    },
                }

                count += 1;
                consecutive_num = false;
            }
        }

        if count != 8 { return Err("Invalid"); }
    }

    Ok(board)
}

fn parse_piece(c: char) -> Option<Piece> {
    match c {
        'p' => Some(Piece(Color::Black, Type::Pawn)),
        'r' => Some(Piece(Color::Black, Type::Rook)),
        'n' => Some(Piece(Color::Black, Type::Knight)),
        'b' => Some(Piece(Color::Black, Type::Bishop)),
        'q' => Some(Piece(Color::Black, Type::Queen)),
        'k' => Some(Piece(Color::Black, Type::King)),
        'P' => Some(Piece(Color::White, Type::Pawn)),
        'R' => Some(Piece(Color::White, Type::Rook)),
        'N' => Some(Piece(Color::White, Type::Knight)),
        'B' => Some(Piece(Color::White, Type::Bishop)),
        'Q' => Some(Piece(Color::White, Type::Queen)),
        'K' => Some(Piece(Color::White, Type::King)),
        _ => None,
    }
}

use crate::parser;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece(pub Color, pub Type);

#[derive(Debug)]
pub struct Board {
    pub state: [Option<Piece>; 64],
}

pub struct App {
    pub board: Board,
    pub input: String,
    pub error: String,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        let start_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let board = parser::run(start_position).unwrap();

        Self {
            board: board,
            input: String::new(),
            error: String::new(),
            exit: false,
        }
    }
}

impl Piece {
     pub fn to_string(self) -> &'static str {
         match self {
             Piece(Color::White, Type::Pawn) => "♙",
             Piece(Color::White, Type::Rook) => "♖",
             Piece(Color::White, Type::Knight) => "♘",
             Piece(Color::White, Type::Bishop) => "♗",
             Piece(Color::White, Type::Queen) => "♕",
             Piece(Color::White, Type::King) => "♔",
             Piece(Color::Black, Type::Pawn) => "♟",
             Piece(Color::Black, Type::Rook) => "♜",
             Piece(Color::Black, Type::Knight) => "♞",
             Piece(Color::Black, Type::Bishop) => "♝",
             Piece(Color::Black, Type::Queen) => "♛",
             Piece(Color::Black, Type::King) => "♚",
         }
     }
 }

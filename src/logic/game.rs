use crate::{Board, Move, Position};

pub struct Game {
    current_board: Board,
    history: Vec<Board>,
    turn: Turn,
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_board: Board::new(),
            history: vec![],
            turn: Turn::White,
        }
    }
    pub fn possible_moves(&self, position: &Position) -> Vec<Move> {
        self.current_board.possible_valid_moves(position)
    }
    pub fn make_move(&mut self, from: &Position, to: &Position) -> Result<(), &str> {
        match self.current_board.make_move(from, to) {
            Some(board) => {
                self.update_game(board);
                Ok(())
            }
            None => Err("invalid move")
        }
    }

    fn update_game(&mut self, new_board: Board) {
        self.history.push(self.current_board.clone());
        self.current_board = new_board;
        self.turn = match self.turn {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White,
        };
    }
}


pub enum Turn {
    White,
    Black,
}
use crate::{Board, Color, Move, Position};
use crate::logic::rules::knight_and_king_rules;

pub fn possible_moves(board: &Board, position: Position) -> Vec<Move> {
    knight_and_king_rules::possible_moves(position, board,  combinations())
}

pub fn possible_captures(board: &Board, position: Position, color: &Color) -> Vec<Move> {
    knight_and_king_rules::possible_captures(position, board, color, combinations())
}

fn combinations() -> Vec<(i32, i32)> {
    vec![
        (2, 1), (1, 2), (2, -1), (1, -2), (-2, 1), (-2, -1), (-1, -2), (-1, 2),
    ]
}
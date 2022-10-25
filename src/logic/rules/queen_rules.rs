use crate::logic::rules::bishop_rules::possible_captures as bishop_captures;
use crate::logic::rules::bishop_rules::possible_moves as bishop_moves;
use crate::logic::rules::rook_rules::possible_captures as rook_captures;
use crate::logic::rules::rook_rules::possible_moves as rook_moves;
use crate::{Board, Color, Move, Position};

pub fn possible_moves(board: &Board, position: Position) -> Vec<Move> {
    let mut moves = vec![];
    moves.append(&mut bishop_moves(board, position));
    moves.append(&mut rook_moves(board, position));
    moves
}

pub fn possible_captures(board: &Board, position: Position, color: &Color) -> Vec<Move> {
    let mut captures = vec![];
    captures.append(&mut bishop_captures(board, position, color));
    captures.append(&mut rook_captures(board, position, color));
    captures
}

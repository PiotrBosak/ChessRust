use crate::logic::*;

use crate::{Board, Color, Move, Piece, PieceType, Position};
use crate::logic::rules::pawn_rules::pawn_rules;

pub fn possible_moves(board: &Board, position: &Position, piece: &Piece) -> Vec<Move> {
    match piece.piece_type {
        PieceType::Pawn => pawn_rules::possible_pawn_moves(board, position, &piece.color),
        _ => vec![],
    }
}

pub fn possible_captures(board: &Board, position: &Position, piece: &Piece) -> Vec<Move> {
    match piece.piece_type {
        PieceType::Pawn => pawn_rules::possible_pawn_captures(board, position, &piece.color),
        _ => vec![],
    }
}

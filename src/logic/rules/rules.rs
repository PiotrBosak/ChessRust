use crate::logic::*;

use crate::{Board, Color, Move, Piece, PieceType, Position};
use crate::logic::rules::{
    pawn_rules,
    king_rules,
    knight_rules,
    rook_rules,
    queen_rules,
    bishop_rules,
};

pub fn possible_moves(board: &Board, position: &Position, piece: &Piece) -> Vec<Move> {
    match piece.piece_type {
        PieceType::Pawn => pawn_rules::possible_moves(board, position, &piece.color),
        PieceType::Knight => knight_rules::possible_moves(board, position.clone()),
        PieceType::Bishop => bishop_rules::possible_moves(board, position.clone()),
        PieceType::Rook => rook_rules::possible_moves(board, position.clone()),
        PieceType::Queen => queen_rules::possible_moves(board, position.clone()),
        PieceType::King => king_rules::possible_moves(board, position.clone(), &piece.color)
    }
}

pub fn possible_captures(board: &Board, position: &Position, piece: &Piece) -> Vec<Move> {
    match piece.piece_type {
        PieceType::Pawn => pawn_rules::possible_captures(board, position, &piece.color),
        PieceType::Knight => knight_rules::possible_captures(board, position.clone(), &piece.color),
        PieceType::Bishop => bishop_rules::possible_captures(board, position.clone(), &piece.color),
        PieceType::Rook => rook_rules::possible_captures(board, position.clone(), &piece.color),
        PieceType::Queen => queen_rules::possible_captures(board, position.clone(), &piece.color),
        PieceType::King => king_rules::possible_captures(board, position.clone(), &piece.color)
    }
}

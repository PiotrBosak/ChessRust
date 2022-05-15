use crate::{Advance, Board, Color, Move, MoveType, Position};

pub fn possible_moves(position: Position, board: &Board, combinations: Vec<(i32, i32)>) -> Vec<Move> {
    find_positions(position, combinations)
        .into_iter()
        .filter(|pos| board.tile_at(pos).is_empty())
        .map(|pos| Move::new(position, pos, MoveType::Move))
        .collect()
}

pub fn possible_captures(position: Position, board: &Board, color: &Color, combinations: Vec<(i32, i32)>) -> Vec<Move> {
    find_positions(position, combinations)
        .into_iter()
        .filter(|pos| has_enemy_piece(board, pos, color))
        .map(|pos| Move::new(position, pos, MoveType::Capture))
        .collect()
}

fn has_enemy_piece(board: &Board, position: &Position, color: &Color) -> bool {
    board
        .tile_at(position)
        .current_piece
        .filter(|piece| piece.color != *color)
        .is_some()
}

fn find_positions(position: Position, combinations: Vec<(i32, i32)>) -> Vec<Position> {
    combinations
        .into_iter()
        .filter_map(|(rank_diff, file_diff)| {
            let next_rank = position.rank.advance(rank_diff)?;
            let next_file = position.file.advance(file_diff)?;
            Some(Position::new(next_file, next_rank))
        })
        .collect()
}


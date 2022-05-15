pub mod bishop_rules {
    use crate::{Board, Color, Move, MoveType, Position};
    use crate::logic::rules::multi_tile_rules::multi_tile_rules::possible_move_positions as multi_tile_possible_moves;
    use crate::logic::rules::multi_tile_rules::multi_tile_rules::possible_capture_positions as multi_tile_possible_captures;

    pub fn possible_moves(board: &Board, position: Position) -> Vec<Move> {
        vec![
            multi_tile_possible_moves(board, position, 1, 1),
            multi_tile_possible_moves(board, position, 1, -1),
            multi_tile_possible_moves(board, position, -1, 1),
            multi_tile_possible_moves(board, position, -1, -1),
        ]
            .into_iter()
            .flatten()
            .map(|next_position| Move::new(position, next_position, MoveType::Move))
            .collect()
    }

    pub fn possible_captures(board: &Board, position: Position, color: &Color) -> Vec<Move> {
        vec![
            multi_tile_possible_captures(board, position, 1, 1, color),
            multi_tile_possible_captures(board, position, 1, -1, color),
            multi_tile_possible_captures(board, position, -1, 1, color),
            multi_tile_possible_captures(board, position, -1, -1, color),
        ]
            .into_iter()
            .flatten()
            .map(|next_position| Move::new(position, next_position, MoveType::Capture))
            .collect()
    }
}
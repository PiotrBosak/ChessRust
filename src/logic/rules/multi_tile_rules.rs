    use crate::{Advance, Board, Color, Move, Position};

    pub fn possible_move_positions(
        board: &Board,
        position: Position,
        next_rank_diff: i8,
        next_file_diff: i8,
    ) -> Vec<Position> {
        let mut next_position = find_next_position(position, next_rank_diff, next_file_diff);
        let mut vec = vec![];
        while let Some(position) = next_position {
            if board.tile_at(&position).is_empty() {
                vec.push(position);
            }
            next_position = find_next_position(position, next_rank_diff, next_file_diff);
        }
        vec
    }

    pub fn possible_capture_positions<'a>(
        board: &Board,
        position: Position,
        next_rank_diff: i8,
        next_file_diff: i8,
        color: &Color,
    ) -> Vec<Position> {
        let mut next_position = find_next_position(position, next_rank_diff, next_file_diff);
        let mut vec = vec![];
        while let Some(position) = next_position {
            if board
                .tile_at(&position)
                .current_piece
                .filter(|piece| piece.color != *color)
                .is_some() {
                vec.push(position);
            }
            next_position = find_next_position(position, next_rank_diff, next_file_diff);
        }
        vec
    }

    fn find_next_position(position: Position, next_rank_diff: i8, next_file_diff: i8) -> Option<Position> {
        let next_file = position.file.advance(next_file_diff as i32)?;
        let next_rank = position.rank.advance(next_rank_diff as i32)?;
        let next_position = Position::new(next_file, next_rank);
        Some(next_position)
    }

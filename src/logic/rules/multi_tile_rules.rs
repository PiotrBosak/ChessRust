use crate::{Advance, Board, Color, File, Move, Position, Rank};

pub fn possible_move_positions(
    board: &Board,
    position: Position,
    next_rank_diff: i8,
    next_file_diff: i8,
) -> Vec<Position> {
    let mut next_position = find_next_position(position, next_rank_diff, next_file_diff);
    let mut vec = vec![];
    while let Some(position) = next_position.filter(|pos| {
        board.tile_at(pos).is_empty()
    }) {
        vec.push(position);
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
) -> Option<Position> {
    let mut posistion = find_next_position(position, next_rank_diff, next_file_diff);
    while let Some(pos) = posistion {
        match board.tile_at(&pos).current_piece {
            Some(piece) =>
                return if &piece.color != color {
                    Some(pos)
                } else {
                    None
                },
            None =>
                posistion = find_next_position(pos, next_rank_diff, next_file_diff)
        }
    }
    None
}

fn find_next_position(position: Position, next_rank_diff: i8, next_file_diff: i8) -> Option<Position> {
    let next_file = position.file.advance(next_file_diff as i32)?;
    let next_rank = position.rank.advance(next_rank_diff as i32)?;
    let next_position = Position::new(next_file, next_rank);
    Some(next_position)
}

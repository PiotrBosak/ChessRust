use crate::{Board, Color, File, Move, MoveType, Position, Tile, ToNumber};
use crate::logic::rules::knight_and_king_rules;
use crate::logic::rules::check_mate_rules;
use std::cmp;

pub fn possible_moves(board: &Board, position: Position, color: &Color) -> Vec<Move> {
    let mut moves = vec![
        castling_move(board, position, color, false),
        castling_move(board, position, color, true),
    ]
        .into_iter()
        .filter_map(|x| x)
        .collect::<Vec<Move>>();
    moves.append(&mut knight_and_king_rules::possible_moves(position, board, combinations()));
    moves
}

pub fn possible_captures(board: &Board, position: Position, color: &Color) -> Vec<Move> {
    knight_and_king_rules::possible_captures(position, board, color, combinations())
}

fn castling_move(board: &Board, position: Position, color: &Color, is_left_rook: bool) -> Option<Move> {
    let rook_file = if is_left_rook { File::A } else { File::H };
    let new_king_file = if is_left_rook { File::B } else { File::G };
    let king_tile = Some(board.tile_at(&position)).filter(|tile| tile.has_moved)?;
    let rook_tile = find_rook_tile(board, &position, rook_file)?;
    if are_tiles_clear(board, &king_tile.position, &rook_tile.position) &&
        !check_mate_rules::is_king_checked(board, color) {
        let new_king_position = Position::new(new_king_file, rook_tile.position.rank);
        Some(Move::new(position, new_king_position, MoveType::Castling))
    } else {
        None
    }
}

fn find_rook_tile<'a>(board: &'a Board, position: &Position, rook_file: File) -> Option<&'a Tile> {
    let tile = board.tile_at(position);
    let rook_position = Position::new(rook_file, tile.position.rank);
    let board_tile = board.tile_at(&rook_position);
    if board_tile.has_moved {
        None
    } else {
        Some(board_tile)
    }
}

fn are_tiles_clear(board: &Board, from: &Position, to: &Position) -> bool {
    board.tiles
        .iter()
        .filter(|(pos, _)| pos.rank == from.rank && is_file_between(&pos.file, from, to))
        .all(|(_, tile)| tile.is_empty())
}

fn is_file_between(file: &File, from: &Position, to: &Position) -> bool {
    let min = cmp::min(from.file.to_number(), to.file.to_number());
    let max = cmp::max(from.file.to_number(), to.file.to_number());
    ((min + 1)..max).contains(&file.to_number())
}


fn combinations() -> Vec<(i32, i32)> {
    let mut combinations = ((-1)..2).into_iter()
        .flat_map(|x| {
            ((-1)..2).into_iter()
                .map(move |y| (x, y))
        })
        .filter(|p| p != &(0, 0))
        .collect::<Vec<(i32, i32)>>();
    combinations.sort();
    combinations.dedup();
    combinations
}

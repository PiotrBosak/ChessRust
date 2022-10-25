use crate::{
    Advance, Board, Color, Move, MoveType, PieceType, Position,
    Rank::{Seven, Two},
    Tile,
};

pub fn possible_moves(board: &Board, position: &Position, color: &Color) -> Vec<Move> {
    let mut moves = vec![];
    moves.push(one_tile_move(board, position, color));
    moves.push(two_tile_move(board, position, color));
    moves.into_iter().filter_map(|x| x).collect()
}

pub fn possible_captures(board: &Board, position: &Position, color: &Color) -> Vec<Move> {
    let mut moves = vec![];
    moves.push(le_passant(board, position, color));
    moves.push(left_attack(board, position, color));
    moves.push(right_attack(board, position, color));
    moves.into_iter().filter_map(|x| x).collect()
}

fn one_tile_move(board: &Board, position: &Position, color: &Color) -> Option<Move> {
    move_pawn(board, position, color, 1, MoveType::Move)
}

fn two_tile_move(board: &Board, position: &Position, color: &Color) -> Option<Move> {
    if !board.tile_at(&position).has_moved {
        move_pawn(board, position, color, 2, MoveType::TwoTilePawnMove)
    } else {
        None
    }
}

fn move_pawn(
    board: &Board,
    position: &Position,
    color: &Color,
    amount: i32,
    move_type: MoveType,
) -> Option<Move> {
    let tile = board.tile_at(position);
    let next_tile = next_pawn_tile(board, position, color, amount);
    next_tile
        .filter(|tile| tile.current_piece.is_none())
        .map(|next| Move {
            from: tile.position.clone(),
            to: next.position.clone(),
            move_type,
        })
}

fn next_pawn_tile<'a>(
    board: &'a Board,
    position: &Position,
    color: &Color,
    amount: i32,
) -> Option<&'a Tile> {
    match color {
        Color::White => position
            .rank
            .advance(amount)
            .map(|rank| Position::new(position.file, rank))
            .map(|pos| board.tile_at(&pos)),
        Color::Black => position
            .rank
            .advance(-amount)
            .map(|rank| Position::new(position.file, rank))
            .map(|pos| board.tile_at(&pos)),
    }
}

fn le_passant(board: &Board, position: &Position, color: &Color) -> Option<Move> {
    le_passant_side(board, position, color, true).or(le_passant_side(board, position, color, false))
}

fn le_passant_side(
    board: &Board,
    position: &Position,
    color: &Color,
    is_left: bool,
) -> Option<Move> {
    let diff: i32 = if is_left { -1 } else { 1 };
    let attacked_position = board.previous_move.as_ref().and_then(|move_| {
        position
            .file
            .advance(-diff)
            .map(|file| Position::new(file, position.rank))
            .filter(|pos| check_for_le_passant(board, pos, color, &move_))
    });
    let rank_difference = if color == &Color::White { 1 } else { -1 };
    let position_to_move = attacked_position.and_then(|pos| {
        pos.rank
            .advance(rank_difference)
            .map(|rank| Position::new(pos.file, rank))
    });
    position_to_move.map(|pos| Move {
        from: position.clone(),
        to: pos,
        move_type: MoveType::LePassant,
    })
}

fn check_for_le_passant(
    board: &Board,
    attacked_position: &Position,
    attacking_color: &Color,
    move_: &Move,
) -> bool {
    let starting_enemy_rank = if attacking_color == &Color::White {
        Seven
    } else {
        Two
    };
    let is_row_the_same = move_.to.rank == attacked_position.rank;
    let is_previous_start_correct = move_.from.rank == starting_enemy_rank;
    let is_column_the_same = move_.to.file == attacked_position.file;
    let is_enemy_pawn = board
        .tile_at(attacked_position)
        .current_piece
        .map(|piece| piece.piece_type == PieceType::Pawn && piece.color != *attacking_color)
        .unwrap_or(false);
    is_row_the_same && is_previous_start_correct && is_column_the_same && is_enemy_pawn
}

fn left_attack(board: &Board, position: &Position, color: &Color) -> Option<Move> {
    let attacked_tile = left_attack_tile(board, position, color)?;
    side_attack(position, color, attacked_tile)
}

fn right_attack(board: &Board, position: &Position, color: &Color) -> Option<Move> {
    let attacked_tile = right_attack_tile(board, position, color)?;
    side_attack(position, color, attacked_tile)
}

fn side_attack(position: &Position, color: &Color, attacked_tile: &Tile) -> Option<Move> {
    match attacked_tile
        .current_piece
        .filter(|piece| piece.color != *color)
    {
        Some(_) => Some(Move {
            from: position.clone(),
            to: attacked_tile.position.clone(),
            move_type: MoveType::Capture,
        }),
        None => None,
    }
}

fn left_attack_tile<'a>(
    board: &'a Board,
    position: &'a Position,
    color: &Color,
) -> Option<&'a Tile> {
    match color {
        Color::White => {
            let rank = position.rank.advance(1)?;
            let file = position.file.advance(-1)?;
            Some(board.tile_at(&Position::new(file, rank)))
        }
        Color::Black => {
            let rank = position.rank.advance(-1)?;
            let file = position.file.advance(-1)?;
            Some(board.tile_at(&Position::new(file, rank)))
        }
    }
}

fn right_attack_tile<'a>(
    board: &'a Board,
    position: &'a Position,
    color: &Color,
) -> Option<&'a Tile> {
    match color {
        Color::White => {
            let rank = position.rank.advance(1)?;
            let file = position.file.advance(1)?;
            Some(board.tile_at(&Position::new(file, rank)))
        }
        Color::Black => {
            let rank = position.rank.advance(1)?;
            let file = position.file.advance(-1)?;
            Some(board.tile_at(&Position::new(file, rank)))
        }
    }
}

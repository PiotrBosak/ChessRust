use crate::{Board, Color, Move, Piece, PieceType, Tile};

pub fn is_king_checked(board: &Board, king_color: &Color) -> bool {
    let king_tile = board
        .tiles
        .iter()
        .find(|(_, tile)| match tile {
            Tile {
                current_piece:
                    Some(Piece {
                        piece_type: PieceType::King,
                        color,
                    }),
                ..
            } => color == king_color,
            _ => false,
        })
        .map(|(_, tile)| tile)
        .expect("King should never be captured");

    let possible_attacks = board
        .tiles
        .values()
        .filter(|tile| is_enemy_color(tile, king_color))
        .flat_map(|tile| {
            let mut moves = board.possible_moves(&tile.position);
            moves.append(&mut board.possible_captures(&tile.position));
            moves
        })
        .collect::<Vec<Move>>();

    possible_attacks
        .into_iter()
        .find(|m| m.to == king_tile.position)
        .is_some()
}
pub fn is_king_mated(board: &Board, king_color: &Color) -> bool {
    is_king_checked(board, king_color) && !can_be_defended(board, king_color)
}

fn can_be_defended(board: &Board, king_color: &Color) -> bool {
    let possible_defense_moves = board
        .tiles
        .values()
        .into_iter()
        .filter(|t| t.current_piece.filter(|p| &p.color == king_color).is_some())
        .flat_map(|t| board.possible_valid_moves(&t.position))
        .collect::<Vec<Move>>();

    possible_defense_moves
        .into_iter()
        .find(|m| {
            board
                .make_move(&m.from, &m.to)
                .filter(|b| !is_king_checked(b, king_color))
                .is_some()
        })
        .is_some()
}

fn is_enemy_color(tile: &Tile, king_color: &Color) -> bool {
    tile.current_piece
        .filter(|piece| piece.color != *king_color)
        .is_some()
}

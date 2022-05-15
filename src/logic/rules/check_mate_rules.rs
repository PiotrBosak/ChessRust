use crate::{Board, Color, Piece, PieceType, Position, Tile};

pub fn is_king_checked(board: &Board, king_color: &Color) -> bool {
    let king_tile = board.tiles
        .iter()
        .find(|(_, tile)| match tile {
            Tile { current_piece: Some(Piece { piece_type: PieceType::King, color }), .. } => color == king_color,
            _ => false,
        })
        .map(|(_, tile)| tile)
        .expect("King should never be captured");

    //todo unfinished
    true
}
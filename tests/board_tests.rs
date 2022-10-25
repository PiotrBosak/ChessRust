mod board_tests {
    use chessTUI::Board;
    use chessTUI::Rank;
    use chessTUI::{Color, File, Piece, PieceType, Position, Tile};
    use std::collections::HashMap;

    #[test]
    fn board_has_64_tiles() {
        let board = Board::new();
        assert_eq!(board.tiles.keys().len(), 64);
    }

    #[test]
    fn board_has_32_tiles_with_pieces() {
        let board = Board::new();
        let tiles_with_pieces = board
            .tiles
            .into_iter()
            .filter(|(_, tile)| tile.current_piece.is_some())
            .map(|(_, tile)| tile)
            .collect::<Vec<Tile>>();
        assert_eq!(tiles_with_pieces.len(), 32);
    }

    #[test]
    fn board_has_32_empty_tiles() {
        let board = Board::new();
        let empty_tiles = board
            .tiles
            .into_iter()
            .filter(|(_, tile)| tile.current_piece.is_none())
            .map(|(_, tile)| tile)
            .collect::<Vec<Tile>>();
        assert_eq!(empty_tiles.len(), 32);
    }

    #[test]
    fn board_has_8_white_pawns() {
        let board = Board::new();
        let white_pawns = board
            .tiles
            .into_iter()
            .filter_map(|(_, tile)| {
                tile.current_piece.filter(|piece| {
                    piece.piece_type == PieceType::Pawn && piece.color == Color::White
                })
            })
            .collect::<Vec<Piece>>();
        assert_eq!(white_pawns.len(), 8);
    }
}

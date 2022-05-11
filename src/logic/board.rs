use std::collections::HashMap;
use crate::logic::File;
use crate::logic::{Move, Piece, Position, Rank, Tile, Advance, PieceType, Color};

pub struct Board {
    pub tiles: HashMap<Position, Tile>,
    previous_move: Option<Move>,
}

impl Board {
    pub fn tile_at(&self, position: Position) -> &Tile {
        self.tiles
            .get(&position)
            .expect("No tile at position, should never happen")
    }
    pub fn update_board(&mut self, move_: Move, new_tile: Tile) {
        self.tiles.insert(new_tile.position, new_tile);
        self.previous_move = Some(move_);
    }
    pub fn new() -> Board {
        Board {
            tiles: Board::make_tiles(),
            previous_move: None,
        }
    }
    fn make_tiles() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.extend(Board::create_tiles(
            Rank::Two,
            Some(Piece::new(PieceType::Pawn, Color::White)))
        );
        tiles.extend(Board::create_tiles(Rank::Three, None));
        tiles.extend(Board::create_tiles(Rank::Four, None));
        tiles.extend(Board::create_tiles(Rank::Five, None));
        tiles.extend(Board::create_tiles(Rank::Six, None));
        tiles.extend(Board::create_tiles(
            Rank::Seven,
            Some(Piece::new(PieceType::Pawn, Color::Black)))
        );
        tiles.extend(Board::create_tiles_with_white_bishops());
        tiles.extend(Board::create_tiles_with_black_bishops());
        tiles.extend(Board::create_tiles_with_white_knights());
        tiles.extend(Board::create_tiles_with_black_knights());
        tiles.extend(Board::create_tiles_with_black_rooks());
        tiles.extend(Board::create_tiles_with_white_rooks());
        tiles.extend(Board::create_tile_with_black_king());
        tiles.extend(Board::create_tile_with_black_queen());
        tiles.extend(Board::create_tile_with_white_king());
        tiles.extend(Board::create_tile_with_white_queen());

        tiles
    }

    fn create_tiles_with_black_bishops() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.insert(Position::new(File::C, Rank::Eight), Tile::with_piece(Position::new(File::C, Rank::Eight), Piece::new(PieceType::Bishop, Color::Black)));
        tiles.insert(Position::new(File::F, Rank::Eight), Tile::with_piece(Position::new(File::F, Rank::Eight), Piece::new(PieceType::Bishop, Color::Black)));
        tiles
    }
    fn create_tiles_with_black_rooks() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.insert(Position::new(File::A, Rank::Eight), Tile::with_piece(Position::new(File::H, Rank::Eight), Piece::new(PieceType::Rook, Color::Black)));
        tiles.insert(Position::new(File::H, Rank::Eight), Tile::with_piece(Position::new(File::H, Rank::Eight), Piece::new(PieceType::Rook, Color::Black)));
        tiles
    }
    fn create_tiles_with_black_knights() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.insert(Position::new(File::B, Rank::Eight), Tile::with_piece(Position::new(File::B, Rank::Eight), Piece::new(PieceType::Knight, Color::Black)));
        tiles.insert(Position::new(File::G, Rank::Eight), Tile::with_piece(Position::new(File::G, Rank::Eight), Piece::new(PieceType::Knight, Color::Black)));
        tiles
    }
    fn create_tile_with_black_king() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.insert(Position::new(File::E, Rank::Eight), Tile::with_piece(Position::new(File::E, Rank::Eight), Piece::new(PieceType::King, Color::Black)));
        tiles
    }
    fn create_tile_with_black_queen() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.insert(Position::new(File::D, Rank::Eight), Tile::with_piece(Position::new(File::D, Rank::Eight), Piece::new(PieceType::Queen, Color::Black)));
        tiles
    }
    fn create_tiles_with_white_bishops() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.insert(Position::new(File::C, Rank::One), Tile::with_piece(Position::new(File::C, Rank::One), Piece::new(PieceType::Bishop, Color::White)));
        tiles.insert(Position::new(File::F, Rank::One), Tile::with_piece(Position::new(File::F, Rank::One), Piece::new(PieceType::Bishop, Color::White)));
        tiles
    }
    fn create_tiles_with_white_rooks() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.insert(Position::new(File::A, Rank::One), Tile::with_piece(Position::new(File::A, Rank::One), Piece::new(PieceType::Rook, Color::White)));
        tiles.insert(Position::new(File::H, Rank::One), Tile::with_piece(Position::new(File::H, Rank::One), Piece::new(PieceType::Rook, Color::White)));
        tiles
    }
    fn create_tiles_with_white_knights() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.insert(Position::new(File::B, Rank::One), Tile::with_piece(Position::new(File::B, Rank::One), Piece::new(PieceType::Knight, Color::White)));
        tiles.insert(Position::new(File::G, Rank::One), Tile::with_piece(Position::new(File::G, Rank::One), Piece::new(PieceType::Knight, Color::White)));
        tiles
    }

    fn create_tile_with_white_king() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.insert(Position::new(File::E, Rank::One), Tile::with_piece(Position::new(File::E, Rank::One), Piece::new(PieceType::King, Color::White)));
        tiles
    }
    fn create_tile_with_white_queen() -> HashMap<Position, Tile> {
        let mut tiles = HashMap::new();
        tiles.insert(Position::new(File::D, Rank::One), Tile::with_piece(Position::new(File::D, Rank::One), Piece::new(PieceType::Queen, Color::White)));
        tiles
    }

    // fn create_tiles_with_black_pawns() -> HashMap<Position, Tile> {}
    fn create_tiles(rank: Rank, piece: Option<Piece>) -> HashMap<Position, Tile> {
        let mut current_file = Some(File::A);
        let mut tiles = HashMap::new();
        while let Some(file) = current_file {
            let position = Position::new(file, rank);
            tiles.insert(position, Tile {
                position,
                current_piece: piece,
                has_moved: false,
            });
            current_file = file.advance(1);
        }

        tiles
    }
}

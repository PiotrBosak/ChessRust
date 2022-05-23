use std::collections::HashMap;
use std::num::IntErrorKind::PosOverflow;
use crate::logic::File;
use crate::logic::{Move, Piece, Position, Rank, Tile, Advance, PieceType, Color};
use crate::logic::rules;
use crate::MoveType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub tiles: HashMap<Position, Tile>,
    pub previous_move: Option<Move>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: Board::make_tiles(),
            previous_move: None,
        }
    }

    pub fn tile_at(&self, position: &Position) -> &Tile {
        self.tiles
            .get(position)
            .expect("No tile at position, should never happen")
    }

    pub fn make_move(&self, move_: &Move) -> Option<Board> {
        match &move_.move_type {
            MoveType::Capture | MoveType::Move | MoveType::TwoTilePawnMove =>
                self.make_normal_move(&move_.from, &move_.to),
            MoveType::Castling =>
                self.make_castling(&move_.from, &move_.to),
            MoveType::LePassant =>
                self.make_le_passant(&move_.from, &move_.to)
        }
    }

    fn make_normal_move(&self, from: &Position, to: &Position) -> Option<Board> {
        let mut new_board = self.clone();
        let tile_from = new_board.tile_at(from);
        let piece_from = tile_from.current_piece?;
        let new_tile_to = Tile::with_piece(*to, piece_from);
        let new_tile_from = Tile::empty(*from);
        new_board.tiles.insert(*to, new_tile_to.mark_moved());
        new_board.tiles.insert(*from, new_tile_from.mark_moved());
        if !rules::is_king_checked(&new_board, &piece_from.color) {
            Some(new_board)
        } else {
            None
        }
    }

    fn make_castling(&self, from: &Position, to: &Position) -> Option<Board> {
        let mut new_board = self.clone();
        let king_tile = new_board.tile_at(from);
        let king_piece = king_tile.current_piece?;
        let rook_file = if from.file == File::B { File::A } else { File::H };
        let new_rook_file = if from.file == File::B { File::C } else { File::F };
        let rook_position = Position::new(rook_file, from.rank);
        let new_rook_position = Position::new(new_rook_file, from.rank);
        let rook_tile = new_board.tile_at(&rook_position);
        let rook_piece = rook_tile.current_piece?;

        new_board.tiles.insert(*from, Tile::empty(*from).mark_moved());
        new_board.tiles.insert(*to, Tile::with_piece(*to, king_piece).mark_moved());
        new_board.tiles.insert(rook_position, Tile::empty(rook_position));
        new_board.tiles.insert(new_rook_position, Tile::with_piece(new_rook_position, rook_piece));
        if !rules::is_king_checked(&new_board, &king_piece.color) {
            Some(new_board)
        } else {
            None
        }
    }
    fn make_le_passant(&self, from: &Position, to: &Position) -> Option<Board> {
        let mut new_board = self.clone();
        let from_tile = new_board.tile_at(from);
        let attacker_piece = from_tile.current_piece?;
        let difference = if attacker_piece.color == Color::White {-1} else {1};
        let captured_rank = to.rank.advance(difference)?;
        let captured_position = Position::new(to.file, captured_rank);
        let tile_from = Tile::empty(*from);
        let tile_to = Tile::with_piece(*to, attacker_piece);
        let captured_tile = Tile::empty(captured_position);
        new_board.tiles.insert(*from, tile_from.mark_moved());
        new_board.tiles.insert(*to, tile_to.mark_moved());
        new_board.tiles.insert(captured_position, captured_tile.mark_moved());
        if !rules::is_king_checked(&new_board, &attacker_piece.color) {
            Some(new_board)
        } else {
            None
        }
    }

    pub fn possible_moves(&self, position: &Position) -> Vec<Move> {
        let piece = self.tile_at(position).current_piece;
        piece
            .map(|p| rules::possible_moves(&self, position, &p))
            .unwrap_or(vec![])
    }
    pub fn possible_valid_moves(&self, position: &Position) -> Vec<Move> {
        let mut moves = self.possible_moves(position);
        moves.append(&mut self.possible_captures(position));
        moves.into_iter()
            .filter(|m| self.make_move(&m).is_some())
            .collect()
    }
    pub fn possible_captures(&self, position: &Position) -> Vec<Move> {
        let piece = self.tile_at(position).current_piece;
        piece
            .map(|p| rules::possible_captures(&self, position, &p))
            .unwrap_or(vec![])
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

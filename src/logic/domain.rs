use std::fmt::{Display, Formatter};
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

trait UsizeIso
    where
        Self: Sized,
{
    fn to_usize(&self) -> i32;
    fn from_usize(u: i32) -> Option<Self>;
    fn from_usize_unsafe(u: i32) -> Self {
        UsizeIso::from_usize(u).expect("Illegal operation")
    }
}

pub trait Advance
    where
        Self: Sized,
{
    fn advance(&self, n: i32) -> Option<Self>;
    fn advance_unsafe(&self, n: i32) -> Self {
        self.advance(n).expect("invalid position")
    }
}

impl<T> Advance for T
    where
        T: UsizeIso + Sized,
{
    fn advance(&self, n: i32) -> Option<Self> {
        UsizeIso::from_usize(self.to_usize() + n)
    }
}

pub trait ToNumber
    where Self: Sized {
    fn to_number(&self) -> i32;
}

impl<T> ToNumber for T
    where T: UsizeIso + Sized {
    fn to_number(&self) -> i32 {
        self.to_usize()
    }
}

impl UsizeIso for File {
    fn to_usize(&self) -> i32 {
        match self {
            File::A => 1,
            File::B => 2,
            File::C => 3,
            File::D => 4,
            File::E => 5,
            File::F => 6,
            File::G => 7,
            File::H => 8,
        }
    }

    fn from_usize(u: i32) -> Option<Self> {
        match u {
            1 => Some(File::A),
            2 => Some(File::B),
            3 => Some(File::C),
            4 => Some(File::D),
            5 => Some(File::E),
            6 => Some(File::F),
            7 => Some(File::G),
            8 => Some(File::H),
            _ => None,
        }
    }
}

impl UsizeIso for Rank {
    fn to_usize(&self) -> i32 {
        match self {
            Rank::One => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
        }
    }

    fn from_usize(u: i32) -> Option<Self> {
        match u {
            1 => Some(Rank::One),
            2 => Some(Rank::Two),
            3 => Some(Rank::Three),
            4 => Some(Rank::Four),
            5 => Some(Rank::Five),
            6 => Some(Rank::Six),
            7 => Some(Rank::Seven),
            8 => Some(Rank::Eight),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub file: File,
    pub rank: Rank,
}

impl Position {
    pub fn new(file: File, rank: Rank) -> Self {
        Position { file, rank }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveType {
    Capture,
    Move,
    TwoTilePawnMove,
    Castling,
    LePassant,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub move_type: MoveType,
}

impl Move {
    pub fn new(from: Position, to: Position, move_type: MoveType) -> Self {
        Move { from, to, move_type }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tile {
    pub position: Position,
    pub current_piece: Option<Piece>,
    pub has_moved: bool,
}

impl Tile {
    pub fn with_piece(position: Position, piece: Piece) -> Tile {
        Tile {
            position,
            current_piece: Some(piece),
            has_moved: false,
        }
    }

    pub fn mark_moved(mut self) -> Self {
        self.has_moved = true;
        self
    }

    pub fn empty(position: Position) -> Tile {
        Tile {
            position,
            current_piece: None,
            has_moved: false,
        }
    }

    pub fn has_piece(&self) -> bool {
        self.current_piece.is_some()
    }
    pub fn is_empty(&self) -> bool {
        self.current_piece.is_none()
    }
}

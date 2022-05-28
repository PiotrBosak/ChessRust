use crate::{Board, Move, Position};

//lepiej, żeby frontend nie używal boarda bezposrenio tylko zawsze posrednio przez game
//a current board i history bylo prywatne
pub struct Game {
    current_board: Board,
    history: Vec<Board>,
    turn: Turn,
}

//todo warto dodac funkcje ktora pokaze faktycznie wszystkie mozliwe ruchy
//tzn ze jezeli niby jest mozliwy ale wtedy by odslonil króla to jednak nie można
//moze w sumie warto zeby to zwracalo nową wersje gry, nawet jezeli to bedzie moved
//wtedy zwroci sie Option<Game> i bedzie wszystko jasne
impl Game {
    pub fn possible_moves(&self, position: &Position) -> Vec<Move> {
        self.current_board.possible_valid_moves(position)
    }


    // pub fn make_move(&mut self, from: &Position, to: &Position) {
    //     let possible_moves = &self.current_board.possible_moves(from);
    // }
}

pub enum Turn {
    White,
    Black,
}
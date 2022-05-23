mod pawn_rules {
    use chessTUI::{Board, File, Position, Rank};

    #[test]
    pub fn pawn_on_a2_has_2_moves() {
        let board = Board::new();
        let position = Position::new(File::A, Rank::Two);
        assert_eq!(2, board.possible_moves(&position).len())
    }

    #[test]
    pub fn after_move_to_a3_pawn_has_one_move() {
        let board = Board::new();
        let from = Position::new(File::A, Rank::Two);
        let to = Position::new(File::A, Rank::Three);

    }
}
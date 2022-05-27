mod queen_rules {
    use chessTUI::{Board, File, Position, Rank, Tile};

    #[test]
    pub fn queen_should_have_0_moves_at_start() {
        let board = Board::new();
        assert_eq!(0, board.possible_valid_moves(&Position::new(File::D, Rank::One)).len())
    }
}
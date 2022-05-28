mod queen_rules {
    use chessTUI::{Board, File, Position, Rank, Tile};

    #[test]
    pub fn queen_should_have_0_moves_at_start() {
        let board = Board::new();
        assert_eq!(0, board.possible_valid_moves(&Position::new(File::D, Rank::One)).len())
    }

    #[test]
    pub fn queen_should_have_16_moves_after_moving_to_d3() -> Result<(), String>{
        let board = Board::new();
        let after_pawn = board
            .make_move(&Position::new(File::D, Rank::Two), &Position::new(File::D, Rank::Four))
            .ok_or("error 1")?;
        let after_queen =
            after_pawn
                .make_move(&Position::new(File::D, Rank::One), &Position::new(File::D, Rank::Three))
                .ok_or("error 2")?;
        assert_eq!(16, after_queen.possible_valid_moves(&Position::new(File::D, Rank::Three)).len());
        Ok(())
    }
}
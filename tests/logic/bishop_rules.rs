mod bishop_rules {
    use chessTUI::{Board, File, Position, Rank};

    #[test]
    pub fn bishop_on_c1_has_0_moves_at_start() {
        let board = Board::new();
        assert_eq!(0, board.possible_valid_moves(&Position::new(File::C, Rank::One)).len());
    }
    #[test]
    pub fn after_bishop_on_c1_it_has_9_moves() -> Result<(), String>{
        let board = Board::new();
        let after_pawn =
            board
            .make_move(&Position::new(File::D, Rank::Two), &Position::new(File::D, Rank::Four))
            .ok_or("error 1")?;
        let after_bishop =
            after_pawn
                .make_move(&Position::new(File::C, Rank::One), &Position::new(File::F, Rank::Four))
                .ok_or("error 2")?;
        assert_eq!(9, after_bishop.possible_valid_moves(&Position::new(File::F, Rank::Four)).len());

        Ok(())
    }
}
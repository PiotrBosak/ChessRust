mod rook_rules {
    use chessTUI::{Board, File, Position, Rank};

    #[test]
    pub fn rook_on_a1_should_have_0_moves_at_start() {
        let board = Board::new();
        let position = Position::new(File::A, Rank::One);
        assert_eq!(0, board.possible_valid_moves(&position).len())
    }
    #[test]
    pub fn after_rook_moves_to_a3_it_has_9_moves() -> Result<(), String> {
        let board = Board::new();
        let after_pawn =
        board
            .make_move(&Position::new(File::A, Rank::Two), &Position::new(File::A, Rank::Four))
            .ok_or("error 1")?;
        let after_first =
            after_pawn
                .make_move(&Position::new(File::A, Rank::One), &Position::new(File::A, Rank::Three))
                .ok_or("error 2")?;
        assert_eq!(9, after_first.possible_valid_moves(&Position::new(File::A, Rank::Three)).len());
        Ok(())
    }

    #[test]
    pub fn after_capturing_d7_rook_has_7_moves() -> Result<(), String> {
        let board = Board::new();
        let after_pawn =
            board
                .make_move(&Position::new(File::A, Rank::Two), &Position::new(File::A, Rank::Four))
                .ok_or("error 1")?;
        let after_first =
            after_pawn
                .make_move(&Position::new(File::A, Rank::One), &Position::new(File::A, Rank::Three))
                .ok_or("error 2")?;
        let after_second =
            after_first
                .make_move(&Position::new(File::A, Rank::Three), &Position::new(File::D, Rank::Three))
                .ok_or("error 3")?;

        println!("{:?}",after_second.possible_captures(&Position::new(File::D, Rank::Three)));
        let after_third =
            after_second
                .make_move(&Position::new(File::D, Rank::Three), &Position::new(File::D, Rank::Seven))
                .ok_or("error 4")?;
        assert_eq!(7, after_third.possible_valid_moves(&Position::new(File::D, Rank::Seven)).len());
        Ok(())
    }
}
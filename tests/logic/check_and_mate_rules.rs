mod check_and_mate_rules {
    use chessTUI::{Board, Color, File, is_king_mated, Position, Rank};
    use chessTUI::is_king_checked;

    #[test]
    pub fn white_king_should_be_checked_by_queen_on_a_5() -> Result<(), String> {
        let board = Board::new();
        let after_white_pawn = board
            .make_move(&Position::new(File::D, Rank::Two), &Position::new(File::D, Rank::Four))
            .ok_or("error 1")?;
        let after_black_pawn =
            after_white_pawn
                .make_move(&Position::new(File::C, Rank::Seven), &Position::new(File::C, Rank::Five))
                .ok_or("error 2")?;
        let after_queen =
            after_black_pawn
                .make_move(&Position::new(File::D, Rank::Eight), &Position::new(File::A, Rank::Five))
                .ok_or("error 3")?;

        assert!(is_king_checked(&after_queen, &Color::White));
        assert!(!is_king_mated(&after_queen, &Color::White));
        Ok(())
    }
    #[test]
    pub fn white_king_is_mated_by_fools_mate() -> Result<(), String> {

        let board = Board::new();
        let first = board
            .make_move(&Position::new(File::F, Rank::Two), &Position::new(File::F, Rank::Three))
            .ok_or("error 1")?;
        let second =
            first
                .make_move(&Position::new(File::E, Rank::Seven), &Position::new(File::E, Rank::Five))
                .ok_or("error 2")?;
        let third =
            second
                .make_move(&Position::new(File::G, Rank::Two), &Position::new(File::G, Rank::Four))
                .ok_or("error 3")?;

        let fourth =
            third
                .make_move(&Position::new(File::D, Rank::Eight), &Position::new(File::H, Rank::Four))
                .ok_or("error 4")?;

        assert!(is_king_checked(&fourth, &Color::White));
        assert!(is_king_mated(&fourth, &Color::White));
        Ok(())
    }
}
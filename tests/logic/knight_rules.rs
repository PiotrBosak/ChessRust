mod knight_rules {
    use chessTUI::{Board, File, Position, Rank};

    #[test]
    pub fn knight_on_b8_should_have_2_moves() {
        let board = Board::new();
        assert_eq!(2, board.possible_valid_moves(&Position::new(File::B, Rank::Eight)).len())
    }
    #[test]
    pub fn knight_on_g1_can_capture_pawn_on_f7() -> Result<(), String> {
        let board = Board::new();
        let after_first =
        board
            .make_move(&Position::new(File::G, Rank::One), &Position::new(File::F, Rank::Three))
            .ok_or("error 1")?;
        let after_second =
            after_first
                .make_move(&Position::new(File::F, Rank::Three), &Position::new(File::E, Rank::Five))
                .ok_or("error 2")?;
        let after_third =
            after_second
                .make_move(&Position::new(File::E, Rank::Five), &Position::new(File::F, Rank::Seven))
                .ok_or("error 3")?;
        let number_of_pieces = after_third.tiles
            .values()
            .into_iter()
            .filter_map(|t| t.current_piece)
            .count();
        assert_eq!(number_of_pieces, 31);
        assert_eq!(after_third.possible_valid_moves(&Position::new(File::F, Rank::Seven)).len(), 6);
        assert_eq!(after_third.possible_captures(&Position::new(File::F, Rank::Seven)).len(), 2);
        Ok(())
    }
}
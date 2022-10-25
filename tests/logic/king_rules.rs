mod king_rules {
    use chessTUI::{Board, File, Position, Rank};

    #[test]
    pub fn king_on_e1_has_0_moves() {
        let board = Board::new();
        assert_eq!(
            0,
            board
                .possible_valid_moves(&Position::new(File::E, Rank::One))
                .len()
        );
    }

    #[test]
    pub fn king_on_e1_can_capture_pawn_on_f7_and_have_5_moves() -> Result<(), String> {
        let board = Board::new();
        let after_pawn = board
            .make_move(
                &Position::new(File::E, Rank::Two),
                &Position::new(File::E, Rank::Four),
            )
            .ok_or("error 1")?;

        let after_first = after_pawn
            .make_move(
                &Position::new(File::E, Rank::One),
                &Position::new(File::E, Rank::Two),
            )
            .ok_or("error 2")?;
        let after_second = after_first
            .make_move(
                &Position::new(File::E, Rank::Two),
                &Position::new(File::E, Rank::Three),
            )
            .ok_or("error 3")?;
        let after_third = after_second
            .make_move(
                &Position::new(File::E, Rank::Three),
                &Position::new(File::F, Rank::Four),
            )
            .ok_or("error 4")?;
        let after_fourth = after_third
            .make_move(
                &Position::new(File::F, Rank::Four),
                &Position::new(File::F, Rank::Five),
            )
            .ok_or("error 5")?;
        let after_fifth = after_fourth
            .make_move(
                &Position::new(File::G, Rank::Seven),
                &Position::new(File::G, Rank::Five),
            )
            .ok_or("error 6")?;
        let after_sixth = after_fifth
            .make_move(
                &Position::new(File::F, Rank::Five),
                &Position::new(File::G, Rank::Five),
            )
            .ok_or("error 7")?;

        let number_of_pieces = after_sixth
            .tiles
            .values()
            .into_iter()
            .filter_map(|t| t.current_piece)
            .count();
        assert_eq!(number_of_pieces, 31);
        assert_eq!(
            after_sixth
                .possible_valid_moves(&Position::new(File::G, Rank::Five))
                .len(),
            5
        );
        assert_eq!(
            after_sixth
                .possible_captures(&Position::new(File::G, Rank::Five))
                .len(),
            0
        );
        Ok(())
    }
}

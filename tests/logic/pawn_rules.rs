use std::vec::Vec;

mod pawn_rules {
    use chessTUI::{Board, File, Position, Rank, Tile};

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
        let to = Position::new(File::A, Rank::Four);
        let board_after_move = board.make_move(&from, &to);
        assert_eq!(1, board_after_move
            .map(|board| board.possible_valid_moves(&to).len())
            .expect("Invalid move")
        )
    }

    #[test]
    pub fn pawn_on_a2_captures_pawn_on_b_7() -> Result<(), String> {
        let board = Board::new();
        let start_position: Position = Position::new(File::A, Rank::Two);
        let first_pos = Position::new(File::A, Rank::Four);
        let second_pos = Position::new(File::A, Rank::Five);
        let third_pos = Position::new(File::A, Rank::Six);
        let fourth_pos = Position::new(File::B, Rank::Seven);
        let after_first = board.make_move(&start_position, &first_pos).ok_or_else(|| "error")?;
        let after_second = after_first.make_move(&first_pos, &second_pos).ok_or_else(|| "error")?;
        let after_third = after_second.make_move(&second_pos, &third_pos).ok_or_else(|| "error")?;
        let after_fourth = after_third.make_move(&third_pos, &fourth_pos).ok_or_else(|| "error")?;
        let number_of_pieces = after_fourth.tiles
            .values()
            .into_iter()
            .filter_map(|t| t.current_piece)
            .count();
        assert_eq!(number_of_pieces, 31);
        assert_eq!(after_fourth.possible_valid_moves(&fourth_pos).len(), 2);
        Ok(())
    }

    #[test]
    pub fn pawn_on_a2_can_make_le_passant() -> Result<(), String> {
        let board = Board::new();
        let first =
            board
                .make_move(&Position::new(File::D, Rank::Two), &Position::new(File::D, Rank::Four))
                .ok_or_else(|| "error 1")?;
        let second =
            first
                .make_move(&Position::new(File::D, Rank::Four), &Position::new(File::D, Rank::Five))
                .ok_or_else(|| "error 2")?;

        let third =
        second
            .make_move(&Position::new(File::E, Rank::Seven), &Position::new(File::E, Rank::Five))
            .ok_or_else(|| "error 3")?;
        assert_eq!(2,
                   third.possible_valid_moves(&Position::new(File::D, Rank::Five)).len());
        let after_le_passant =
            third
                .make_move(&Position::new(File::D, Rank::Five), &Position::new(File::E, Rank::Six))
                .ok_or_else(|| "error 4")?;

        let number_of_pieces = after_le_passant.tiles
            .values()
            .into_iter()
            .filter_map(|t| t.current_piece)
            .count();
        assert_eq!(number_of_pieces, 31);
        Ok(())
    }
    #[test]
    pub fn pawn_on_a2_should_not_be_able_to_le_passant() -> Result<(), String> {

        let board = Board::new();
        let first =
            board
                .make_move(&Position::new(File::D, Rank::Two), &Position::new(File::D, Rank::Four))
                .ok_or_else(|| "error 1")?;
        let second =
            first
                .make_move(&Position::new(File::D, Rank::Four), &Position::new(File::D, Rank::Five))
                .ok_or_else(|| "error 2")?;

        let third =
            second
                .make_move(&Position::new(File::E, Rank::Seven), &Position::new(File::E, Rank::Six))
                .ok_or_else(|| "error 3")?;

        let fourth =
            third
                .make_move(&Position::new(File::E, Rank::Six), &Position::new(File::E, Rank::Five))
                .ok_or_else(|| "error 4")?;
        assert_eq!(1,
                   fourth.possible_valid_moves(&Position::new(File::D, Rank::Five)).len());
        Ok(())
    }
}
#[cfg(test)]
mod board_tests {
    use chessTUI::File;
    use chessTUI::Rank;
    use chessTUI::Board;

    #[test]
    fn test_board_creation() {
        let board = Board::new();
        assert_eq!(board.tiles.keys().len(), 64);
    }
}
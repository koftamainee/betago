use betago::*;
use rstest::rstest;

    fn pos(x: usize, y: usize) -> Position {
        Position { x, y }
    }

  #[rstest]
    #[case::single_center(
        vec![pos(4, 4)], 
        vec![pos(3, 4), pos(5, 4), pos(4, 3), pos(4, 5)],
        pos(4, 4)
    )]
    #[case::single_edge(
        vec![pos(0, 4)],
        vec![pos(0, 3), pos(0, 5), pos(1, 4)],
        pos(0, 4)
    )]
    #[case::horizontal_group(
        vec![pos(3, 3), pos(4, 3)],
        vec![pos(2, 3), pos(5, 3), pos(3, 2), pos(4, 2), pos(3, 4), pos(4, 4)],
        pos(3, 3)
    )]
    fn test_capture_scenarios(
        #[case] black_stones: Vec<Position>,
        #[case] white_stones: Vec<Position>,
        #[case] stone_to_capture: Position,
    ) {
        let mut board = Board::new(9);

        for &pos in &black_stones {
            board.place_stone(pos, Stone::Black).unwrap();
        }

        for &pos in &white_stones[..white_stones.len() - 1] {
            board.place_stone(pos, Stone::White).unwrap();
        }

        assert_eq!(
            board.get_stone(stone_to_capture).unwrap(),
            Some(Stone::Black),
            "Stone should exists before capture"
        );

        let last_white_pos = *white_stones.last().unwrap();
        board.place_stone(last_white_pos, Stone::White).unwrap();

        assert_eq!(
            board.get_stone(stone_to_capture).unwrap(),
            None,
            "Stone should be captured"
        );

        for pos in black_stones {
            assert_eq!(
                board.get_stone(pos).unwrap(),
                None,
                "All stones in group should be captured"
            );
        }
    }

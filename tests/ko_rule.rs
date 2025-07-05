use betago::*;

#[test]
fn test_ko_rule() {
    let mut board = Board::new(5);

    let black_pos1 = Position { x: 0, y: 1 };
    let black_pos2 = Position { x: 1, y: 0 };
    let black_pos3 = Position { x: 2, y: 1 };
    let black_pos4 = Position { x: 1, y: 2 };
    let white_pos1 = Position { x: 0, y: 2 };
    let white_pos2 = Position { x: 2, y: 2 };
    let white_pos3 = Position { x: 1, y: 3 };

    board.place_stone(black_pos1, Stone::Black).unwrap();
    board.place_stone(black_pos2, Stone::Black).unwrap();
    board.place_stone(black_pos3, Stone::Black).unwrap();
    board.place_stone(black_pos4, Stone::Black).unwrap();

    board.place_stone(white_pos1, Stone::White).unwrap();
    board.place_stone(white_pos2, Stone::White).unwrap();
    board.place_stone(white_pos3, Stone::White).unwrap();

    let capture_pos1 = Position { x: 1, y: 1 };
    let result = board.place_stone(capture_pos1, Stone::White);
    assert!(result.is_ok(), "Should succesfully place stones");
    assert_eq!(result.unwrap(), 1, "Should be captured 1 stone");

    let capture_pos2 = Position { x: 1, y: 2 };
    let ko_result = board.place_stone(capture_pos2, Stone::Black);
    assert!(ko_result.is_err(), "Ko rule should deny this move");
    assert!(
        matches!(ko_result, Err(GoError::KoRuleViolation)),
        "Error should be KoRuleViolation"
    );

    let other_pos = Position { x: 3, y: 1 };
    assert!(
        board.place_stone(other_pos, Stone::Black).is_ok(),
        "Should be able to move to other place"
    );

    assert!(
        board.place_stone(capture_pos2, Stone::Black).is_ok(),
        "should be able to make this move after situation on board changes"
    );
}

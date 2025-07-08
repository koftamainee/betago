use rand::{random, random_range};

use crate::ai::GoAI;
use crate::ai::random::RandomAI;
use crate::{Board, Position, Stone};

pub struct HeuristicAI {}

impl GoAI for HeuristicAI {
    fn select_move(&self, board: &Board, player: Stone) -> Option<Position> {
        let opponent = player.opposite();
        let mut scores: Vec<f64> = vec![0.0; board.size() * board.size()];

        let capture_weight = 10.0;
        let save_weight = 2.5;
        let attack_weight = 1.5;
        let expand_weight = 5.0;
        let eye_weight = 1.5;

        let mut enemy_count = 0;
        let mut allay_count = 0;

        for x in 0..board.size() {
            for y in 0..board.size() {
                let pos = Position { x, y };
                let idx = board.pos_to_index(pos);

                if let Some(stone) = board.get_stone(pos).unwrap() {
                    if stone == player.opposite() {
                        enemy_count += 1;
                    } else {
                        allay_count += 1;
                    }
                }

                if !board.is_valid_move(pos, player) {
                    scores[idx] = -1.0;
                    continue;
                }

                let mut capture_score = 0.0;
                for neighbor in board.get_neighbors(pos) {
                    if let Ok(Some(stone)) = board.get_stone(neighbor) {
                        if stone == opponent {
                            let group = board.get_group(neighbor);
                            if board.count_liberties(&group) == 1 {
                                capture_score += capture_weight * group.len() as f64;
                            }
                        }
                    }
                }

                let mut save_score = 0.0;
                for neighbor in board.get_neighbors(pos) {
                    if let Ok(Some(stone)) = board.get_stone(neighbor) {
                        if stone == player {
                            let group = board.get_group(neighbor);
                            let liberties = board.count_liberties(&group);
                            if liberties == 1 {
                                save_score += save_weight * group.len() as f64;
                            } else if liberties == 2 {
                                save_score += 0.5 * save_weight * group.len() as f64;
                            }
                        }
                    }
                }

                let mut attack_score = 0.0;
                for neighbor in board.get_neighbors(pos) {
                    if let Ok(Some(stone)) = board.get_stone(neighbor) {
                        if stone == opponent {
                            let group = board.get_group(neighbor);
                            let liberties = board.count_liberties(&group);
                            if liberties <= 2 {
                                attack_score += attack_weight
                                    * (3.0 - liberties as f64)
                                    * (group.len() as f64).sqrt();
                            }
                        }
                    }
                }

                let mut expand_score = 0.0;
                let empty_neighbors = board
                    .get_neighbors(pos)
                    .iter()
                    .filter(|&p| board.get_stone(*p).ok().flatten().is_none())
                    .count();
                expand_score += expand_weight * empty_neighbors as f64;

                let mut eye_score = 0.0;
                if is_potential_eye(board, pos, player) {
                    eye_score += eye_weight;
                }

                scores[idx] += capture_score + save_score + attack_score + expand_score + eye_score;

                if would_be_captured_next_move(board, pos, player) {
                    scores[idx] *= 0.2;
                }
            }
        }

        if enemy_count == 0 && allay_count > 0 {
            return None;
        }

        let mut max_score = None;
        for (idx, &score) in scores.iter().enumerate() {
            match max_score {
                None => {
                    if score > 0.0 {
                        max_score = Some((idx, score))
                    }
                }
                Some((_, current_max)) if score > current_max && score > 0.0 => {
                    max_score = Some((idx, score))
                }
                _ => {}
            }
        }

        if let Some((best_idx, _)) = max_score {
            let x = best_idx % board.size();
            let y = best_idx / board.size();
            return Some(Position { x, y });
        }
        None
    }
}

fn is_potential_eye(board: &Board, pos: Position, player: Stone) -> bool {
    let mut friendly_stones = 0;
    let mut empty_or_friendly = 0;

    for neighbor in board.get_neighbors(pos) {
        match board.get_stone(neighbor) {
            Ok(Some(stone)) if stone == player => friendly_stones += 1,
            Ok(None) => empty_or_friendly += 1,
            _ => (),
        }
    }

    friendly_stones >= 3 || (friendly_stones >= 2 && empty_or_friendly == 4)
}

fn would_be_captured_next_move(board: &Board, pos: Position, player: Stone) -> bool {
    let mut test_board = board.clone();
    if test_board.place_stone(pos, player).is_err() {
        return true;
    }

    let opponent = player.opposite();
    for neighbor in test_board.get_neighbors(pos) {
        if let Ok(Some(stone)) = test_board.get_stone(neighbor) {
            if stone == opponent {
                let group = test_board.get_group(neighbor);
                if test_board.count_liberties(&group) == 1 {
                    return true;
                }
            }
        }
    }

    false
}

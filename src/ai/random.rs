use super::GoAI;
use crate::core::{Board, Position, Stone};
use rand::random_range;

#[derive(Default)]
pub struct RandomAI {}

impl GoAI for RandomAI {
    fn select_move(&self, board: &Board, player: Stone) -> Option<Position> {
        let opp = rand::random_range(0..101);
        if opp < 10 {
            return None;
        }
        let mut valid_moves = Vec::new();

        for x in 0..board.size() {
            for y in 0..board.size() {
                let pos = Position { x, y };
                if board.is_valid_move(pos, player) {
                    valid_moves.push(pos);
                }
            }
        }
        if valid_moves.is_empty() {
            None
        } else {
            Some(valid_moves[rand::random_range(0..valid_moves.len())])
        }
    }
}

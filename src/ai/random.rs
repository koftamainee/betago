use super::GoAI;
use crate::core::{Board, Position, Stone};
use rand::{Rng, random_range};

#[derive(Default)]
pub struct RandomAI {}

impl GoAI for RandomAI {
    fn select_move(&self, board: &Board, player: Stone) -> Option<Position> {
        let size = board.size();
        let mut attempts = 0;
        let max_attempts = size * size * 2;
        loop {
            attempts += 1;
            if attempts > max_attempts {
                return None;
            }

            let x = rand::random_range(0..size);
            let y = random_range(0..size);
            let pos = Position { x, y };
            if board.is_valid_move(pos).is_ok() {
                return Some(pos);
            }
        }
    }
}

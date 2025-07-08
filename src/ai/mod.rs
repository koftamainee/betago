use crate::core::{Board, Position, Stone};

pub mod heuristic;
pub mod random;

pub trait GoAI {
    fn select_move(&self, board: &Board, player: Stone) -> Option<Position>;
}

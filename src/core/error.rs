use crate::core::board::Position;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GoError {
    #[error("Position {pos:?} is out of bounds")]
    OutOfBounds { pos: Position },

    #[error("Position {pos:?} is already occupied")]
    PositionOccupied { pos: Position },

    #[error("Suicidal move")]
    SuicidalMove,

    #[error("Ko rule violation")]
    KoRuleViolation,
}

impl GoError {
    pub fn out_of_bounds(pos: Position) -> Self {
        Self::OutOfBounds { pos }
    }

    pub fn position_occupied(pos: Position) -> Self {
        Self::PositionOccupied { pos }
    }

    pub fn suicidal_move() -> Self {
        Self::SuicidalMove
    }

    pub fn ko_rule_violation() -> Self {
        Self::KoRuleViolation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_out_of_bounds_error() {
        let pos = Position { x: 0, y: 0 };
        let error = GoError::out_of_bounds(pos);
        assert!(matches!(error, GoError::OutOfBounds { .. }));
    }

    #[test]
    fn create_position_occupied_error() {
        let pos = Position { x: 0, y: 0 };

        let error = GoError::position_occupied(pos);
        assert!(matches!(error, GoError::PositionOccupied { .. }));
    }

    #[test]
    fn create_suicidal_move_error() {
        let error = GoError::suicidal_move();
        assert!(matches!(error, GoError::SuicidalMove));
    }

    #[test]
    fn create_ko_rule_violation_error() {
        let error = GoError::ko_rule_violation();
        assert!(matches!(error, GoError::KoRuleViolation));
    }
}

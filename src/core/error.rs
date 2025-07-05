use crate::core::board::Position;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GoError {
    #[error("Position {pos:?} is out of bounds")]
    OutOfBounds { pos: Position },

    #[error("Position {pos:?} is already occupied")]
    PositionOccupied { pos: Position },

    #[error("Invalid mode: {reason}")]
    InvalidMove { reason: String },

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

    pub fn invalid_move<S: Into<String>>(reason: S) -> Self {
        Self::InvalidMove {
            reason: reason.into(),
        }
    }

    pub fn ko_rule_violation() -> Self {
        Self::KoRuleViolation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_out_of_bounds() {
        let pos = Position { x: 0, y: 0 };
        let error = GoError::out_of_bounds(pos);
        assert!(matches!(error, GoError::OutOfBounds { .. }));
    }

    #[test]
    fn create_position_occupied() {
        let pos = Position { x: 0, y: 0 };

        let error = GoError::position_occupied(pos);
        assert!(matches!(error, GoError::PositionOccupied { .. }));
    }

    #[test]
    fn create_invalid_move() {
        let reason = "Test reason";

        let error = GoError::invalid_move(reason);
        assert!(matches!(error, GoError::InvalidMove { .. }));
    }

    #[test]
    fn create_ko_rule_violation() {
        let error = GoError::ko_rule_violation();
        assert!(matches!(error, GoError::KoRuleViolation));
    }
}

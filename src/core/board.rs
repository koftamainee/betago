use crate::GoError;
use rand::{self, Rng};
use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    pub fn opposite(&self) -> Self {
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    grid: Vec<Option<Stone>>,

    zobrist_table: Vec<Vec<u64>>,
    current_hash: u64,
    previous_hash: Option<u64>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        if size < 1 {
            panic!("Size of the board should be positive");
        }

        let mut zobrist_table = vec![vec![0; 2]; size * size];
        let mut rng = rand::rng();

        (0..size * size).for_each(|i| {
            zobrist_table[i][0] = rng.random();
            zobrist_table[i][1] = rng.random();
        });

        Board {
            size,
            grid: vec![None; size * size],
            previous_hash: None,
            current_hash: 0,
            zobrist_table,
        }
    }

    #[inline]
    pub fn pos_to_index(&self, pos: Position) -> usize {
        pos.x + pos.y * self.size
    }

    fn update_hash(&mut self, pos: Position, stone: Option<Stone>) {
        let idx = self.pos_to_index(pos);
        if let Some(s) = self.grid[idx] {
            let color_idx = match s {
                Stone::Black => 0,
                Stone::White => 1,
            };
            self.current_hash ^= self.zobrist_table[idx][color_idx];
        }
        if let Some(s) = stone {
            let color_idx = match s {
                Stone::Black => 0,
                Stone::White => 1,
            };
            self.current_hash ^= self.zobrist_table[idx][color_idx];
        }
    }

    fn internal_move_validate(&self, pos: Position) -> Result<(), GoError> {
        if !self.is_on_board(pos) {
            return Err(GoError::out_of_bounds(pos));
        }
        if self.grid[self.pos_to_index(pos)].is_some() {
            return Err(GoError::PositionOccupied { pos });
        }

        Ok(())
    }

    pub fn is_valid_move(&self, pos: Position, stone: Stone) -> bool {
        let mut board_clone = self.clone();

        board_clone.place_stone(pos, stone).is_ok()
    }

    pub fn place_stone(&mut self, pos: Position, stone: Stone) -> Result<usize, GoError> {
        self.internal_move_validate(pos)?;

        let current_hash = self.calculate_hash();

        let index = self.pos_to_index(pos);
        self.grid[index] = Some(stone);
        self.update_hash(pos, Some(stone));

        let opponent = stone.opposite();

        let mut captured = HashSet::new();

        for neighbor in self.get_neighbors(pos) {
            if self.get_stone(neighbor)? == Some(opponent) {
                let group = self.get_group(neighbor);
                if !self.has_liberties(&group) {
                    captured.extend(group);
                }
            }
        }

        let current_group = self.get_group(pos);
        if !self.has_liberties(&current_group) && captured.is_empty() {
            self.grid[index] = None;
            self.update_hash(pos, None);
            return Err(GoError::suicidal_move());
        }

        let captured_len = captured.len();
        for pos in &captured {
            self.update_hash(*pos, None);
            let index = self.pos_to_index(*pos);
            self.grid[index] = None;
        }

        if captured_len == 1 {
            if let Some(prev_hash) = self.previous_hash {
                if prev_hash == self.calculate_hash() {
                    self.grid[index] = None;
                    for pos in self.get_group(pos) {
                        self.update_hash(pos, None);
                    }
                    return Err(GoError::ko_rule_violation());
                }
            }
        }

        if captured_len > 0 {
            self.previous_hash = Some(current_hash);
        }

        Ok(captured_len)
    }

    fn calculate_hash(&self) -> u64 {
        let mut hash = 0;
        for (i, stone) in self.grid.iter().enumerate() {
            if let Some(s) = stone {
                let color_idx = match s {
                    Stone::Black => 0,
                    Stone::White => 1,
                };
                hash ^= self.zobrist_table[i][color_idx];
            }
        }
        hash
    }

    pub fn get_stone(&self, pos: Position) -> Result<Option<Stone>, GoError> {
        if !self.is_on_board(pos) {
            return Err(GoError::OutOfBounds { pos });
        }

        Ok(self.grid[self.pos_to_index(pos)])
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn is_on_board(&self, pos: Position) -> bool {
        pos.x < self.size && pos.y < self.size
    }

    pub fn get_group(&self, pos: Position) -> Vec<Position> {
        let mut group = Vec::new();

        if let Some(stone) = self.get_stone(pos).unwrap_or(None) {
            let mut visited = vec![false; self.size * self.size];
            self.find_connected_stones(pos, stone, &mut visited, &mut group);
        }

        group
    }

    fn find_connected_stones(
        &self,
        pos: Position,
        stone: Stone,
        visited: &mut [bool],
        group: &mut Vec<Position>,
    ) {
        if visited[self.pos_to_index(pos)] {
            return;
        }
        visited[self.pos_to_index(pos)] = true;
        group.push(pos);
        for neighbor in self.get_neighbors(pos) {
            if let Some(s) = self.get_stone(neighbor).unwrap_or(None) {
                if s == stone {
                    self.find_connected_stones(neighbor, stone, visited, group);
                }
            }
        }
    }

    pub fn get_neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dx, dy) in directions {
            let x = pos.x as i32 + dx;
            let y = pos.y as i32 + dy;

            if x >= 0 && x < self.size as i32 && y >= 0 && y < self.size as i32 {
                neighbors.push(Position {
                    x: x as usize,
                    y: y as usize,
                });
            }
        }

        neighbors
    }

    pub fn count_liberties(&self, group: &[Position]) -> usize {
        let mut liberties = 0;
        let mut counted = vec![false; self.size * self.size];

        for pos in group {
            for neighbor in self.get_neighbors(*pos) {
                let neighbor_index = self.pos_to_index(neighbor);
                if !counted[neighbor_index] && self.get_stone(neighbor).unwrap_or(None).is_none() {
                    liberties += 1;
                    counted[neighbor_index] = true;
                }
            }
        }

        liberties
    }
    fn has_liberties(&self, group: &[Position]) -> bool {
        self.count_liberties(group) > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn create_new_board() {
        const BOARD_SIZE: usize = 19;
        let board = Board::new(BOARD_SIZE);
        assert_eq!(board.size(), BOARD_SIZE, "Board should has size of 19");

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let pos = Position { x, y };
                assert!(
                    board.get_stone(pos).unwrap().is_none(),
                    "New board should be empty"
                );
            }
        }
    }

    #[rstest]
    #[case(0, 0, Stone::Black)]
    #[case(4, 5, Stone::White)]
    #[case(3, 3, Stone::Black)]
    fn place_stone_valid(#[case] x: usize, #[case] y: usize, #[case] stone: Stone) {
        let mut board = Board::new(19);

        let pos = Position { x, y };
        assert!(
            board.place_stone(pos, stone).is_ok(),
            "Stone should be placed"
        );
        assert_eq!(
            board.get_stone(pos).unwrap(),
            Some(stone),
            "Stone should appear on the board"
        );
    }

    #[test]
    fn place_stone_already_occupied() {
        let mut board = Board::new(19);
        let pos = Position { x: 0, y: 0 };
        board.place_stone(pos, Stone::Black).unwrap();

        let test_result = board.place_stone(pos, Stone::White);
        assert!(
            matches!(test_result, Err(GoError::PositionOccupied { .. })),
            "Shoudn't be able to place stone on occupied position"
        );
    }

    #[rstest]
    #[case(0, 19)]
    #[case(19, 0)]
    #[case(19, 19)]
    fn place_stone_out_of_bounds(#[case] x: usize, #[case] y: usize) {
        let mut board = Board::new(19);
        let pos = Position { x, y };

        assert!(
            matches!(
                board.place_stone(pos, Stone::Black),
                Err(GoError::OutOfBounds { .. })
            ),
            "Shoudn't be able to place stone of of board bounds"
        );
    }

    #[rstest]
    #[case(0, 0, None)]
    #[case(1, 1, Some(Stone::Black))]
    fn get_stone(#[case] x: usize, #[case] y: usize, #[case] expected: Option<Stone>) {
        let mut board = Board::new(19);
        let pos = Position { x, y };
        if let Some(stone) = expected {
            board.place_stone(pos, stone).unwrap();
        }

        assert_eq!(
            board.get_stone(pos).unwrap(),
            expected,
            "Stone should be placed on a board"
        );
    }

    #[test]
    fn get_stone_out_of_bounds() {
        let board = Board::new(19);
        let pos = Position { x: 19, y: 19 };
        assert!(
            board.get_stone(pos).is_err(),
            "Shouldn't be able to get stone from out of bounds of the board"
        );
    }

    #[test]
    #[should_panic]
    fn create_zero_size_board() {
        let _board = Board::new(0);
    }

    #[test]
    fn create_minimal_size_board() {
        let board = Board::new(1);
        assert!(
            board.get_stone(Position { x: 0, y: 0 }).is_ok(),
            "One-sized board should have [0,0] position"
        );
        assert!(
            board.get_stone(Position { x: 0, y: 1 }).is_err(),
            "One-sized board should have ONLY one posision"
        );
        assert!(
            board.get_stone(Position { x: 1, y: 0 }).is_err(),
            "One-sized board should have ONLY one posision"
        );
    }
}

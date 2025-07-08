use crate::{Board, GoError, Position, Stone};

pub struct Game {
    board: Board,
    current_player: Stone,
    captured_stones: (usize, usize),
    current_move: usize,
    passes_count: u8,
}

impl Game {
    pub fn new(board_size: usize) -> Self {
        Game {
            board: Board::new(board_size),
            current_player: Stone::Black,
            captured_stones: (0, 0),
            current_move: 1,
            passes_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::new(self.board_size());
        self.current_player = Stone::Black;
        self.captured_stones = (0, 0);
        self.current_move = 1;
        self.passes_count = 0;
    }

    #[inline]
    fn switch_player(&mut self) {
        self.current_player = self.current_player.opposite();
    }

    pub fn pass(&mut self) {
        self.passes_count += 1;
        self.switch_player();
    }

    pub fn is_game_over(&self) -> Option<Stone> {
        match self.passes_count {
            2 => Some(self.determine_winner()),
            _ => None,
        }
    }
    fn determine_winner(&self) -> Stone {
        if self.captured_stones.0 > self.captured_stones.1 {
            Stone::Black
        } else {
            Stone::White
        }
    }

    pub fn make_move(&mut self, pos: Position) -> Result<(), GoError> {
        if self.passes_count == 2 {
            return Err(GoError::GameOver);
        }

        let captured_count = self.board.place_stone(pos, self.current_player)?;

        match self.current_player {
            Stone::Black => self.captured_stones.0 += captured_count,
            Stone::White => self.captured_stones.1 += captured_count,
        }

        self.current_move += 1;

        if self.current_player == Stone::White {
            self.passes_count = 0;
        }
        self.switch_player();

        Ok(())
    }

    pub fn board_size(&self) -> usize {
        self.board.size()
    }

    pub fn current_player(&self) -> Stone {
        self.current_player
    }

    pub fn captured_stones(&self) -> (usize, usize) {
        self.captured_stones
    }

    pub fn current_move(&self) -> usize {
        self.current_move
    }

    pub fn winner(&self) -> Option<Stone> {
        self.is_game_over()
    }

    pub fn board_state(&self) -> &Board {
        &self.board
    }

    pub fn stone_at(&self, pos: Position) -> Result<Option<Stone>, GoError> {
        self.board.get_stone(pos)
    }
}

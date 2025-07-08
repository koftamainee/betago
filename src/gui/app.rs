use crate::{
    ai::{GoAI, heuristic::HeuristicAI, random::RandomAI},
    core::{Stone, game::Game},
};
use eframe::egui;

pub struct GoApp {
    game: Game,
    board_size: f32,

    black_ai: Box<dyn GoAI>,
    white_ai: Box<dyn GoAI>,
}

impl Default for GoApp {
    fn default() -> Self {
        Self {
            game: Game::new(19),
            board_size: 800.0,
            black_ai: Box::new(RandomAI {}),
            white_ai: Box::new(HeuristicAI {}),
        }
    }
}

impl eframe::App for GoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        if self.game.is_game_over().is_none() {
            let ai_move = match self.game.current_player() {
                Stone::Black => self
                    .black_ai
                    .select_move(self.game.board_state(), Stone::Black),
                Stone::White => self
                    .white_ai
                    .select_move(self.game.board_state(), Stone::White),
            };

            match ai_move {
                Some(pos) => match self.game.make_move(pos) {
                    Ok(_) => {}
                    Err(_) => self.game.pass(),
                },
                None => self.game.pass(),
            }
        } else {
            self.game.reset();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_board(ui);
        });
    }
}

impl GoApp {
    fn draw_board(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(
            egui::vec2(self.board_size, self.board_size),
            egui::Sense::click(),
        );

        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                self.handle_click(pos, response.rect);
            }
        }

        painter.rect_filled(response.rect, 25.0, egui::Color32::from_rgb(210, 180, 130));

        self.draw_grid(&painter, response.rect);

        self.draw_stones(&painter, response.rect);
    }

    fn handle_click(&mut self, pos: egui::Pos2, rect: egui::Rect) {
        let board_size = (self.game.board_size() + 2) as f32;
        let cell_size = rect.width() / (board_size - 1.0);

        let x = ((pos.x - rect.left()) / cell_size).round() as usize;
        let y = ((pos.y - rect.top()) / cell_size).round() as usize;

        if x < board_size as usize && y < board_size as usize && x > 0 && y > 0 {
            let _ = self.game.make_move(crate::Position { x: x - 1, y: y - 1 });
        }
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        let board_size = (self.game.board_size() + 2) as f32;
        let cell_size = rect.width() / (board_size - 1.0);

        for i in 0..self.game.board_size() + 1 {
            let pos = i as f32 * cell_size;
            painter.line_segment(
                [
                    egui::Pos2::new(rect.left(), rect.top() + pos),
                    egui::Pos2::new(rect.right(), rect.top() + pos),
                ],
                egui::Stroke::new(1.0, egui::Color32::BLACK),
            );

            painter.line_segment(
                [
                    egui::Pos2::new(rect.left() + pos, rect.top()),
                    egui::Pos2::new(rect.left() + pos, rect.bottom()),
                ],
                egui::Stroke::new(1.0, egui::Color32::BLACK),
            );
        }
    }

    fn draw_stones(&self, painter: &egui::Painter, rect: egui::Rect) {
        let board_size = (self.game.board_size() + 2) as f32;
        let cell_size = rect.width() / (board_size - 1.0);
        let stone_radius = cell_size * 0.4;

        let board_state = self.game.board_state();

        for y in 0..self.game.board_size() {
            for x in 0..self.game.board_size() {
                if let Ok(Some(stone)) = board_state.get_stone(crate::Position { x, y }) {
                    let pos = egui::Pos2::new(
                        rect.left() + (x + 1) as f32 * cell_size,
                        rect.top() + (y + 1) as f32 * cell_size,
                    );

                    match stone {
                        Stone::Black => {
                            painter.circle_filled(pos, stone_radius, egui::Color32::BLACK);
                        }
                        Stone::White => {
                            painter.circle_filled(pos, stone_radius, egui::Color32::WHITE);
                            painter.circle_stroke(
                                pos,
                                stone_radius,
                                egui::Stroke::new(1.0, egui::Color32::BLACK),
                            );
                        }
                    }
                }
            }
        }
    }
}

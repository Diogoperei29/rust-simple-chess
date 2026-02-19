use chess_core::{Color, PieceType};
use egui::{Context, Pos2, Vec2};
use crate::assets::PieceAssets;

/// Represents the result of a chess game
#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    Checkmate(Color), // Winner's color
    Stalemate,
    Draw,
}

/// Handles the end game victory screen
pub struct ChessEndScreen {
    window_initialized: bool,
}

impl ChessEndScreen {
    pub fn new() -> Self {
        Self {
            window_initialized: false,
        }
    }

    /// Draw the victory screen
    /// Returns (should_close, new_game_requested)
    pub fn draw(
        &mut self,
        ctx: &Context,
        result: GameResult,
        board_center: Pos2,
        piece_assets: &Option<PieceAssets>,
    ) -> (bool, bool) {
        let mut should_close = false;
        let mut new_game_requested = false;
        let mut open = true;
        
        let window_size = Vec2::new(300.0, 200.0);
        let window_pos = Pos2::new(
            board_center.x - window_size.x / 2.0,
            board_center.y - window_size.y / 2.0,
        );

        let window_title = match result {
            GameResult::Checkmate(_) => "Game Over",
            GameResult::Stalemate => "Stalemate",
            GameResult::Draw => "Draw",
        };

        let mut window = egui::Window::new(window_title)
            .collapsible(false)
            .resizable(false)
            .default_size(window_size)
            .movable(true)
            .open(&mut open);
        
        // Only set position on first show
        if !self.window_initialized {
            window = window.default_pos(window_pos);
            self.window_initialized = true;
        }
        
        window.show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    
                    match result {
                        GameResult::Checkmate(winner) => {
                            // Draw king piece
                            if let Some(assets) = piece_assets {
                                if let Some(texture) = assets.get(winner, PieceType::King) {
                                    ui.add(egui::Image::new(texture).fit_to_exact_size(Vec2::new(80.0, 80.0)));
                                }
                            }
                            
                            ui.add_space(10.0);
                            
                            // Winner text
                            let winner_text = match winner {
                                Color::White => "White wins!",
                                Color::Black => "Black wins!",
                            };
                            ui.label(
                                egui::RichText::new(winner_text)
                                    .size(24.0)
                                    .strong()
                            );
                        }
                        GameResult::Stalemate => {
                            ui.add_space(20.0);
                            ui.label(
                                egui::RichText::new("Stalemate!")
                                    .size(24.0)
                                    .strong()
                            );
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new("No legal moves available")
                                    .size(14.0)
                            );
                        }
                        GameResult::Draw => {
                            ui.add_space(20.0);
                            ui.label(
                                egui::RichText::new("Draw!")
                                    .size(24.0)
                                    .strong()
                            );
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new("Game ended in a draw")
                                    .size(14.0)
                            );
                        }
                    }
                    
                    ui.add_space(15.0);
                    
                    // Buttons - centered
                    ui.horizontal(|ui| {
                        let button_width = 80.0;
                        let total_width = button_width * 2.0 + ui.spacing().item_spacing.x;
                        let padding = (ui.available_width() - total_width) / 2.0;
                        ui.add_space(padding);
                        
                        if ui.add_sized([button_width, 24.0], egui::Button::new("New Game")).clicked() {
                            new_game_requested = true;
                        }
                        
                        if ui.add_sized([button_width, 24.0], egui::Button::new("Close")).clicked() {
                            should_close = true;
                        }
                    });
                });
            });
        
        if !open {
            should_close = true;
        }
        
        (should_close, new_game_requested)
    }

    pub fn reset(&mut self) {
        self.window_initialized = false;
    }
}

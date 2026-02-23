use crate::assets::PieceAssets;
use crate::end_screen::{ChessEndScreen, GameResult};
use crate::game_view::ChessGameView;
use chess_core::Game;
use egui::{CentralPanel, Context, SidePanel};

/// Main application that orchestrates the chess GUI
pub struct ChessApp {
    game: Game,
    piece_assets: Option<PieceAssets>,
    game_view: ChessGameView,
    end_screen: ChessEndScreen,
    game_over: Option<GameResult>,
}

impl ChessApp {
    pub fn new(egui_ctx: &egui::Context) -> Self {
        Self {
            game: Game::new(),
            piece_assets: Some(PieceAssets::new(egui_ctx)),
            game_view: ChessGameView::new(),
            end_screen: ChessEndScreen::new(),
            game_over: None,
        }
    }
}

// main component for GUI
impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Calculate move panel width as a fixed size
        let move_panel_width = 150.0;

        // Move history panel on the right - fixed width
        SidePanel::right("move_history_panel")
            .exact_width(move_panel_width)
            .resizable(false)
            .show(ctx, |ui| {
                self.game_view.draw_move_history(ui, &self.game);
            });

        // Main board area
        CentralPanel::default().show(ctx, |ui| {
            // Calculate board size based on central panel available space
            let available_size = ui.available_size();
            let board_size = (available_size.x.min(available_size.y) * 0.85).max(300.0);
            let left_margin = (available_size.x - board_size) / 2.0;

            ui.vertical(|ui| {
                // Top player label and New Game button - both aligned with board
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(left_margin);

                    // Create a horizontal layout that spans the board width
                    ui.allocate_ui(egui::Vec2::new(board_size, 30.0), |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Opponent").size(18.0));

                            // Push button to the right within the board width
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .button(egui::RichText::new("New Game").size(16.0))
                                        .clicked()
                                    {
                                        self.game.reset();
                                        self.game_over = None;
                                        self.game_view.reset();
                                    }
                                },
                            );
                        });
                    });
                });
                ui.add_space(10.0);

                // Draw the chess board - centered
                self.game_view.draw(
                    ui,
                    board_size,
                    left_margin,
                    &mut self.game,
                    &self.piece_assets,
                );

                // Check for game over after drawing
                if self.game_over.is_none() {
                    let current_player = self.game.active_player();
                    if self.game.is_checkmated(current_player) {
                        self.game_over = Some(GameResult::Checkmate(current_player.opposite()));
                    } else if self.game.is_stalemate(current_player) {
                        self.game_over = Some(GameResult::Stalemate);
                    }
                }

                // Bottom player label - left aligned with board
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(left_margin);
                    ui.label(egui::RichText::new("You").size(18.0));
                });
            });
        });

        // Draw victory screen overlay if game is over
        if let Some(result) = self.game_over {
            let (should_close, new_game_requested) = self.end_screen.draw(
                ctx,
                result,
                self.game_view.board_center(),
                &self.piece_assets,
            );

            if new_game_requested {
                self.game.reset();
                self.game_over = None;
                self.game_view.reset();
                self.end_screen.reset();
            } else if should_close {
                self.game_over = None;
                self.end_screen.reset();
            }
        }
    }
}

use crate::assets::PieceAssets;
use chess_core::{Game, Piece, PieceType, Square};
use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Ui, Vec2};

/// Handles the main chess game board and move history display
pub struct ChessGameView {
    cached_valid_moves: Vec<Square>,
    board_center: Pos2,
}

impl ChessGameView {
    pub fn new() -> Self {
        Self {
            cached_valid_moves: Vec::new(),
            board_center: Pos2::new(400.0, 400.0),
        }
    }

    pub fn board_center(&self) -> Pos2 {
        self.board_center
    }

    /// Draw the main game area including board and move history
    pub fn draw(
        &mut self,
        ui: &mut Ui,
        board_size: f32,
        left_margin: f32,
        game: &mut Game,
        piece_assets: &Option<PieceAssets>,
    ) {
        // Draw the chess board
        ui.horizontal(|ui| {
            ui.add_space(left_margin);
            let board_rect = self.draw_board(ui, board_size, game, piece_assets);
            self.board_center = board_rect.center();

            // Draw promotion UI if needed
            if let Some(promo_square) = game.get_promotion_square() {
                let square_size = board_size / 8.0;
                self.draw_promotion_ui(
                    ui,
                    board_rect,
                    square_size,
                    promo_square,
                    game,
                    piece_assets,
                );
            }
        });
    }

    /// Draw the move history panel
    pub fn draw_move_history(&self, ui: &mut Ui, game: &Game) {
        ui.heading("Move History");
        ui.separator();

        let scroll_area = egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true);
        scroll_area.show(ui, |ui| {
            let last_move_index = game.move_history().len().saturating_sub(1);
            let moves = game.move_history();

            // Group moves into pairs (white, black)
            let mut move_pairs: Vec<(usize, Option<&String>, Option<&String>)> = Vec::new();
            for i in (0..moves.len()).step_by(2) {
                let move_number = (i / 2) + 1;
                let white_move = moves.get(i);
                let black_move = moves.get(i + 1);
                move_pairs.push((move_number, white_move, black_move));
            }

            // Draw table with subtle styling
            for (pair_index, (move_number, white_move, black_move)) in
                move_pairs.into_iter().enumerate()
            {
                // Draw horizontal divider above each row except the first
                if pair_index > 0 {
                    ui.painter().hline(
                        ui.cursor().x_range(),
                        ui.cursor().top(),
                        egui::Stroke::new(0.3, Color32::from_rgba_premultiplied(150, 150, 150, 20)),
                    );
                }

                ui.horizontal(|ui| {
                    ui.add_space(3.0);

                    // Move number on the left (outside the table)
                    ui.label(egui::RichText::new(format!("{}.", move_number)).size(11.0));

                    ui.add_space(5.0);

                    // White move column with fixed width
                    ui.allocate_ui_with_layout(
                        Vec2::new(40.0, 18.0),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            if let Some(white_text) = white_move {
                                let white_index = (move_number - 1) * 2;
                                let is_last = white_index == last_move_index;

                                let mut text =
                                    egui::RichText::new(white_text).font(FontId::monospace(12.0));
                                if is_last {
                                    text = text.strong();
                                }

                                let button = egui::Button::new(text)
                                    .frame(false)
                                    .fill(Color32::TRANSPARENT);

                                let response = ui.add(button);

                                if response.hovered() {
                                    ui.painter().rect_filled(
                                        response.rect,
                                        2.0,
                                        Color32::from_rgba_premultiplied(100, 100, 100, 40),
                                    );
                                }

                                if response.clicked() {
                                    println!("move {} clicked", white_index);
                                }
                            }
                        },
                    );

                    // Draw vertical divider
                    ui.separator();

                    // Black move column with fixed width
                    ui.allocate_ui_with_layout(
                        Vec2::new(40.0, 18.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            if let Some(black_text) = black_move {
                                let black_index = (move_number - 1) * 2 + 1;
                                let is_last = black_index == last_move_index;

                                let mut text =
                                    egui::RichText::new(black_text).font(FontId::monospace(12.0));
                                if is_last {
                                    text = text.strong();
                                }

                                let button = egui::Button::new(text)
                                    .frame(false)
                                    .fill(Color32::TRANSPARENT);

                                let response = ui.add(button);

                                if response.hovered() {
                                    ui.painter().rect_filled(
                                        response.rect,
                                        2.0,
                                        Color32::from_rgba_premultiplied(100, 100, 100, 40),
                                    );
                                }

                                if response.clicked() {
                                    println!("move {} clicked", black_index);
                                }
                            }
                        },
                    );
                });
            }
        });
    }

    fn draw_board(
        &mut self,
        ui: &mut Ui,
        board_size: f32,
        game: &mut Game,
        piece_assets: &Option<PieceAssets>,
    ) -> Rect {
        let square_size = board_size / 8.0;

        let (response, painter) =
            ui.allocate_painter(Vec2::new(board_size, board_size), Sense::click_and_drag());

        let board_rect = response.rect;

        // Handle inputs
        self.handle_input(&response, board_rect, square_size, game);

        // Draw squares
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::new(rank, file).unwrap();
                let display_rank = 7 - rank;
                let x = board_rect.min.x + file as f32 * square_size;
                let y = board_rect.min.y + display_rank as f32 * square_size;

                let square_rect =
                    Rect::from_min_size(Pos2::new(x, y), Vec2::new(square_size, square_size));

                let is_light_square = (rank + file) % 2 == 0;
                let is_selected = game.selected_square() == Some(square);
                let is_last_move = game
                    .last_move()
                    .map(|(from, to)| from == square || to == square)
                    .unwrap_or(false);
                let is_valid_move = self.cached_valid_moves.contains(&square);

                let color = if is_selected {
                    if is_light_square {
                        Color32::from_rgb(205, 210, 106)
                    } else {
                        Color32::from_rgb(170, 162, 58)
                    }
                } else if is_last_move {
                    if is_light_square {
                        Color32::from_rgb(205, 210, 106)
                    } else {
                        Color32::from_rgb(170, 162, 58)
                    }
                } else {
                    if is_light_square {
                        Color32::from_rgb(238, 238, 210)
                    } else {
                        Color32::from_rgb(118, 150, 86)
                    }
                };

                painter.rect_filled(square_rect, 0.0, color);

                // Draw valid move indicator
                if is_valid_move {
                    let center = square_rect.center();
                    let has_piece = game.get_piece(square).is_some();

                    if has_piece {
                        let radius = square_size * 0.4;
                        painter.circle_stroke(
                            center,
                            radius,
                            egui::Stroke::new(
                                square_size * 0.08,
                                Color32::from_rgba_premultiplied(0, 0, 0, 100),
                            ),
                        );
                    } else {
                        let radius = square_size * 0.15;
                        painter.circle_filled(
                            center,
                            radius,
                            Color32::from_rgba_premultiplied(0, 0, 0, 100),
                        );
                    }
                }

                // Draw piece if present (and not being dragged)
                if Some(square) != game.selected_square() || !response.dragged() {
                    if let Some(piece) = game.get_piece(square) {
                        self.draw_piece(&painter, square_rect, piece, piece_assets);
                    }
                }

                // Draw coordinates
                if file == 0 {
                    painter.text(
                        Pos2::new(x + 4.0, y + 4.0),
                        Align2::LEFT_TOP,
                        format!("{}", rank + 1),
                        FontId::monospace(12.0),
                        if is_light_square {
                            Color32::from_rgb(118, 150, 86)
                        } else {
                            Color32::from_rgb(238, 238, 210)
                        },
                    );
                }

                if rank == 0 {
                    let file_char = (b'a' + file as u8) as char;
                    painter.text(
                        Pos2::new(x + square_size - 4.0, y + square_size - 4.0),
                        Align2::RIGHT_BOTTOM,
                        format!("{}", file_char),
                        FontId::monospace(12.0),
                        if is_light_square {
                            Color32::from_rgb(118, 150, 86)
                        } else {
                            Color32::from_rgb(238, 238, 210)
                        },
                    );
                }
            }
        }

        // Draw dragged piece
        if let Some(selected) = game.selected_square() {
            if response.dragged() {
                if let Some(piece) = game.get_piece(selected) {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let drag_rect =
                            Rect::from_center_size(pos, Vec2::new(square_size, square_size));
                        self.draw_piece(&painter, drag_rect, piece, piece_assets);
                    }
                }
            }
        }

        board_rect
    }

    fn draw_piece(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        piece: Piece,
        piece_assets: &Option<PieceAssets>,
    ) {
        if let Some(assets) = piece_assets {
            if let Some(texture) = assets.get(piece.color, piece.piece_type) {
                painter.image(
                    texture.id(),
                    rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );
            }
        }
    }

    fn handle_input(
        &mut self,
        response: &egui::Response,
        board_rect: Rect,
        square_size: f32,
        game: &mut Game,
    ) {
        self.handle_click(response, board_rect, square_size, game);
        self.handle_drag(response, board_rect, square_size, game);
    }

    fn handle_click(
        &mut self,
        response: &egui::Response,
        board_rect: Rect,
        square_size: f32,
        game: &mut Game,
    ) {
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(clicked_square) = self.pos_to_square(pos, board_rect, square_size) {
                    self.on_square_clicked(clicked_square, game);
                }
            }
        }
    }

    fn handle_drag(
        &mut self,
        response: &egui::Response,
        board_rect: Rect,
        square_size: f32,
        game: &mut Game,
    ) {
        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(square) = self.pos_to_square(pos, board_rect, square_size) {
                    self.on_drag_started(square, game);
                }
            }
        }

        if response.drag_stopped() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(to_square) = self.pos_to_square(pos, board_rect, square_size) {
                    self.on_drag_stopped(to_square, game);
                    return;
                }
            }
            self.on_drag_cancelled(game);
        }
    }

    fn on_square_clicked(&mut self, clicked_square: Square, game: &mut Game) {
        // Don't allow moves if promotion is pending
        if game.get_promotion_square().is_some() {
            return;
        }

        if let Some(selected) = game.selected_square() {
            if clicked_square == selected {
                self.deselect_piece(game);
            } else if self.cached_valid_moves.contains(&clicked_square) {
                game.execute_move(selected, clicked_square);
                self.deselect_piece(game);
            } else if game.get_piece(clicked_square).is_some() {
                self.select_piece(clicked_square, game);
            } else {
                self.deselect_piece(game);
            }
        } else {
            if game.get_piece(clicked_square).is_some() {
                self.select_piece(clicked_square, game);
            }
        }
    }

    fn on_drag_started(&mut self, square: Square, game: &mut Game) {
        // Don't allow dragging if promotion is pending
        if game.get_promotion_square().is_some() {
            return;
        }

        if game.get_piece(square).is_some() {
            self.select_piece(square, game);
        }
    }

    fn on_drag_stopped(&mut self, to_square: Square, game: &mut Game) {
        // Don't allow moves if promotion is pending
        if game.get_promotion_square().is_some() {
            self.deselect_piece(game);
            return;
        }

        if let Some(from_square) = game.selected_square() {
            if from_square != to_square && self.cached_valid_moves.contains(&to_square) {
                game.execute_move(from_square, to_square);
            }
        }
        self.deselect_piece(game);
    }

    fn on_drag_cancelled(&mut self, game: &mut Game) {
        self.deselect_piece(game);
    }

    fn select_piece(&mut self, square: Square, game: &mut Game) {
        if let Some(piece) = game.get_piece(square) {
            if piece.color == game.active_player() {
                game.set_selected_square(Some(square));
                self.cached_valid_moves = game.get_valid_moves(square);
                return;
            }
        }
        self.deselect_piece(game);
    }

    fn deselect_piece(&mut self, game: &mut Game) {
        game.set_selected_square(None);
        self.cached_valid_moves.clear();
    }

    fn pos_to_square(&self, pos: Pos2, board_rect: Rect, square_size: f32) -> Option<Square> {
        if !board_rect.contains(pos) {
            return None;
        }

        let file = ((pos.x - board_rect.min.x) / square_size) as u8;
        let display_rank = ((pos.y - board_rect.min.y) / square_size) as u8;
        let rank = 7 - display_rank;

        Square::new(rank, file).ok()
    }

    pub fn reset(&mut self) {
        self.cached_valid_moves.clear();
    }

    fn draw_promotion_ui(
        &mut self,
        ui: &mut Ui,
        board_rect: Rect,
        square_size: f32,
        promotion_square: Square,
        game: &mut Game,
        piece_assets: &Option<PieceAssets>,
    ) {
        // Get the piece color from the promotion square
        let piece_color = if let Some(piece) = game.get_piece(promotion_square) {
            piece.color
        } else {
            return;
        };

        // Calculate position for the promotion UI
        let file = promotion_square.file() as f32;
        let display_rank = (7 - promotion_square.rank()) as f32;

        // Make promotion squares 10% smaller
        let promo_square_size = square_size * 0.9;
        let offset = (square_size - promo_square_size) / 2.0;

        // Position the UI above or below the square depending on which side is promoting
        let x = board_rect.min.x + file * square_size + offset;
        let y = if promotion_square.rank() == 7 {
            // White promoting at top, show below
            board_rect.min.y + display_rank * square_size + square_size + offset
        } else {
            // Black promoting at bottom, show above
            board_rect.min.y + display_rank * square_size - promo_square_size * 4.0 - offset
        };

        // Define the promotion options
        let promotion_options = [
            PieceType::Queen,
            PieceType::Rook,
            PieceType::Bishop,
            PieceType::Knight,
        ];

        // Draw promotion pieces vertically
        for (i, &piece_type) in promotion_options.iter().enumerate() {
            let piece_y = y + i as f32 * promo_square_size;
            let piece_rect = Rect::from_min_size(
                Pos2::new(x, piece_y),
                Vec2::new(promo_square_size, promo_square_size),
            );

            // Draw background (white-grayish)
            let bg_color = Color32::from_rgb(245, 245, 245);
            ui.painter().rect_filled(piece_rect, 0.0, bg_color);

            // Draw border
            let border_rect = piece_rect.expand(1.0);
            ui.painter().rect_filled(border_rect, 0.0, Color32::BLACK);
            ui.painter().rect_filled(piece_rect, 0.0, bg_color);

            // Draw piece
            let piece = Piece::new(piece_color, piece_type);
            self.draw_piece(ui.painter(), piece_rect, piece, piece_assets);

            // Check for click
            let response = ui.allocate_rect(piece_rect, Sense::click());
            if response.clicked() {
                game.promote_square(promotion_square, piece_type);
            }

            // Hover effect
            if response.hovered() {
                let hover_rect = piece_rect.expand(2.0);
                ui.painter()
                    .rect_filled(hover_rect, 0.0, Color32::from_rgb(255, 255, 0));
                ui.painter().rect_filled(piece_rect, 0.0, bg_color);
                self.draw_piece(ui.painter(), piece_rect, piece, piece_assets);
            }
        }
    }
}

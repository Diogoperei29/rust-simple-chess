use crate::{Board, Color, Piece, Square};

pub enum MoveType {
    Move,
    Take,
    QueenSideCastle,
    KingSideCastle,
}

pub struct Game {
    board: Board,
    active_player: Color,
    selected_square: Option<Square>,
    last_move: Option<(Square, Square)>,
    move_history: Vec<String>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new_starting_default(),
            active_player: Color::White,
            selected_square: None,
            last_move: None,
            move_history: Vec::new(),
        }
    }

    /// Get the board reference
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get the currently selected square
    pub fn selected_square(&self) -> Option<Square> {
        self.selected_square
    }

    /// Set the selected square
    pub fn set_selected_square(&mut self, square: Option<Square>) {
        self.selected_square = square;
    }

    /// Get the last move made
    pub fn last_move(&self) -> Option<(Square, Square)> {
        self.last_move
    }

    /// Get the move history
    pub fn move_history(&self) -> &Vec<String> {
        &self.move_history
    }

    /// Get valid moves for the currently selected piece
    pub fn get_valid_moves(&self, square: Square) -> Vec<Square> {
        self.board.get_valid_moves(square)
    }

    /// Get piece at a specific square
    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.board.get_piece(square)
    }

    /// Execute a move from one square to another
    pub fn execute_move(&mut self, from: Square, to: Square) {
        // Move the piece
        if let Some(piece) = self.board.get_piece(from) {
            // Check if it's a capture before making the move
            let is_capture = self.board.get_piece(to).is_some();

            self.board.make_move(from, to);

            // Check for check and checkmate after the move
            let opponent_color = self.active_player.opposite();
            let is_checkmate = self.board.is_checkmated(opponent_color);
            let is_check = !is_checkmate && self.board.is_king_in_check(opponent_color);

            // Record move in history with proper notation
            let move_notation = to.to_notation();
            let piece_symbol = piece.get_piece_ascii();

            // Build move string: [symbol] [capture?] [square] [check/checkmate?]
            let mut move_str = String::new();

            // Add piece symbol
            move_str.push_str(&piece_symbol);

            // Add capture notation
            if is_capture {
                move_str.push('x');
            }

            // Add square
            move_str.push_str(&move_notation);

            // Add check or checkmate notation
            if is_checkmate {
                move_str.push('#');
            } else if is_check {
                move_str.push('+');
            }

            // Format based on color: White: [full notation], Black: [full notation]
            let move_text = match piece.color {
                Color::White => move_str,
                Color::Black => move_str,
            };
            self.move_history.push(move_text);

            // Update last move highlight
            self.last_move = Some((from, to));

            // Switch active player
            self.active_player = self.active_player.opposite();
        }
    }

    pub fn is_checkmated(&self, color: Color) -> bool {
        self.board.is_checkmated(color)
    }

    pub fn is_stalemate(&self, color: Color) -> bool {
        self.board.is_stalemate(color)
    }

    /// Get the active player
    pub fn active_player(&self) -> Color {
        self.active_player
    }

    /// Reset the game to starting position
    pub fn reset(&mut self) {
        self.board = Board::new_starting_default();
        self.active_player = Color::White;
        self.selected_square = None;
        self.last_move = None;
        self.move_history.clear();
    }

    /// Get the square that needs promotion, if any
    pub fn get_promotion_square(&self) -> Option<Square> {
        self.board.get_promotion_square()
    }

    /// Promote a pawn at the given square to a new piece type
    pub fn promote_square(&mut self, square: Square, piece_type: crate::PieceType) {
        self.board.promote_square(square, piece_type);
    }
}

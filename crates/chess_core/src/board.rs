use crate::{Color, Piece, PieceType, Square};

#[derive(Clone)]
pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    en_passant: Option<Square>,
    castle_king_side_white: bool,
    castle_queen_side_white: bool,
    castle_king_side_black: bool,
    castle_queen_side_black: bool,
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: [[None; 8]; 8],
            en_passant: None,
            castle_king_side_white: true,
            castle_queen_side_white: true,
            castle_king_side_black: true,
            castle_queen_side_black: true,
        }
    }

    pub fn new_starting_default() -> Self {
        let mut board = Self::new();

        for file in 0..8 {
            board.set_piece(
                Square::new(1, file).unwrap(),
                Some(Piece::new(Color::White, PieceType::Pawn)),
            );
            board.set_piece(
                Square::new(6, file).unwrap(),
                Some(Piece::new(Color::Black, PieceType::Pawn)),
            );
        }

        let back_rank = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for (file, &piece_type) in back_rank.iter().enumerate() {
            board.set_piece(
                Square::new(0, file as u8).unwrap(),
                Some(Piece::new(Color::White, piece_type)),
            );
            board.set_piece(
                Square::new(7, file as u8).unwrap(),
                Some(Piece::new(Color::Black, piece_type)),
            );
        }

        board
    }

    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.squares[square.rank() as usize][square.file() as usize]
    }

    pub fn set_piece(&mut self, square: Square, piece: Option<Piece>) {
        self.squares[square.rank() as usize][square.file() as usize] = piece;
    }

    fn move_allows_en_passant(&mut self, piece: Piece, from: Square, to: Square) -> bool {
        if piece.piece_type != PieceType::Pawn {
            return false;
        }

        return (piece.color == Color::White && from.rank() == 1 && to.rank() == 3)
            || (piece.color == Color::Black && from.rank() == 6 && to.rank() == 4);
    }

    fn is_move_a_castle(&self, from_piece: Piece, from: Square, to: Square) -> bool {
        if from_piece.piece_type != PieceType::King {
            return false;
        }

        // white king moves to castle
        if from.rank_file() == (0, 4) && (to.rank_file() == (0, 2) || to.rank_file() == (0, 6)) {
            return true;
        }

        // black king moves to castle
        if from.rank_file() == (7, 4) && (to.rank_file() == (7, 2) || to.rank_file() == (7, 6)) {
            return true;
        }

        false
    }

    fn move_rook_to_castle(&mut self, king_to: Square) {
        let (rook_square, rook_to) = match king_to.rank_file() {
            (0, 2) => (
                Square::new(0, 0).expect("0,0 always valid"),
                Square::new(0, 3).expect("0,3 always valid"),
            ),
            (0, 6) => (
                Square::new(0, 7).expect("0,7 always valid"),
                Square::new(0, 5).expect("0,5 always valid"),
            ),
            (7, 2) => (
                Square::new(7, 0).expect("7,0 always valid"),
                Square::new(7, 3).expect("7,3 always valid"),
            ),
            (7, 6) => (
                Square::new(7, 7).expect("7,7 always valid"),
                Square::new(7, 5).expect("7,5 always valid"),
            ),
            _ => return,
        };

        let rook_piece = self.get_piece(rook_square);
        self.set_piece(rook_to, rook_piece);
        self.set_piece(rook_square, None);
    }

    fn verify_and_move_en_passant(&mut self, from_piece: Piece, to: Square) {
        if from_piece.piece_type != PieceType::Pawn {
            return;
        }

        // if from_p is attacking en_passant square
        if let Some(passant) = self.en_passant {
            let direction = match from_piece.color {
                Color::White => 1,
                Color::Black => -1,
            };

            if Ok(to) == passant.offset(direction, 0) {
                self.set_piece(passant, None);
            }
        }
    }

    fn update_castling_allowance(&mut self, from_piece: Piece, from: Square) {
        if from_piece.piece_type == PieceType::King {
            match from_piece.color {
                Color::Black => {
                    self.castle_king_side_black = false;
                    self.castle_queen_side_black = false;
                }
                Color::White => {
                    self.castle_king_side_white = false;
                    self.castle_queen_side_white = false;
                }
            }
        }

        if from_piece.piece_type == PieceType::Rook {
            match from.rank_file() {
                (0, 0) => self.castle_queen_side_white = false,
                (0, 7) => self.castle_king_side_white = false,
                (7, 0) => self.castle_queen_side_black = false,
                (7, 7) => self.castle_king_side_black = false,
                _ => return,
            };
        }
    }

    pub fn make_move(&mut self, from: Square, to: Square) {
        let from_piece = match self.get_piece(from) {
            Some(p) => p,
            None => return,
        };
        // move piece
        self.set_piece(to, Some(from_piece));
        self.set_piece(from, None);

        // handle en_passant
        self.verify_and_move_en_passant(from_piece, to);

        // handle castling
        if self.is_move_a_castle(from_piece, from, to) {
            self.move_rook_to_castle(to)
        }

        // update castling allowance every move
        self.update_castling_allowance(from_piece, from);

        // update en_passant after every move
        self.en_passant = None;
        if self.move_allows_en_passant(from_piece, from, to) {
            self.en_passant = Some(to);
        }
    }

    pub fn is_checkmated(&self, color: Color) -> bool {
        if !self.is_king_in_check(color) {
            return false;
        }

        for square in self.get_all_color_squares(color) {
            if !self.get_valid_moves(square).is_empty() {
                return false;
            }
        }

        true
    }

    pub fn is_stalemate(&self, color: Color) -> bool {
        if self.is_king_in_check(color) {
            return false;
        }

        for square in self.get_all_color_squares(color) {
            if !self.get_valid_moves(square).is_empty() {
                return false;
            }
        }

        true
    }

    pub fn get_valid_moves(&self, square: Square) -> Vec<Square> {
        let piece = match self.get_piece(square) {
            Some(p) => p,
            None => return Vec::new(),
        };

        let mut valid_moves = Vec::new();
        let unchecked_moves: Vec<Square> = self.get_all_unchecked_moves(square);
        for new_square in unchecked_moves {
            let mut new_board = self.clone();
            new_board.make_move(square, new_square);

            if !new_board.is_king_in_check(piece.color) {
                valid_moves.push(new_square);
            }
        }

        valid_moves
    }

    fn is_square_in_check(&self, check_square: Square, color: Color) -> bool {
        for rank in 0..8 {
            for file in 0..8 {
                if let Ok(square) = Square::new(rank, file) {
                    if let Some(piece) = self.get_piece(square) {
                        if piece.opposite_color() == color
                            && self.can_attack_square(square, check_square)
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn is_king_in_check(&self, color: Color) -> bool {
        let king_square = match self.find_king_square(color) {
            Some(p) => p,
            None => return true,
        };

        self.is_square_in_check(king_square, color)
    }

    fn can_king_side_castle(&self, color: Color) -> bool {
        // If any of the pieces aready moved: cannot castle
        if (color == Color::White && !self.castle_king_side_white)
            || (color == Color::Black && !self.castle_king_side_black)
        {
            return false;
        }
        let empty_squares = if color == Color::White {
            vec![(0, 5), (0, 6)]
        } else {
            vec![(7, 5), (7, 6)]
        };
        let check_squares = if color == Color::White {
            vec![(0, 4), (0, 5), (0, 6)]
        } else {
            vec![(7, 4), (7, 5), (7, 6)]
        };

        // If there is a piece between King and Rook: cannot castle
        for (rank, file) in empty_squares {
            if let Ok(square) = Square::new(rank, file) {
                if self.get_piece(square).is_some() {
                    return false;
                }
            }
        }

        // If King passes through a square that is attacked: cannot castle
        for (rank, file) in check_squares {
            if let Ok(square) = Square::new(rank, file) {
                if self.is_square_in_check(square, color) {
                    return false;
                }
            }
        }

        true
    }

    fn can_queen_side_castle(&self, color: Color) -> bool {
        // If any of the pieces aready moved: cannot castle
        if (color == Color::White && !self.castle_queen_side_white)
            || (color == Color::Black && !self.castle_queen_side_black)
        {
            return false;
        }
        let empty_squares = if color == Color::White {
            vec![(0, 1), (0, 2), (0, 3)]
        } else {
            vec![(7, 1), (7, 2), (7, 3)]
        };
        let check_squares = if color == Color::White {
            vec![(0, 2), (0, 3), (0, 4)]
        } else {
            vec![(7, 2), (7, 3), (7, 4)]
        };

        // If there is a piece between King and Rook: cannot castle
        for (rank, file) in empty_squares {
            if let Ok(square) = Square::new(rank, file) {
                if self.get_piece(square).is_some() {
                    return false;
                }
            }
        }

        // If King passes through a square that is attacked: cannot castle
        for (rank, file) in check_squares {
            if let Ok(square) = Square::new(rank, file) {
                if self.is_square_in_check(square, color) {
                    return false;
                }
            }
        }

        true
    }

    fn get_all_color_squares(&self, color: Color) -> Vec<Square> {
        let mut all_color_squared: Vec<Square> = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                if let Ok(square) = Square::new(rank, file) {
                    if let Some(piece) = self.get_piece(square) {
                        if piece.color == color {
                            all_color_squared.push(square);
                        }
                    }
                }
            }
        }

        all_color_squared
    }

    fn can_sliding_piece_reach(&self, from: Square, to: Square, piece: Piece) -> bool {
        let offsets = piece.piece_type.get_offsets();

        for &(rank_offset, file_offset) in offsets {
            for radius in 1..8 {
                if let Ok(square) = from.offset(radius * rank_offset, radius * file_offset) {
                    if square == to {
                        return true;
                    }
                    // blocked by another piece
                    if self.get_piece(square).is_some() {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        false
    }

    fn get_sliding_moves(&self, from: Square, piece: Piece) -> Vec<Square> {
        let mut all_moves = Vec::new();

        for &(rank_offset, file_offset) in piece.piece_type.get_offsets() {
            for radius in 1..8 {
                if let Ok(square) = from.offset(radius * rank_offset, radius * file_offset) {
                    if let Some(other_piece) = self.get_piece(square) {
                        // blocked - add if opposite color
                        if other_piece.color == piece.opposite_color() {
                            all_moves.push(square);
                        }
                        break;
                    } else {
                        all_moves.push(square);
                    }
                } else {
                    break;
                }
            }
        }

        all_moves
    }

    fn can_attack_square(&self, from: Square, to: Square) -> bool {
        let current = match self.get_piece(from) {
            Some(p) => p,
            None => return false,
        };

        let target = match self.get_piece(to) {
            Some(p) => p,
            None => return false,
        };

        if target.color != current.opposite_color() {
            return false;
        }

        match current.piece_type {
            PieceType::Knight => {
                return PieceType::Knight.get_offsets().iter().any(
                    |&(rank_offset, file_offset)| {
                        from.offset(rank_offset, file_offset).ok() == Some(to)
                    },
                );
            }
            PieceType::King => {
                return PieceType::King
                    .get_offsets()
                    .iter()
                    .any(|&(rank_offset, file_offset)| {
                        from.offset(rank_offset, file_offset).ok() == Some(to)
                    });
            }
            PieceType::Pawn => {
                let direction = match current.color {
                    Color::White => 1,
                    Color::Black => -1,
                };

                // ToDo: Review this
                if let Some(passant) = self.en_passant {
                    if passant.offset(0, -1).ok() == Some(to)
                        || passant.offset(0, -1).ok() == Some(to)
                    {
                        return true;
                    }
                }

                return from.offset(direction, -1).ok() == Some(to)
                    || from.offset(direction, 1).ok() == Some(to);
            }
            PieceType::Bishop | PieceType::Rook | PieceType::Queen => {
                return self.can_sliding_piece_reach(from, to, current);
            }
        }
    }

    fn find_king_square(&self, color: Color) -> Option<Square> {
        for rank in 0..8 {
            for file in 0..8 {
                if let Ok(s) = Square::new(rank, file) {
                    if let Some(p) = self.get_piece(s) {
                        if p.piece_type == PieceType::King && p.color == color {
                            return Some(s);
                        }
                    }
                }
            }
        }
        None
    }

    fn get_all_unchecked_moves(&self, square: Square) -> Vec<Square> {
        let piece = match self.get_piece(square) {
            Some(p) => p,
            None => return Vec::new(),
        };

        match piece.piece_type {
            PieceType::Knight => {
                return PieceType::Knight
                    .get_offsets()
                    .iter()
                    .filter_map(|&(rank_offset, file_offset)| {
                        let target = square.offset(rank_offset, file_offset).ok()?;
                        match self.get_piece(target) {
                            Some(p) if p.color == piece.color => None, // Can't capture own piece
                            _ => Some(target),
                        }
                    })
                    .collect();
            }
            PieceType::King => {
                let mut moves: Vec<Square> = PieceType::King
                    .get_offsets()
                    .iter()
                    .filter_map(|&(rank_offset, file_offset)| {
                        let target = square.offset(rank_offset, file_offset).ok()?;
                        match self.get_piece(target) {
                            Some(p) if p.color == piece.color => None, // Can't capture own piece
                            _ => Some(target),
                        }
                    })
                    .collect();

                let king_base_rank = if piece.color == Color::White { 0 } else { 7 };
                if self.can_king_side_castle(piece.color) {
                    if let Ok(move_square) = Square::new(king_base_rank, 6) {
                        moves.push(move_square);
                    }
                }
                if self.can_queen_side_castle(piece.color) {
                    if let Ok(move_square) = Square::new(king_base_rank, 2) {
                        moves.push(move_square);
                    }
                }

                return moves;
            }
            PieceType::Pawn => {
                let mut all_moves = Vec::new();
                let (direction, starting_rank) = match piece.color {
                    Color::White => (1, 1),
                    Color::Black => (-1, 6),
                };

                // Diagonal captures only
                for offset in [-1, 1] {
                    if let Ok(s) = square.offset(direction, offset) {
                        if let Some(p) = self.get_piece(s) {
                            if p.color == piece.opposite_color() {
                                all_moves.push(s);
                            }
                        }
                    }
                }

                // Forward move
                if let Ok(s1) = square.offset(direction, 0) {
                    if self.get_piece(s1).is_none() {
                        all_moves.push(s1);

                        // Double move from starting position
                        if square.rank() == starting_rank {
                            if let Ok(s2) = square.offset(direction * 2, 0) {
                                if self.get_piece(s2).is_none() {
                                    all_moves.push(s2);
                                }
                            }
                        }
                    }
                }

                // En Passant rule
                if let Some(passant) = self.en_passant {
                    for offset in [-1, 1] {
                        if let Ok(square_adjacent) = square.offset(0, offset) {
                            if Some(square_adjacent) == self.en_passant {
                                if let Some(passant_piece) = self.get_piece(square_adjacent) {
                                    if passant_piece.color == piece.opposite_color() {
                                        // opponent is in en_passant square
                                        println!("trying en_passant {passant:?}");
                                        if let Ok(s) = square.offset(direction, offset) {
                                            all_moves.push(s);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                return all_moves;
            }
            PieceType::Bishop | PieceType::Rook | PieceType::Queen => {
                return self.get_sliding_moves(square, piece);
            }
        }
    }

    pub fn get_promotion_square(&self) -> Option<Square> {
        // there can only be one promotion square
        for rank in [0, 7] {
            for file in 0..8 {
                let square = Square::new(rank, file).expect("always valid");
                if let Some(piece) = self.get_piece(square) {
                    if piece.piece_type == PieceType::Pawn {
                        return Some(square);
                    }
                }
            }
        }

        None
    }

    pub fn promote_square(&mut self, square: Square, promote_to: PieceType) {
        if let Some(mut piece) = self.get_piece(square) {
            piece.promote_to(promote_to);
            self.set_piece(square, Some(piece));
        }
    }
}

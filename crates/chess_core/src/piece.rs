#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn get_offsets(&self) -> &'static [(i8, i8)] {
        match self {
            PieceType::Knight => &[
                (-2, -1),
                (-2, 1),
                (-1, -2),
                (-1, 2),
                (1, -2),
                (1, 2),
                (2, -1),
                (2, 1),
            ],
            PieceType::Bishop => &[(-1, -1), (-1, 1), (1, -1), (1, 1)],
            PieceType::Rook => &[(-1, 0), (0, -1), (0, 1), (1, 0)],
            PieceType::Queen => &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
            PieceType::King => &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
            _ => &[],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self { color, piece_type }
    }

    pub fn opposite_color(&self) -> Color {
        self.color.opposite()
    }

    pub fn get_piece_ascii(&self) -> String {
        match self.color {
            Color::White => match self.piece_type {
                PieceType::King => "♔".to_string(),
                PieceType::Queen => "♕".to_string(),
                PieceType::Rook => "♖".to_string(),
                PieceType::Bishop => "♗".to_string(),
                PieceType::Knight => "♘".to_string(),
                PieceType::Pawn => "♙".to_string(),
            },
            Color::Black => match self.piece_type {
                PieceType::King => "♚".to_string(),
                PieceType::Queen => "♛".to_string(),
                PieceType::Rook => "♜".to_string(),
                PieceType::Bishop => "♝".to_string(),
                PieceType::Knight => "♞".to_string(),
                PieceType::Pawn => "♟".to_string(),
            },
        }
    }

    pub fn promote_to(&mut self, piece_type: PieceType) {
        self.piece_type = piece_type;
    }
}

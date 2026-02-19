use std::collections::HashMap;
use egui::{ColorImage, TextureHandle, Context, Vec2};
use chess_core::{Color, PieceType};


pub struct PieceAssets {
    textures: HashMap<(Color, PieceType), TextureHandle>
}

impl PieceAssets {
    pub fn new(ctx: &Context) -> Self {
        let mut textures = HashMap::new();

        // Load all piece SVGs
        let pieces = [
            (Color::White, PieceType::King, include_str!("../../assets/pieces/white_king.svg")),
            (Color::White, PieceType::Queen, include_str!("../../assets/pieces/white_queen.svg")),
            (Color::White, PieceType::Rook, include_str!("../../assets/pieces/white_rook.svg")),
            (Color::White, PieceType::Bishop, include_str!("../../assets/pieces/white_bishop.svg")),
            (Color::White, PieceType::Knight, include_str!("../../assets/pieces/white_knight.svg")),
            (Color::White, PieceType::Pawn, include_str!("../../assets/pieces/white_pawn.svg")),
            (Color::Black, PieceType::King, include_str!("../../assets/pieces/black_king.svg")),
            (Color::Black, PieceType::Queen, include_str!("../../assets/pieces/black_queen.svg")),
            (Color::Black, PieceType::Rook, include_str!("../../assets/pieces/black_rook.svg")),
            (Color::Black, PieceType::Bishop, include_str!("../../assets/pieces/black_bishop.svg")),
            (Color::Black, PieceType::Knight, include_str!("../../assets/pieces/black_knight.svg")),
            (Color::Black, PieceType::Pawn, include_str!("../../assets/pieces/black_pawn.svg")),
        ];

        for (color, piece_type, svg_data) in pieces {
            if let Some(texture) = Self::svg_to_texture(ctx, svg_data, 128, color, piece_type) {
                textures.insert((color, piece_type), texture);
            }
        }

        Self { textures }
    }

    pub fn get(&self, color: Color, piece_type: PieceType) -> Option<&TextureHandle> {
        self.textures.get(&(color, piece_type))
    }

    fn svg_to_texture(ctx: &Context, svg_data: &str, size: u32, color: Color, piece_type: PieceType) -> Option<TextureHandle>{
        let opt = usvg::Options::default();
        let tree = usvg::Tree::from_str(svg_data, &opt).ok()?;
        
        let mut pixmap = tiny_skia::Pixmap::new(size, size)?;
        
        // SVG -> pixmap
        let tree_size = tree.size();
        let scale_x = size as f32 / tree_size.width();
        let scale_y = size as f32 / tree_size.height();
        let scale = scale_x.min(scale_y);
        
        let transform = tiny_skia::Transform::from_scale(scale, scale);
        
        resvg::render(&tree, transform, &mut pixmap.as_mut());
        
        // convert to egui ColorImage
        let pixels = pixmap.data();
        let mut color_image = ColorImage {
            size: [size as usize, size as usize],
            pixels: Vec::with_capacity((size * size) as usize),
            source_size: Vec2::new(size as f32, size as f32),
        };
        
        // convert RGBA to egui format
        for chunk in pixels.chunks(4) {
            color_image.pixels.push(egui::Color32::from_rgba_premultiplied(
                chunk[0], chunk[1], chunk[2], chunk[3],
            ));
        }
        
        // create texture
        Some(ctx.load_texture(
            format!("piece_texture_{:?}_{:?}", color, piece_type),
            color_image,
            egui::TextureOptions::LINEAR,
        ))
    }

}
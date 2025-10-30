use cursive::theme::BaseColor;
use cursive::theme::Color;

pub type Tile = Option<Block>;

#[derive(Copy, Clone)]
pub enum Block {
    Red,
    Green,
    Blue,
    Magenta,
    Yellow,
    Cyan,
    Black,
    White,
}

impl Block {
    pub fn get_color(&self) -> cursive::theme::Color {
        match self {
            Block::Red => Color::Dark(cursive::theme::BaseColor::Red),
            Block::Green => Color::Dark(BaseColor::Green),
            Block::Blue => Color::Dark(BaseColor::Blue),
            Block::Magenta => Color::Dark(BaseColor::Magenta),
            Block::Yellow => Color::Dark(BaseColor::Yellow),
            Block::Cyan => Color::Dark(BaseColor::Cyan),
            Block::Black => Color::Dark(BaseColor::Black),
            Block::White => Color::Dark(BaseColor::White),
        }
    }
}

enum PieceType {
    I,
    O,
    J,
    L,
    S,
    Z,
    T,
}

impl PieceType {
    fn get_colored_block(&self) -> Block {
        match self {
            PieceType::I => Block::Cyan,
            PieceType::O => Block::Yellow,
            PieceType::J => Block::Blue,
            PieceType::L => Block::White, // maybe change
            PieceType::S => Block::Green,
            PieceType::Z => Block::Red,
            PieceType::T => Block::Magenta,
        }
    }
    // for checking if rotation is possible
    fn get_rot_diameter(&self) -> usize {
        match self {
            PieceType::I => 4,
            PieceType::O => 4,
            _ => 3,
        }
    }
    fn get_layout(&self) -> [[Tile; 4]; 4] {
        static I_LAYOUT: [[u8; 4]; 4] = [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]];

        [[None; 4]; 4] // TODO, placeholder
    }
    fn convert_bitmap_to_layout(&self, bitmap: [[u8; 4]; 4]) -> [[Tile; 4]; 4] {
        let mut layout: [[Tile; 4]; 4] = [[None; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                if bitmap[i][j] == 1 {
                    layout[i][j] = Some(self.get_colored_block());
                }
            }
        }
        layout
    }
}

struct Piece {
    piece_type: PieceType,
    layout: [[Tile; 4]; 4],
}

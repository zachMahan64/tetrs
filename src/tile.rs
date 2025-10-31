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

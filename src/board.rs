use crate::constants;
use crate::constants::BOARD_HEIGHT;
use crate::constants::BOARD_WIDTH;
use cursive::Printer;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::theme::Color;

type Tile = Option<Block>;

#[derive(Copy, Clone)]
enum Block {
    Red,
    Green,
    Blue,
    Magenta,
    Yellow,
    Cyan,
    Black,
}

impl Block {
    fn get_cursive_color(&self) -> cursive::theme::Color {
        match self {
            Block::Red => Color::Rgb(255, 0, 0),
            Block::Green => Color::Rgb(0, 255, 0),
            Block::Blue => Color::Rgb(0, 0, 255),
            Block::Magenta => Color::Rgb(255, 0, 255),
            Block::Yellow => Color::Rgb(255, 255, 0),
            Block::Cyan => Color::Rgb(0, 255, 255),
            Block::Black => Color::Rgb(0, 0, 0),
        }
    }
}

pub struct Board {
    tiles: [[Tile; constants::BOARD_WIDTH]; constants::BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Board {
            tiles: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        // TODO, impl this?
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        // TODO
        //self.handle_event(event, false);
        EventResult::Consumed(None)
    }
    fn draw(&self, printer: &Printer) {
        // TODO, draw the board
        //printer.print(cursive::XY::new(0, 0), &self.tiles);
    }
}

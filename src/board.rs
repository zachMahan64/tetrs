use crate::constants;
use crate::constants::BOARD_HEIGHT;
use crate::constants::BOARD_WIDTH;
use crate::text_art::BLOCK_CHAR;
use cursive::Printer;
use cursive::View;
use cursive::XY;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::style;
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
    fn get_color(&self) -> cursive::theme::Color {
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
    /*
        fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
            // TODO, impl this?
        }
    */
}

impl View for Board {
    fn on_event(&mut self, event: Event) -> EventResult {
        // TODO
        //self.handle_event(event, false);
        EventResult::Consumed(None)
    }
    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        let board_width_in_chars = BOARD_WIDTH * 2;
        let board_height_in_chars = BOARD_HEIGHT * 2;
        (board_width_in_chars, board_height_in_chars).into()
    }

    fn draw(&self, printer: &Printer) {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                let tile = self.tiles[i][j];
                let i = 2 * i;
                let j = 2 * j;
                let coords = [(i, j), (i + 1, j), (i, j + 1), (i + 1, j + 1)];
                for coord in coords {
                    match tile {
                        Some(block) => {
                            printer.with_style(block.get_color(), |p| {
                                p.print(coord, BLOCK_CHAR);
                            });
                        }
                        None => {
                            printer.with_style(Color::Rgb(0, 0, 0), |p| {
                                p.print(coord, BLOCK_CHAR);
                            });
                        }
                    }
                }
            }
        }
    }
}

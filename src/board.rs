use crate::constants;
use crate::constants::BOARD_HEIGHT;
use crate::constants::BOARD_WIDTH;
use crate::text_art::BLOCK_CHAR;
use cursive::Printer;
use cursive::View;
use cursive::event::Event;
use cursive::event::EventResult;
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

enum ScaleMode {
    Small,
    Large,
}

impl ScaleMode {
    fn get_scale(&self) -> usize {
        match self {
            Self::Small => 1,
            Self::Large => 2,
        }
    }
    fn default() -> Self {
        Self::Large
    }
}

pub struct Board {
    scale_mode: ScaleMode,
    tiles: [[Tile; constants::BOARD_WIDTH]; constants::BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            scale_mode: ScaleMode::default(), // make variable eventually
            tiles: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
        };
        // TODO: test
        board.tiles[2][4] = Some(Block::Cyan);
        board.tiles[3][4] = Some(Block::Cyan);
        board.tiles[3][5] = Some(Block::Cyan);
        board.tiles[3][6] = Some(Block::Cyan);
        board
    }

    fn for_each_tile<F: FnMut((usize, usize), Tile)>(&self, mut f: F) {
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, &tile) in row.iter().enumerate() {
                f((i, j), tile);
            }
        }
    }
    // helper
    fn draw_tile(printer: &Printer, tile: Option<Block>, coord: (usize, usize)) {
        match tile {
            Some(block) => printer.with_style(block.get_color(), |p| p.print(coord, BLOCK_CHAR)),
            None => printer.with_style(Color::Rgb(0, 0, 0), |p| p.print(coord, BLOCK_CHAR)),
        }
    }
}

impl View for Board {
    fn on_event(&mut self, event: Event) -> EventResult {
        // TODO
        //self.handle_event(event, false);
        EventResult::Consumed(None)
    }
    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        let board_width_in_chars = BOARD_WIDTH * 2 * self.scale_mode.get_scale();
        let board_height_in_chars = BOARD_HEIGHT * self.scale_mode.get_scale();
        (board_width_in_chars, board_height_in_chars).into()
    }

    fn draw(&self, printer: &Printer) {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                let tile = self.tiles[i][j];
                let i = self.scale_mode.get_scale() * i;
                // constant 2 to account for characters inheritantly being narrow
                let j = self.scale_mode.get_scale() * j * 2;
                match self.scale_mode {
                    ScaleMode::Small => {
                        // 2 chars wide, 1 char tall
                        for dx in 0..2 {
                            Board::draw_tile(printer, tile, (j + dx, i));
                        }
                    }
                    ScaleMode::Large => {
                        for dx in 0..4 {
                            // 4 chars wide
                            for dy in 0..2 {
                                // 2 chars tall
                                Board::draw_tile(printer, tile, (j + dx, i + dy));
                            }
                        }
                    }
                }
            }
        }
        printer.with_style(Color::Rgb(255, 0, 0), |p| {
            p.print(
                (
                    self.tiles[0].len() * 2 * self.scale_mode.get_scale() - 1,
                    self.tiles.len() * self.scale_mode.get_scale() - 1,
                ),
                BLOCK_CHAR,
            );
        });
    }
}

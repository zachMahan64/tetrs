use crate::constants;
use crate::constants::BOARD_HEIGHT;
use crate::constants::BOARD_WIDTH;
use crate::tetrs;
use crate::text_art::BLOCK_CHAR;
use crate::tile::Block;
use crate::tile::Tile;
use cursive::Printer;
use cursive::View;
use cursive::direction::Direction;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::event::Key;
use cursive::theme::BaseColor;
use cursive::theme::Color;
use cursive::view::CannotFocus;
use cursive::views::TextView;
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

    // helper
    fn draw_tile(printer: &Printer, tile: Option<Block>, coord: (usize, usize)) {
        match tile {
            Some(block) => printer.with_style(block.get_color(), |p| p.print(coord, BLOCK_CHAR)),
            None => printer.with_style(Color::Dark(BaseColor::Black), |p| {
                p.print(coord, BLOCK_CHAR)
            }),
        }
    }

    fn handle_event(&self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Left) => EventResult::with_cb(|s| {
                s.call_on_name("action", |t: &mut TextView| {
                    t.set_content("Left!");
                });
            }),
            Event::Key(Key::Right) => EventResult::with_cb(|s| {
                s.call_on_name("action", |t: &mut TextView| {
                    t.set_content("Right!");
                });
            }),
            Event::Char('z') => EventResult::with_cb(|s| {
                s.call_on_name("action", |t: &mut TextView| {
                    t.set_content("Rotated Left!");
                });
            }),
            Event::Char('x') => EventResult::with_cb(|s| {
                s.call_on_name("action", |t: &mut TextView| {
                    t.set_content("Rotated Right!");
                });
            }),
            _ => EventResult::Ignored,
        }
    }
}

impl View for Board {
    fn on_event(&mut self, event: Event) -> EventResult {
        self.handle_event(event)
    }
    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }

    fn required_size(&mut self, constraint: cursive::XY<usize>) -> cursive::XY<usize> {
        let large_x = BOARD_WIDTH * 2 * ScaleMode::Large.get_scale();
        let large_y = BOARD_HEIGHT * ScaleMode::Large.get_scale();

        if large_x > constraint.pair().0 || large_y > constraint.pair().1 {
            self.scale_mode = ScaleMode::Small;
        } else {
            self.scale_mode = ScaleMode::Large;
        }

        let dimen_x = BOARD_WIDTH * 2 * self.scale_mode.get_scale();
        let dimen_y = BOARD_HEIGHT * self.scale_mode.get_scale();
        (dimen_x, dimen_y).into()
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
        // TODO: test print red rect at bottom right corner
        printer.with_style(Color::Dark(BaseColor::Red), |p| {
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

use crate::piece::Piece;
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
use cursive::view::Margins;
use cursive::views::DummyView;
use cursive::views::PaddedView;
use cursive::views::TextView;

static BOARD_WIDTH: usize = 10;
static BOARD_HEIGHT: usize = 20;

#[derive(PartialEq, Clone, Copy)]
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
    fn get_side_stack_margins(&self) -> Margins {
        match self {
            Self::Small => Margins::lrtb(8, 9, 0, 0),
            ScaleMode::Large => Margins::lrtb(8, 9, 10, 0),
        }
    }
}

pub struct Board {
    // board layout things
    scale_mode: ScaleMode,
    tiles: [[Tile; BOARD_WIDTH]; BOARD_HEIGHT],
    needs_relayout: bool,
    // current piece things
    current_piece: Piece,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            scale_mode: ScaleMode::default(),
            tiles: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
            needs_relayout: false,
            current_piece: Piece::random_new(), // TODO placeholder
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

    fn handle_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Refresh => self.handle_refresh(),
            Event::Key(Key::Left) => EventResult::with_cb(|s| {
                s.call_on_name("action", |t: &mut TextView| {
                    t.set_content("Moved Left!");
                });
            }),
            Event::Key(Key::Right) => EventResult::with_cb(|s| {
                s.call_on_name("action", |t: &mut TextView| {
                    t.set_content("Moved Right!");
                });
            }),
            Event::Key(Key::Down) => EventResult::with_cb(|s| {
                s.call_on_name("action", |t: &mut TextView| {
                    t.set_content("Moved Down!");
                });
            }),
            Event::Key(Key::Up) => EventResult::with_cb(|s| {
                s.call_on_name("action", |t: &mut TextView| {
                    t.set_content("Fast Dropped!");
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
    // handle refresh logic, like what to do relayout is needed
    fn handle_refresh(&mut self) -> EventResult {
        if !self.needs_relayout {
            return EventResult::Ignored;
        }
        self.needs_relayout = false; //reset 
        let margins = self.scale_mode.get_side_stack_margins();
        EventResult::with_cb(move |s| {
            s.call_on_name("padded", |t: &mut PaddedView<DummyView>| {
                t.set_margins(margins);
            });
        })
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
        let starting_scale = self.scale_mode.clone();
        let large_x = BOARD_WIDTH * 2 * ScaleMode::Large.get_scale();
        let large_y = BOARD_HEIGHT * ScaleMode::Large.get_scale();

        if large_x > constraint.pair().0 || large_y > constraint.pair().1 {
            self.scale_mode = ScaleMode::Small;
        } else {
            self.scale_mode = ScaleMode::Large;
        }
        let updated_scale = self.scale_mode.clone();

        if starting_scale != updated_scale {
            self.needs_relayout = true;
        }

        let dimen_x = BOARD_WIDTH * 2 * self.scale_mode.get_scale();
        let dimen_y = BOARD_HEIGHT * self.scale_mode.get_scale();
        (dimen_x, dimen_y).into()
    }

    fn draw(&self, printer: &Printer) {
        // rendering logic for current piece, TODO: handle overlap and scale issues here
        for i in 0..self.current_piece.layout.len() {
            for j in 0..self.current_piece.layout[i].len() {
                Board::draw_tile(
                    printer,
                    self.current_piece.layout[i][j],
                    (
                        j + self.current_piece.coord.0 as usize, // TODO handle negatives properly
                        i + self.current_piece.coord.1 as usize, // TODO handle negatives properly
                    ),
                );
            }
        }
        // rendering logic for static board
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

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
use cursive::view::scroll::layout;
use cursive::views::DummyView;
use cursive::views::PaddedView;
use cursive::views::TextView;
use std::time;
use std::time::Instant;

pub static BOARD_WIDTH: usize = 10;
pub static BOARD_HEIGHT: usize = 20;

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

    // piece things
    current_piece: Piece,
    next_piece: Piece,
    last_tick: time::Instant,
    tick_time: time::Duration, // make this vary by level/difficulty

    //stats
    score: u32,
    level: u8,
}

impl Board {
    pub fn new() -> Self {
        const STARTING_TICK_TIME_MILLIS: u64 = 1000;
        Board {
            scale_mode: ScaleMode::default(),
            tiles: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
            needs_relayout: false,
            // TODO impl proper piece spawning
            current_piece: Piece::random_new().at(5, -1),
            next_piece: Piece::random_new(),
            last_tick: time::Instant::now(),
            tick_time: time::Duration::from_millis(STARTING_TICK_TIME_MILLIS),

            // stats
            score: 0,
            level: 1,
        }
    }
    fn draw_tile(&self, printer: &Printer, tile: Tile, row: usize, col: usize) {
        let i = self.scale_mode.get_scale() * row;
        // constant 2 to account for characters inheritantly being narrow
        let j = self.scale_mode.get_scale() * col * 2;
        match self.scale_mode {
            ScaleMode::Small => {
                // 2 chars wide, 1 char tall
                for dx in 0..2 {
                    Board::draw_tile_char(printer, tile, (j + dx, i));
                }
            }
            ScaleMode::Large => {
                for dx in 0..4 {
                    // 4 chars wide
                    for dy in 0..2 {
                        // 2 chars tall
                        Board::draw_tile_char(printer, tile, (j + dx, i + dy));
                    }
                }
            }
        }
    }
    // helper
    fn draw_tile_char(printer: &Printer, tile: Option<Block>, coord: (usize, usize)) {
        match tile {
            Some(block) => printer.with_style(block.get_color(), |p| p.print(coord, BLOCK_CHAR)),
            None => printer.with_style(Color::Dark(BaseColor::Black), |p| {
                p.print(coord, BLOCK_CHAR)
            }),
        }
    }

    fn handle_event(&mut self, event: Event) -> EventResult {
        match event {
            // refresh handles gravity logic
            Event::Refresh => self.handle_refresh(),
            Event::Key(Key::Left) => {
                self.try_piece_movement(&Piece::move_left);
                EventResult::with_cb(|s| {
                    s.call_on_name("action", |t: &mut TextView| {
                        t.set_content("Moved Left!");
                    });
                })
            }
            Event::Key(Key::Right) => {
                self.try_piece_movement(&Piece::move_right);
                EventResult::with_cb(|s| {
                    s.call_on_name("action", |t: &mut TextView| {
                        t.set_content("Moved Right!");
                    });
                })
            }
            Event::Key(Key::Down) => {
                self.try_piece_movement(&Piece::move_down);
                EventResult::with_cb(|s| {
                    s.call_on_name("action", |t: &mut TextView| {
                        t.set_content("Moved Down!");
                    });
                })
            }
            Event::Key(Key::Up) => EventResult::with_cb(|s| {
                s.call_on_name("action", |t: &mut TextView| {
                    t.set_content("Fast Dropped!");
                });
            }),

            Event::Char('z') => {
                self.try_piece_movement(&Piece::rotate_left);
                EventResult::with_cb(|s| {
                    s.call_on_name("action", |t: &mut TextView| {
                        t.set_content("Rotated Left!");
                    });
                })
            }
            Event::Char('x') => {
                self.try_piece_movement(&Piece::rotate_right);
                EventResult::with_cb(|s| {
                    s.call_on_name("action", |t: &mut TextView| {
                        t.set_content("Rotated Right!");
                    });
                })
            }
            _ => EventResult::Ignored,
        }
    }
    // handle refresh logic, like what to do relayout is needed
    fn handle_refresh(&mut self) -> EventResult {
        // check to move down current piece
        self.check_to_tick_down_piece();
        // check to see if we need to relayout
        return self.check_to_relayout();
    }
    fn check_to_tick_down_piece(&mut self) {
        let now = Instant::now();
        if now > self.last_tick + self.tick_time {
            // TODO placement and next piece logic
            self.try_piece_movement(&Piece::move_down);
            self.last_tick = now;
        }
    }
    fn check_to_relayout(&mut self) -> EventResult {
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
    fn try_piece_movement<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(&mut Piece),
    {
        let mut temp = self.current_piece.clone();
        // try movement by transforming temp
        f(&mut temp);

        if self.valid_piece(&temp) {
            self.current_piece = temp;
            true
        } else {
            //don't transform current piece
            false
        }
    }
    fn valid_piece(&self, piece: &Piece) -> bool {
        // this order matters, checking intersection can get an out of bounds error if
        // we don't check bounds first
        !piece.is_out_of_bounds() && !self.check_if_piece_intersects_any_blocks(piece)
    }
    fn check_if_piece_intersects_any_blocks(&self, piece: &Piece) -> bool {
        for i in 0..piece.layout().len() {
            for j in 0..piece.layout()[i].len() {
                let tile = piece.layout()[i][j];
                if tile.is_none() {
                    continue; // we do not care, no block
                }
                let x = j as i8 + piece.coord().0;
                let y = i as i8 + piece.coord().1;

                // out of bounds guard to be extra safe
                if x < 0 || y < 0 || x >= BOARD_WIDTH as i8 || y >= BOARD_HEIGHT as i8 {
                    continue;
                }
                match self.tiles[y as usize][x as usize] {
                    None => continue,
                    Some(_) => return true,
                }
            }
        }
        false
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
        // rendering logic for static board
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].len() {
                let tile = self.tiles[i][j];
                self.draw_tile(printer, tile, i, j);
            }
        }
        // draw piece AFTER board, simply "project" it onto the board
        for i in 0..self.current_piece.layout().len() {
            for j in 0..self.current_piece.layout()[i].len() {
                let tile = self.current_piece.layout()[i][j];
                let row = self.current_piece.coord().1 + i as i8;
                let col = self.current_piece.coord().0 + j as i8;
                // don't attempt to print negatives
                if row < 0 || col < 0 {
                    continue;
                }
                match tile {
                    // don't draw black tiles on None becuz we don't want to overwrite anything on
                    // static board
                    None => {}
                    _ => self.draw_tile(printer, tile, row as usize, col as usize),
                }
            }
        }
    }
}

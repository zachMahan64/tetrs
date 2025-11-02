use crate::ids;
use crate::piece::Piece;
use crate::piece::PieceView;
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
use cursive::view::Margins;
use cursive::views::Dialog;
use cursive::views::DummyView;
use cursive::views::PaddedView;
use cursive::views::TextView;
use std::cmp::min;
use std::time;
use std::time::Instant;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const PIECE_START_X: i8 = 4;
pub const PIECE_START_Y: i8 = -1;

const MAX_LEVEL: u8 = u8::MAX; //theoretically...

#[derive(PartialEq, Clone, Copy)]
enum ScaleMode {
    TooSmall,
    Small,
    Large,
}

impl ScaleMode {
    fn get_scale(&self) -> usize {
        match self {
            Self::TooSmall => 1,
            Self::Small => 1,
            Self::Large => 2,
        }
    }
    fn default() -> Self {
        Self::Small
    }
    // useless currently, but could be useful
    fn get_right_stack_margins(&self) -> Margins {
        match self {
            Self::Small | Self::TooSmall => Margins::lrtb(8, 8, 0, 0),
            ScaleMode::Large => Margins::lrtb(8, 8, 0, 0),
        }
    }
}

pub struct Board {
    // board layout things
    scale_mode: ScaleMode,
    tiles: [[Tile; BOARD_WIDTH]; BOARD_HEIGHT],
    needs_relayout: bool,
    loss_state: LossState,

    // piece things
    current_piece: Piece,
    next_piece: Piece,
    last_tick: time::Instant,
    tick_time: time::Duration, // make this vary by level/difficulty

    //stats
    score: u32,
    lines: u32,
    level: u8,
    starting_level: u8,
    high_score: u32,

    // settable settings,
    ghost_piece_on: bool,
}

#[derive(Clone, Copy)]
enum LossState {
    NotLost,
    Lost,
}

enum TickState {
    NotTicked,
    Ticked,
}

pub struct BoardSettings {
    pub starting_level: u8,
    pub ghost_piece_on: bool,
}

impl Board {
    pub fn new(settings: BoardSettings) -> Self {
        const STARTING_TICK_TIME_MILLIS: u64 = 1000;
        let mut board = Board {
            // static board stuff
            scale_mode: ScaleMode::default(),
            tiles: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
            needs_relayout: false,
            loss_state: LossState::NotLost,

            // TODO impl proper piece spawning, maybe add a "bag feature"
            current_piece: Piece::random_new().at(PIECE_START_X, PIECE_START_Y),
            next_piece: Piece::random_new().at(PIECE_START_X, PIECE_START_Y),
            last_tick: time::Instant::now(),
            tick_time: time::Duration::from_millis(STARTING_TICK_TIME_MILLIS),

            // stats
            score: 0,
            lines: 0,
            level: settings.starting_level,
            starting_level: settings.starting_level,
            high_score: 0,

            // settings
            ghost_piece_on: settings.ghost_piece_on,
        };
        board.update_tick_time();
        board
    }
    pub fn get_settings(&self) -> BoardSettings {
        BoardSettings {
            starting_level: self.starting_level,
            ghost_piece_on: self.ghost_piece_on,
        }
    }
    fn restart(&mut self) {
        let old_high_score = self.high_score;
        let latest_score = self.score;
        *self = Board::new(self.get_settings());
        self.high_score = match latest_score > old_high_score {
            true => latest_score,
            false => old_high_score,
        };
    }
    fn draw_tile(&self, printer: &Printer, tile: Tile, row: usize, col: usize) {
        let i = self.scale_mode.get_scale() * row;
        // constant 2 to account for characters inheritantly being narrow
        let j = self.scale_mode.get_scale() * col * 2;
        match self.scale_mode {
            ScaleMode::Small | ScaleMode::TooSmall => {
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
            Event::Refresh => self.on_refresh(),
            Event::Key(Key::Left) => {
                self.try_current_piece_movement(&Piece::move_left);
                EventResult::with_cb(|s| {
                    s.call_on_name(ids::ACTION, |t: &mut TextView| {
                        t.set_content("Move Left!");
                    });
                })
            }
            Event::Key(Key::Right) => {
                self.try_current_piece_movement(&Piece::move_right);
                EventResult::with_cb(|s| {
                    s.call_on_name(ids::ACTION, |t: &mut TextView| {
                        t.set_content("Move Right!");
                    });
                })
            }
            Event::Key(Key::Down) => {
                if !self.try_current_piece_movement(&Piece::move_down) {
                    self.consume_piece();
                }
                EventResult::with_cb(|s| {
                    s.call_on_name(ids::ACTION, |t: &mut TextView| {
                        t.set_content("Move Down!");
                    });
                })
            }
            Event::Key(Key::Up) => {
                while self.try_current_piece_movement(&Piece::move_down) {}
                self.consume_piece();
                EventResult::with_cb(|s| {
                    s.call_on_name(ids::ACTION, |t: &mut TextView| {
                        t.set_content("Fast Drop!");
                    });
                })
            }

            Event::Char('z') => {
                self.try_current_piece_movement(&Piece::rotate_left);
                EventResult::with_cb(|s| {
                    s.call_on_name(ids::ACTION, |t: &mut TextView| {
                        t.set_content("Rotate Left!");
                    });
                })
            }
            Event::Char('x') => {
                self.try_current_piece_movement(&Piece::rotate_right);
                EventResult::with_cb(|s| {
                    s.call_on_name(ids::ACTION, |t: &mut TextView| {
                        t.set_content("Rotate Right!");
                    });
                })
            }
            _ => EventResult::Ignored,
        }
    }
    // handle refresh logic, like what to do relayout is needed
    fn on_refresh(&mut self) -> EventResult {
        // check to move down current piece
        let tick_state: TickState = self.check_to_tick_down_piece();
        match tick_state {
            TickState::NotTicked => self.handle_no_tick(),
            TickState::Ticked => self.handle_tick(),
        }
    }

    fn check_to_tick_down_piece(&mut self) -> TickState {
        let now = Instant::now();
        if now < self.last_tick + self.tick_time {
            return TickState::NotTicked; // we haven't ticked yet
        }
        self.last_tick = now;
        // only consume piece and check loss if it can't move
        if !self.try_current_piece_movement(&Piece::move_down) {
            self.consume_piece();
        }
        TickState::Ticked
    }
    // sets self.LossState and also returns true if lost
    fn consume_piece(&mut self) -> bool {
        let piece = &self.current_piece;
        for i in 0..piece.layout().len() {
            for j in 0..piece.layout()[i].len() {
                let piece_tile = piece.layout()[i][j];
                if piece_tile.is_none() {
                    continue; // we do not care, no block in this tile
                }
                let x = j as i8 + piece.coord().0;
                let y = i as i8 + piece.coord().1;
                // piece too high, loss
                if y < 0 {
                    self.loss_state = LossState::Lost;
                    return true;
                }
                self.tiles[y as usize][x as usize] = piece_tile;
            }
        }
        self.current_piece = self.next_piece;
        self.next_piece = Piece::random_new().at(PIECE_START_X, PIECE_START_Y);
        self.score += 1; // give pity point
        // check to clear any lines that are now full after consuming a piece
        self.clear_any_full_lines();
        // update level and tick time accordingly
        self.level = self.starting_level
            + min(
                MAX_LEVEL as u32,
                self.lines as u32 / (10 * self.starting_level as u32),
            ) as u8;
        self.update_tick_time();
        false
    }
    // helper
    fn update_tick_time(&mut self) {
        const CURVE: u64 = 15; // to not be too hard
        let millis = match self.level {
            0 => 800,
            1 => 717,
            2 => 633,
            3 => 550,
            4 => 467,
            5 => 383,
            6 => 300,
            7 => 217,
            8 => 133,
            9 => 100,
            10..=12 => 83 + CURVE,
            13..=15 => 67 + CURVE,
            16..=18 => 50 + CURVE,
            19..=28 => 33 + CURVE,
            _ => 17,
        };
        self.tick_time = std::time::Duration::from_millis(millis);
    }
    // clears any full lines on the board
    fn clear_any_full_lines(&mut self) {
        let mut num_cleared = 0;
        let mut i = BOARD_HEIGHT as isize - 1;
        while i >= 0 {
            if self.tiles[i as usize].iter().all(|t| t.is_some()) {
                num_cleared += 1;
                self.clear_line_and_shift_down(i as usize);
                i += 1; // recheck the same row after shifting down
            }
            i -= 1;
        }
        self.award_points(num_cleared);
        self.lines += num_cleared as u32;
    }
    // helper for clear_any_full_lines
    fn clear_line_and_shift_down(&mut self, row: usize) {
        for i in (1..=row).rev() {
            self.tiles[i] = self.tiles[i - 1];
        }
        self.tiles[0] = [None; BOARD_WIDTH];
    }

    fn award_points(&mut self, num_cleared: u8) {
        let mut points: u32 = match num_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0, // not possible
        };
        points = points * self.level as u32;
        self.score += points;
    }
    fn handle_tick(&mut self) -> EventResult {
        if self.needs_relayout {
            self.needs_relayout = false; //reset 
        }
        let margins = self.scale_mode.get_right_stack_margins();
        let score = self.score;
        let level = self.level;
        let lines = self.lines;
        let high_score = self.high_score;
        let loss_state = self.loss_state;
        let next_piece = self.next_piece.clone();

        match loss_state {
            LossState::NotLost => {}
            LossState::Lost => self.restart(),
        }

        EventResult::with_cb(move |s| {
            match loss_state {
                LossState::NotLost => {}
                LossState::Lost => {
                    let game_over_title = match score > high_score {
                        true => "New High Score!",
                        false => "Game Over!",
                    };
                    s.add_layer(
                        Dialog::around(
                            TextView::new(format!(
                                "Score: {}\nLines: {} \nLevel: {}",
                                score, lines, level
                            ))
                            .center(),
                        )
                        .title(game_over_title)
                        .button("Play Again", |s| {
                            s.pop_layer();
                        })
                        .button("Return to Title", |s| {
                            s.pop_layer();
                            s.pop_layer();
                            tetrs::show_title_menu(s);
                        }),
                    );
                }
            }

            s.call_on_name(ids::NEXT_PIECE, |n: &mut PieceView| {
                n.set_piece(next_piece);
            });

            s.call_on_name(ids::PADDED, |t: &mut PaddedView<DummyView>| {
                t.set_margins(margins);
            });

            s.call_on_name(ids::SCORE, |t: &mut TextView| {
                t.set_content(format!("{}", score));
            });

            s.call_on_name(ids::LEVEL, |t: &mut TextView| {
                t.set_content(format!("{}", level));
            });
            s.call_on_name(ids::LINES, |t: &mut TextView| {
                t.set_content(format!("{}", lines));
            });
            s.call_on_name(ids::HIGH_SCORE, |t: &mut TextView| {
                t.set_content(format!("{}", high_score));
            });
        })
    }
    fn handle_no_tick(&mut self) -> EventResult {
        let next_piece = self.next_piece;
        let scale = self.scale_mode;
        EventResult::with_cb(move |s| {
            s.call_on_name(ids::NEXT_PIECE, |n: &mut PieceView| {
                n.set_piece(next_piece);
                match scale {
                    ScaleMode::TooSmall | ScaleMode::Small => n.set_scale(false),
                    ScaleMode::Large => n.set_scale(true),
                }
            });
        })
    }
    fn try_current_piece_movement<F>(&mut self, mut f: F) -> bool
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
    fn try_piece_movement<F>(&self, piece: &mut Piece, mut f: F) -> bool
    where
        F: FnMut(&mut Piece),
    {
        let mut temp = piece.clone();
        // try movement by transforming temp
        f(&mut temp);

        if self.valid_piece(&temp) {
            *piece = temp;
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

        let small_x = BOARD_WIDTH * 2 * ScaleMode::Small.get_scale();
        let small_y = BOARD_HEIGHT * ScaleMode::Small.get_scale();

        let large_x = BOARD_WIDTH * 2 * ScaleMode::Large.get_scale();
        let large_y = BOARD_HEIGHT * ScaleMode::Large.get_scale();

        if small_x > constraint.pair().0 || small_y > constraint.pair().1 {
            self.scale_mode = ScaleMode::TooSmall;
        } else if large_x > constraint.pair().0 || large_y > constraint.pair().1 {
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
        // draw stateless ghost piece
        let mut ghost_piece = self.current_piece.clone();
        // shift ghost piece down all the way
        if self.ghost_piece_on {
            while self.try_piece_movement(&mut ghost_piece, &Piece::move_down) {}

            for i in 0..ghost_piece.layout().len() {
                for j in 0..ghost_piece.layout()[i].len() {
                    let tile = ghost_piece.layout()[i][j];
                    let row = ghost_piece.coord().1 + i as i8;
                    let col = ghost_piece.coord().0 + j as i8;
                    // don't attempt to print negatives
                    if row < 0 || col < 0 {
                        continue;
                    }
                    match tile {
                        // draw non-none tiles as gray for ghostly appearance
                        None => {}
                        _ => self.draw_tile(printer, Some(Block::Gray), row as usize, col as usize),
                    }
                }
            }
        }
        // draw piece AFTER board and ghost piece, simply "project" it onto everything, should
        // never be obstructed
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
        match self.scale_mode {
            ScaleMode::TooSmall => {
                let enlargement_notice_0 = "  Window Too Small  ";
                let enlargement_notice_1 = "Increase Window Size";
                printer.with_style(Color::Dark(BaseColor::Red), |p| {
                    p.print((0, 0), enlargement_notice_0);
                    p.print((0, 1), enlargement_notice_1);
                })
            }
            _ => {}
        }
    }
}

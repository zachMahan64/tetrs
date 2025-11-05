use crate::{
    board,
    text_art::BLOCK_CHAR,
    tile::{Block, Tile},
};
use cursive::{
    Printer, View,
    theme::{BaseColor, Color},
};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum PieceType {
    I,
    O,
    J,
    L,
    S,
    Z,
    T,
}

const LAYOUT_LEN: usize = 4;
pub type PieceLayout = [[Tile; LAYOUT_LEN]; LAYOUT_LEN];

pub enum Rotation {
    Left,
    Right,
}

impl PieceType {
    fn get_colored_block(&self) -> Block {
        match self {
            PieceType::I => Block::Cyan,
            PieceType::O => Block::Yellow,
            PieceType::J => Block::Blue,
            PieceType::L => Block::Orange,
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
    fn get_layout(&self) -> PieceLayout {
        type Lay = [[u8; LAYOUT_LEN]; LAYOUT_LEN];

        static I_LAYOUT: Lay = [
            [0, 0, 0, 0],
            [1, 1, 1, 1],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ];
        static O_LAYOUT: Lay = [
            [0, 0, 0, 0],
            [0, 1, 1, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0],
        ];
        static J_LAYOUT: Lay = [
            [1, 0, 0, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ];
        static L_LAYOUT: Lay = [
            [0, 0, 1, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ];
        static S_LAYOUT: Lay = [
            [0, 1, 1, 0],
            [1, 1, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ];
        static Z_LAYOUT: Lay = [
            [1, 1, 0, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ];
        static T_LAYOUT: Lay = [
            [0, 1, 0, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ];

        let bitmap_layout: Lay = match self {
            PieceType::I => I_LAYOUT,
            PieceType::O => O_LAYOUT,
            PieceType::J => J_LAYOUT,
            PieceType::L => L_LAYOUT,
            PieceType::S => S_LAYOUT,
            PieceType::Z => Z_LAYOUT,
            PieceType::T => T_LAYOUT,
        };

        self.to_tilemap(bitmap_layout)
    }
    fn to_tilemap(&self, bitmap: [[u8; LAYOUT_LEN]; LAYOUT_LEN]) -> PieceLayout {
        let mut layout: PieceLayout = [[None; LAYOUT_LEN]; LAYOUT_LEN];
        for i in 0..LAYOUT_LEN {
            for j in 0..LAYOUT_LEN {
                if bitmap[i][j] == 1 {
                    layout[i][j] = Some(self.get_colored_block());
                }
            }
        }
        layout
    }
}
#[derive(Clone, Copy)]
pub struct Piece {
    piece_type: PieceType,
    layout: PieceLayout,
    // of top left, signed so piece itself can go to edge even when top left of 4x4 layout is at
    // some 0 coord
    coord: (i8, i8),
}

impl Piece {
    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
    pub fn layout(&self) -> &PieceLayout {
        &self.layout
    }
    pub fn layout_mut(&mut self) -> &mut PieceLayout {
        &mut self.layout
    }
    pub fn coord(&self) -> (i8, i8) {
        self.coord
    }
    pub fn random_new() -> Self {
        let mut rng = rand::rng();
        let piece_type = match rng.random_range(0..=6) {
            0 => PieceType::I,
            1 => PieceType::O,
            2 => PieceType::J,
            3 => PieceType::L,
            4 => PieceType::S,
            5 => PieceType::Z,
            6 => PieceType::T,
            _ => PieceType::I, // shouldn't happen
        };
        let layout = piece_type.get_layout();
        Self {
            piece_type: piece_type,
            layout: layout,
            coord: (0, 0),
        }
    }
    pub fn new(piece_type: PieceType) -> Self {
        let layout = piece_type.get_layout();
        Self {
            piece_type: piece_type,
            layout: layout,
            coord: (0, 0),
        }
    }

    pub fn at(mut self, x: i8, y: i8) -> Self {
        self.coord = (x, y);
        self
    }
    pub fn set_at(&mut self, x: i8, y: i8) -> &Self {
        self.coord = (x, y);
        self
    }
    pub fn move_left(&mut self) {
        self.coord.0 -= 1
    }

    pub fn move_right(&mut self) {
        self.coord.0 += 1;
    }
    pub fn move_down(&mut self) {
        self.coord.1 += 1;
    }

    pub fn move_by(&mut self, x: i8, y: i8) {
        self.coord.0 += x;
        self.coord.1 += y;
    }
    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Left => {
                self.rotate_left();
            }
            Rotation::Right => {
                self.rotate_right();
            }
        }
    }
    pub fn rotate_left(&mut self) {
        let mut temp: PieceLayout = [[None; 4]; 4];
        match self.piece_type.get_rot_diameter() {
            4 => {
                temp[0][0] = self.layout[0][3];
                temp[0][1] = self.layout[1][3];
                temp[0][2] = self.layout[2][3];
                temp[0][3] = self.layout[3][3];

                temp[1][0] = self.layout[0][2];
                temp[1][1] = self.layout[1][2];
                temp[1][2] = self.layout[2][2];
                temp[1][3] = self.layout[3][2];

                temp[2][0] = self.layout[0][1];
                temp[2][1] = self.layout[1][1];
                temp[2][2] = self.layout[2][1];
                temp[2][3] = self.layout[3][1];

                temp[3][0] = self.layout[0][0];
                temp[3][1] = self.layout[1][0];
                temp[3][2] = self.layout[2][0];
                temp[3][3] = self.layout[3][0];
            }
            3 => {
                temp[0][0] = self.layout[0][2];
                temp[0][1] = self.layout[1][2];
                temp[0][2] = self.layout[2][2];

                temp[1][0] = self.layout[0][1];
                temp[1][1] = self.layout[1][1];
                temp[1][2] = self.layout[2][1];

                temp[2][0] = self.layout[0][0];
                temp[2][1] = self.layout[1][0];
                temp[2][2] = self.layout[2][0];
            }
            _ => {
                //impossible, but we'll know it fails because the piece will be empty
            }
        }
        self.layout = temp;
    }
    pub fn rotate_right(&mut self) {
        let mut temp: PieceLayout = [[None; 4]; 4];

        match self.piece_type.get_rot_diameter() {
            4 => {
                temp[0][0] = self.layout[3][0];
                temp[0][1] = self.layout[2][0];
                temp[0][2] = self.layout[1][0];
                temp[0][3] = self.layout[0][0];

                temp[1][0] = self.layout[3][1];
                temp[1][1] = self.layout[2][1];
                temp[1][2] = self.layout[1][1];
                temp[1][3] = self.layout[0][1];

                temp[2][0] = self.layout[3][2];
                temp[2][1] = self.layout[2][2];
                temp[2][2] = self.layout[1][2];
                temp[2][3] = self.layout[0][2];

                temp[3][0] = self.layout[3][3];
                temp[3][1] = self.layout[2][3];
                temp[3][2] = self.layout[1][3];
                temp[3][3] = self.layout[0][3];
            }
            3 => {
                temp[0][2] = self.layout[0][0];
                temp[1][2] = self.layout[0][1];
                temp[2][2] = self.layout[0][2];

                temp[0][1] = self.layout[1][0];
                temp[1][1] = self.layout[1][1];
                temp[2][1] = self.layout[1][2];

                temp[0][0] = self.layout[2][0];
                temp[1][0] = self.layout[2][1];
                temp[2][0] = self.layout[2][2];
            }
            _ => {
                //impossible, but we'll know it fails because the piece will be empty
            }
        }
        self.layout = temp;
    }
    // checks if piece is out of bounds for movement purposes, but pieces above the board are not
    // considered out of bounds
    pub fn is_out_of_bounds(&self) -> bool {
        for i in 0..self.layout.len() {
            for j in 0..self.layout[i].len() {
                let tile = self.layout[i][j];
                if tile.is_none() {
                    continue; // we do not care, no block
                }
                let x = j as i8 + self.coord.0;
                let y = i as i8 + self.coord.1;
                if x < 0 || x >= board::BOARD_WIDTH as i8 || y >= board::BOARD_HEIGHT as i8 {
                    return true;
                }
            }
        }
        false
    }
}

pub struct PieceView {
    piece: Option<Piece>,
    large: bool,
}

impl PieceView {
    pub fn new() -> Self {
        PieceView {
            piece: None,
            large: true,
        }
    }

    pub fn set_piece(&mut self, piece: Piece) {
        self.piece = Some(piece);
        // shift up O (square) so he can fit into 2x4
        if let Some(ref mut p) = self.piece {
            match p.piece_type() {
                PieceType::O => {
                    for i in 0..PIECEVIEW_HEIGHT - 1 {
                        p.layout_mut()[i] = p.layout_mut()[i + 1];
                    }
                }
                _ => {}
            }
        }
    }
    pub fn set_piece_optional(&mut self, opt_piece: Option<Piece>) {
        match opt_piece {
            Some(p) => self.set_piece(p),
            None => {}
        }
    }
    fn get_scale(&self) -> usize {
        match self.large {
            true => 2,
            false => 1,
        }
    }
    pub fn set_scale(&mut self, make_large: bool) {
        self.large = make_large;
    }
    fn draw_tile(&self, printer: &Printer, tile: Tile, row: usize, col: usize) {
        let i = self.get_scale() * row;
        // constant 2 to account for characters inheritantly being narrow
        let j = self.get_scale() * col * 2;
        const SMALL_SHIFT: usize = 2; // adjust for center alignment when small
        match self.large {
            false => {
                // 2 chars wide, 1 char tall
                for dx in 0..2 {
                    Self::draw_tile_char(printer, tile, (j + dx + SMALL_SHIFT, i));
                }
            }
            true => {
                for dx in 0..4 {
                    // 4 chars wide
                    for dy in 0..2 {
                        // 2 chars tall
                        Self::draw_tile_char(printer, tile, (j + dx, i + dy));
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
}

const PIECEVIEW_WIDTH: usize = 4;
const PIECEVIEW_HEIGHT: usize = 2;

impl View for PieceView {
    fn required_size(&mut self, _constraint: cursive::XY<usize>) -> cursive::XY<usize> {
        // scale should be externally managed by the owning view by passing in a bool for large as
        // true/false
        let dimen_x = PIECEVIEW_WIDTH * 2 * self.get_scale();
        let dimen_y = PIECEVIEW_HEIGHT * self.get_scale();
        (dimen_x, dimen_y).into()
    }
    fn draw(&self, printer: &Printer) {
        // rendering logic for static board
        for i in 0..PIECEVIEW_HEIGHT {
            for j in 0..PIECEVIEW_WIDTH {
                match self.piece {
                    None => {
                        self.draw_tile(printer, None, i, j);
                    }
                    Some(piece) => {
                        let tile = piece.layout()[i][j];
                        self.draw_tile(printer, tile, i, j);
                    }
                }
            }
        }
    }
}

const PIECE_BAG_SIZE: usize = 7;
pub struct PieceBag {
    pieces: [Piece; PIECE_BAG_SIZE],
    curr: usize,
}

impl PieceBag {
    pub fn new() -> Self {
        PieceBag {
            pieces: [(); PIECE_BAG_SIZE]
                .map(|_| Piece::random_new().at(board::PIECE_START_X, board::PIECE_START_Y)),
            curr: 0,
        }
    }
    #[inline]
    pub fn pop(&mut self) -> Piece {
        let piece = self.pieces[self.curr];
        self.pieces[self.curr] = Piece::random_new().at(board::PIECE_START_X, board::PIECE_START_Y);
        self.curr = (self.curr + 1) % PIECE_BAG_SIZE;
        piece
    }

    #[inline]
    pub fn get(&self, idx: usize) -> Piece {
        assert!(idx < PIECE_BAG_SIZE);
        let true_pos = (self.curr + idx) % PIECE_BAG_SIZE;
        self.pieces[true_pos]
    }
}

use crate::tile::{Block, Tile};
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
            [0, 0, 0, 0],
            [1, 1, 1, 1],
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
    pub fn at(&mut self, x: i8, y: i8) -> &mut Self {
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
}

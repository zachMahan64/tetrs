use crate::tile::{Block, Tile};

enum PieceType {
    I,
    O,
    J,
    L,
    S,
    Z,
    T,
}

const LAYOUT_LEN: usize = 4;

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
    fn get_layout(&self) -> [[Tile; LAYOUT_LEN]; LAYOUT_LEN] {
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
    fn to_tilemap(
        &self,
        bitmap: [[u8; LAYOUT_LEN]; LAYOUT_LEN],
    ) -> [[Tile; LAYOUT_LEN]; LAYOUT_LEN] {
        let mut layout: [[Tile; LAYOUT_LEN]; LAYOUT_LEN] = [[None; LAYOUT_LEN]; LAYOUT_LEN];
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

struct Piece {
    piece_type: PieceType,
    layout: [[Tile; LAYOUT_LEN]; LAYOUT_LEN],
    // of top left, signed so piece itself can go to edge even when top left of 4x4 layout is at
    // some 0 coord
    coord: (i8, i8),
}

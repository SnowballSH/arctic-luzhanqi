use serde::{Deserialize, Serialize};

pub const NUM_SQUARES: usize = 65;
pub type SquareIndex = u8;

// Helper functions for squares

const fn idx_to_rc(idx: SquareIndex) -> (i32, i32) {
    let i = idx as usize;
    ((i / 5) as i32, (i % 5) as i32)
}

const fn rc_to_idx(r: i32, c: i32) -> Option<SquareIndex> {
    if 0 <= r && r < 13 && 0 <= c && c < 5 {
        Some((r as usize * 5 + c as usize) as SquareIndex)
    } else {
        None
    }
}

const fn up(sq: SquareIndex) -> Option<SquareIndex> {
    let (r, c) = idx_to_rc(sq);
    rc_to_idx(r - 1, c)
}

const fn down(sq: SquareIndex) -> Option<SquareIndex> {
    let (r, c) = idx_to_rc(sq);
    rc_to_idx(r + 1, c)
}

const fn left(sq: SquareIndex) -> Option<SquareIndex> {
    let (r, c) = idx_to_rc(sq);
    rc_to_idx(r, c - 1)
}

const fn right(sq: SquareIndex) -> Option<SquareIndex> {
    let (r, c) = idx_to_rc(sq);
    rc_to_idx(r, c + 1)
}

const fn up_left(sq: SquareIndex) -> Option<SquareIndex> {
    let (r, c) = idx_to_rc(sq);
    rc_to_idx(r - 1, c - 1)
}

const fn up_right(sq: SquareIndex) -> Option<SquareIndex> {
    let (r, c) = idx_to_rc(sq);
    rc_to_idx(r - 1, c + 1)
}

const fn down_left(sq: SquareIndex) -> Option<SquareIndex> {
    let (r, c) = idx_to_rc(sq);
    rc_to_idx(r + 1, c - 1)
}

const fn down_right(sq: SquareIndex) -> Option<SquareIndex> {
    let (r, c) = idx_to_rc(sq);
    rc_to_idx(r + 1, c + 1)
}

/// Types of squares on the board.
/// Empty: no special properties (X)
/// Railroads: connected; regular pieces can move straight any number of squares;
///     engineers can move to any connected railroad (R)
/// Camp: pieces cannot be attacked in the camp, but can be moved to (C)
/// Headquarters: pieces can move in, but can never move out (H)
/// Frontline: acts like a railroad, but pieces cannot land on it (F)
/// Mountain: cannot be used (M)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SquareType {
    Empty,     // X
    Railroad,  // R
    Camp,      // C
    HQ,        // H
    Frontline, // F
    Mountain,  // M
}

/*
   1       2       3       4       5
m  X 00    H 01    X 02    H 03    X 04
l  R 05    R 06    R 07    R 08    R 09
k  R 10    C 11    X 12    C 13    R 14
j  R 15    X 16    C 17    X 18    R 19
i  R 20    C 21    X 22    C 23    R 24
h  R 25    R 26    R 27    R 28    R 29
g  F 30    M 31    F 32    M 33    F 34
f  R 35    R 36    R 37    R 38    R 39
e  R 40    C 41    X 42    C 43    R 44
d  R 45    X 46    C 47    X 48    R 49
c  R 50    C 51    X 52    C 53    R 54
b  R 55    R 56    R 57    R 58    R 59
a  X 60    H 61    X 62    H 63    X 64
   1       2       3       4       5
 */

/// Maps square indices to their types.
pub const SQUARE_TO_SQUARETYPE: [SquareType; NUM_SQUARES] = [
    // row m
    SquareType::Empty, //  0: m1 (X)
    SquareType::HQ,    //  1: m2 (H)
    SquareType::Empty, //  2: m3 (X)
    SquareType::HQ,    //  3: m4 (H)
    SquareType::Empty, //  4: m5 (X)
    // row l
    SquareType::Railroad, //  5: l1 (R)
    SquareType::Railroad, //  6: l2 (R)
    SquareType::Railroad, //  7: l3 (R)
    SquareType::Railroad, //  8: l4 (R)
    SquareType::Railroad, //  9: l5 (R)
    // row k
    SquareType::Railroad, // 10: k1 (R)
    SquareType::Camp,     // 11: k2 (C)
    SquareType::Empty,    // 12: k3 (X)
    SquareType::Camp,     // 13: k4 (C)
    SquareType::Railroad, // 14: k5 (R)
    // row j
    SquareType::Railroad, // 15: j1 (R)
    SquareType::Empty,    // 16: j2 (X)
    SquareType::Camp,     // 17: j3 (C)
    SquareType::Empty,    // 18: j4 (X)
    SquareType::Railroad, // 19: j5 (R)
    // row i
    SquareType::Railroad, // 20: i1 (R)
    SquareType::Camp,     // 21: i2 (C)
    SquareType::Empty,    // 22: i3 (X)
    SquareType::Camp,     // 23: i4 (C)
    SquareType::Railroad, // 24: i5 (R)
    // row h
    SquareType::Railroad, // 25: h1 (R)
    SquareType::Railroad, // 26: h2 (R)
    SquareType::Railroad, // 27: h3 (R)
    SquareType::Railroad, // 28: h4 (R)
    SquareType::Railroad, // 29: h5 (R)
    // row g
    SquareType::Frontline, // 30: g1 (F)
    SquareType::Mountain,  // 31: g2 (M)
    SquareType::Frontline, // 32: g3 (F)
    SquareType::Mountain,  // 33: g4 (M)
    SquareType::Frontline, // 34: g5 (F)
    // row f
    SquareType::Railroad, // 35: f1 (R)
    SquareType::Railroad, // 36: f2 (R)
    SquareType::Railroad, // 37: f3 (R)
    SquareType::Railroad, // 38: f4 (R)
    SquareType::Railroad, // 39: f5 (R)
    // row e
    SquareType::Railroad, // 40: e1 (R)
    SquareType::Camp,     // 41: e2 (C)
    SquareType::Empty,    // 42: e3 (X)
    SquareType::Camp,     // 43: e4 (C)
    SquareType::Railroad, // 44: e5 (R)
    // row d
    SquareType::Railroad, // 45: d1 (R)
    SquareType::Empty,    // 46: d2 (X)
    SquareType::Camp,     // 47: d3 (C)
    SquareType::Empty,    // 48: d4 (X)
    SquareType::Railroad, // 49: d5 (R)
    // row c
    SquareType::Railroad, // 50: c1 (R)
    SquareType::Camp,     // 51: c2 (C)
    SquareType::Empty,    // 52: c3 (X)
    SquareType::Camp,     // 53: c4 (C)
    SquareType::Railroad, // 54: c5 (R)
    // row b
    SquareType::Railroad, // 55: b1 (R)
    SquareType::Railroad, // 56: b2 (R)
    SquareType::Railroad, // 57: b3 (R)
    SquareType::Railroad, // 58: b4 (R)
    SquareType::Railroad, // 59: b5 (R)
    // row a
    SquareType::Empty, // 60: a1 (X)
    SquareType::HQ,    // 61: a2 (H)
    SquareType::Empty, // 62: a3 (X)
    SquareType::HQ,    // 63: a4 (H)
    SquareType::Empty, // 64: a5 (X)
];

pub const NUM_PIECETYPES: usize = 12;

/// Types of pieces in the game.
/// Overall > Army > Division > Brigade > Regiment > Battalion > Company > Platoon > Engineer
/// Bomb: destroys both itself and the piece it attacks. Cannot be on the front line.
/// Landmine: destroys the piece that attacks it, but not itself, except when attacked by engineer or bomb.
///     Can only be placed on the last two rows of each player. Cannot move.
/// Flag: Must be placed on a HQ. Cannot move. Capture the flag to win the game.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
    Overall,   // O
    Army,      // A
    Division,  // D
    Brigade,   // B
    Regiment,  // R
    Battalion, // T
    Company,   // C
    Platoon,   // P
    Engineer,  // E
    Bomb,      // X
    Landmine,  // L
    Flag,      // F
}

pub const LIST_OF_PIECETYPES: [PieceType; NUM_PIECETYPES] = [
    PieceType::Overall,
    PieceType::Army,
    PieceType::Division,
    PieceType::Brigade,
    PieceType::Regiment,
    PieceType::Battalion,
    PieceType::Company,
    PieceType::Platoon,
    PieceType::Engineer,
    PieceType::Bomb,
    PieceType::Landmine,
    PieceType::Flag,
];

impl PieceType {
    /// Returns the rank of the piece type.
    pub fn rank(&self) -> usize {
        match self {
            PieceType::Overall => 9,
            PieceType::Army => 8,
            PieceType::Division => 7,
            PieceType::Brigade => 6,
            PieceType::Regiment => 5,
            PieceType::Battalion => 4,
            PieceType::Company => 3,
            PieceType::Platoon => 2,
            PieceType::Engineer => 1,
            _ => 0, // Bomb, Landmine, Flag
        }
    }

    /// Returns the number of pieces of this type per player.
    pub fn num_per_player(&self) -> usize {
        match self {
            PieceType::Overall => 1,
            PieceType::Army => 1,
            PieceType::Division => 2,
            PieceType::Brigade => 2,
            PieceType::Regiment => 2,
            PieceType::Battalion => 2,
            PieceType::Company => 3,
            PieceType::Platoon => 3,
            PieceType::Engineer => 3,
            PieceType::Bomb => 2,
            PieceType::Landmine => 3,
            PieceType::Flag => 1,
        }
    }
}

/// Represents the color of a piece in the game.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Red,
    Black,
}

impl Color {
    /// Returns the next color in the turn order.
    pub fn other(&self) -> Self {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }
}

/// Represents a piece in the game, with its type and color.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub ty: PieceType,
    pub color: Color,
}

/// Represents the state of the game.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub turn: Color,
    pub board: Vec<Option<Piece>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            turn: Color::Red,
            board: vec![None; NUM_SQUARES],
        }
    }

    pub fn is_startpos_legal(&self) -> bool {
        if self.board.len() != NUM_SQUARES {
            return false;
        }

        // Check if count matches the number of pieces per player
        let mut counts = [[0; NUM_PIECETYPES]; 2];
        for piece in &self.board {
            if let Some(p) = piece {
                counts[p.color as usize][p.ty as usize] += 1;
            }
        }

        for color in [Color::Red, Color::Black].iter() {
            for (i, &piece_type) in LIST_OF_PIECETYPES.iter().enumerate() {
                if counts[*color as usize][i] != piece_type.num_per_player() {
                    return false;
                }
            }
        }

        // All pieces must be on X, R, or H, and nowhere else
        for (i, square_type) in SQUARE_TO_SQUARETYPE.iter().enumerate() {
            if let Some(piece) = &self.board[i] {
                match square_type {
                    SquareType::Empty | SquareType::Railroad | SquareType::HQ => continue,
                    _ => return false, // Invalid piece placement
                }
            }
        }

        // Each player's pieces are on their own side of the board
        for i in 0..NUM_SQUARES {
            if let Some(piece) = &self.board[i] {
                if piece.color == Color::Red && i < 35 {
                    // Red pieces must be on rows a-f
                    return false;
                }
                if piece.color == Color::Black && i >= 30 {
                    // Black pieces must be on rows h-m
                    return false;
                }
            }
        }

        // Bombs may not be on the first row of each player's side (Red: f, Black: h)
        for i in 35..40 {
            // Red's first row (f)
            if let Some(piece) = &self.board[i] {
                if piece.color == Color::Red && piece.ty == PieceType::Bomb {
                    return false;
                }
            }
        }

        for i in 25..30 {
            // Black's first row (h)
            if let Some(piece) = &self.board[i] {
                if piece.color == Color::Black && piece.ty == PieceType::Bomb {
                    return false;
                }
            }
        }

        // Landmines must not be on the last two rows of each player's side (Red: a,b, Black: l,m)
        for i in 55..65 {
            // Red's last two rows (a,b)
            if let Some(piece) = &self.board[i] {
                if piece.color == Color::Red && piece.ty == PieceType::Landmine {
                    return false;
                }
            }
        }

        for i in 0..10 {
            // Black's last two rows (l,m)
            if let Some(piece) = &self.board[i] {
                if piece.color == Color::Black && piece.ty == PieceType::Landmine {
                    return false;
                }
            }
        }

        // Flag must be in the player's HQ (Red: 61, 63, Black: 01, 03)
        if self.board[61].is_none_or(|p| !(p.color == Color::Red && p.ty == PieceType::Flag))
            && self.board[63].is_none_or(|p| !(p.color == Color::Red && p.ty == PieceType::Flag))
        {
            return false;
        }

        if self.board[1].is_none_or(|p| !(p.color == Color::Black && p.ty == PieceType::Flag))
            && self.board[3].is_none_or(|p| !(p.color == Color::Black && p.ty == PieceType::Flag))
        {
            return false;
        }

        true
    }
}

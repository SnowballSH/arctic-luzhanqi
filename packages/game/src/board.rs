use serde::{Deserialize, Serialize};
use crate::prng::PseudoRng;

pub const NUM_SQUARES: usize = 65;
pub type SquareIndex = u8;

/// Convenience constants for each square on the board. The indices follow the
/// diagram below where row `m` is at the top and row `a` at the bottom.
///
/// ```text
///    1       2       3       4       5
/// m  X 00    H 01    X 02    H 03    X 04
/// l  R 05    R 06    R 07    R 08    R 09
/// k  R 10    C 11    X 12    C 13    R 14
/// j  R 15    X 16    C 17    X 18    R 19
/// i  R 20    C 21    X 22    C 23    R 24
/// h  R 25    R 26    R 27    R 28    R 29
/// g  F 30    M 31    F 32    M 33    F 34
/// f  R 35    R 36    R 37    R 38    R 39
/// e  R 40    C 41    X 42    C 43    R 44
/// d  R 45    X 46    C 47    X 48    R 49
/// c  R 50    C 51    X 52    C 53    R 54
/// b  R 55    R 56    R 57    R 58    R 59
/// a  X 60    H 61    X 62    H 63    X 64
/// ```
pub mod sq {
    use super::SquareIndex;

    // Top to bottom rows m..a
    pub const M1: SquareIndex = 0;
    pub const M2: SquareIndex = 1;
    pub const M3: SquareIndex = 2;
    pub const M4: SquareIndex = 3;
    pub const M5: SquareIndex = 4;

    pub const L1: SquareIndex = 5;
    pub const L2: SquareIndex = 6;
    pub const L3: SquareIndex = 7;
    pub const L4: SquareIndex = 8;
    pub const L5: SquareIndex = 9;

    pub const K1: SquareIndex = 10;
    pub const K2: SquareIndex = 11;
    pub const K3: SquareIndex = 12;
    pub const K4: SquareIndex = 13;
    pub const K5: SquareIndex = 14;

    pub const J1: SquareIndex = 15;
    pub const J2: SquareIndex = 16;
    pub const J3: SquareIndex = 17;
    pub const J4: SquareIndex = 18;
    pub const J5: SquareIndex = 19;

    pub const I1: SquareIndex = 20;
    pub const I2: SquareIndex = 21;
    pub const I3: SquareIndex = 22;
    pub const I4: SquareIndex = 23;
    pub const I5: SquareIndex = 24;

    pub const H1: SquareIndex = 25;
    pub const H2: SquareIndex = 26;
    pub const H3: SquareIndex = 27;
    pub const H4: SquareIndex = 28;
    pub const H5: SquareIndex = 29;

    pub const G1: SquareIndex = 30;
    pub const G2: SquareIndex = 31;
    pub const G3: SquareIndex = 32;
    pub const G4: SquareIndex = 33;
    pub const G5: SquareIndex = 34;

    pub const F1: SquareIndex = 35;
    pub const F2: SquareIndex = 36;
    pub const F3: SquareIndex = 37;
    pub const F4: SquareIndex = 38;
    pub const F5: SquareIndex = 39;

    pub const E1: SquareIndex = 40;
    pub const E2: SquareIndex = 41;
    pub const E3: SquareIndex = 42;
    pub const E4: SquareIndex = 43;
    pub const E5: SquareIndex = 44;

    pub const D1: SquareIndex = 45;
    pub const D2: SquareIndex = 46;
    pub const D3: SquareIndex = 47;
    pub const D4: SquareIndex = 48;
    pub const D5: SquareIndex = 49;

    pub const C1: SquareIndex = 50;
    pub const C2: SquareIndex = 51;
    pub const C3: SquareIndex = 52;
    pub const C4: SquareIndex = 53;
    pub const C5: SquareIndex = 54;

    pub const B1: SquareIndex = 55;
    pub const B2: SquareIndex = 56;
    pub const B3: SquareIndex = 57;
    pub const B4: SquareIndex = 58;
    pub const B5: SquareIndex = 59;

    pub const A1: SquareIndex = 60;
    pub const A2: SquareIndex = 61;
    pub const A3: SquareIndex = 62;
    pub const A4: SquareIndex = 63;
    pub const A5: SquareIndex = 64;

    /// Squares considered the headquarters for each side
    pub const RED_HQ: [SquareIndex; 2] = [A2, A4];
    pub const BLACK_HQ: [SquareIndex; 2] = [M2, M4];

    /// Squares on the first row (closest to the frontline) of each side
    pub const RED_FIRST_ROW: [SquareIndex; 5] = [F1, F2, F3, F4, F5];
    pub const BLACK_FIRST_ROW: [SquareIndex; 5] = [H1, H2, H3, H4, H5];

    /// Squares on the back two rows of each side
    pub const RED_BACK_ROWS: [SquareIndex; 10] = [B1, B2, B3, B4, B5, A1, A2, A3, A4, A5];
    pub const BLACK_BACK_ROWS: [SquareIndex; 10] = [L1, L2, L3, L4, L5, M1, M2, M3, M4, M5];

    /// All playable squares for each side (excluding camps, mountains and frontline)
    pub const RED_SIDE: [SquareIndex; 25] = [
        F1, F2, F3, F4, F5,
        E1, E3, E5,
        D1, D2, D4, D5,
        C1, C3, C5,
        B1, B2, B3, B4, B5,
        A1, A2, A3, A4, A5,
    ];

    pub const BLACK_SIDE: [SquareIndex; 25] = [
        L1, L2, L3, L4, L5,
        K1, K3, K5,
        J1, J2, J4, J5,
        I1, I3, I5,
        H1, H2, H3, H4, H5,
        M1, M2, M3, M4, M5,
    ];
}

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

    /// Generate a random legal starting position using the provided seed.
    pub fn random_start(seed: u64) -> Self {
        let mut rng = PseudoRng::new(seed);
        let mut state = GameState::new();

        for &color in &[Color::Red, Color::Black] {
            let mut available: Vec<SquareIndex> = if color == Color::Red {
                sq::RED_SIDE.to_vec()
            } else {
                sq::BLACK_SIDE.to_vec()
            };

            // Flag placed on a HQ
            let hqs = if color == Color::Red { sq::RED_HQ } else { sq::BLACK_HQ };
            let flag_sq = hqs[rng.gen_range(0..hqs.len())];
            state.board[flag_sq as usize] = Some(Piece { ty: PieceType::Flag, color });
            available.retain(|&s| s != flag_sq);

            // Landmines placed on back rows
            let back_rows = if color == Color::Red { sq::RED_BACK_ROWS } else { sq::BLACK_BACK_ROWS };
            for _ in 0..PieceType::Landmine.num_per_player() {
                let choices: Vec<usize> = available
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| back_rows.contains(s))
                    .map(|(i, _)| i)
                    .collect();
                let idx = choices[rng.gen_range(0..choices.len())];
                let sq = available.remove(idx);
                state.board[sq as usize] = Some(Piece { ty: PieceType::Landmine, color });
            }

            // Bombs not allowed on the first row
            let first_row = if color == Color::Red { sq::RED_FIRST_ROW } else { sq::BLACK_FIRST_ROW };
            for _ in 0..PieceType::Bomb.num_per_player() {
                let choices: Vec<usize> = available
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| !first_row.contains(s))
                    .map(|(i, _)| i)
                    .collect();
                let idx = choices[rng.gen_range(0..choices.len())];
                let sq = available.remove(idx);
                state.board[sq as usize] = Some(Piece { ty: PieceType::Bomb, color });
            }

            // Remaining piece types
            let mut remaining: Vec<PieceType> = Vec::new();
            for ty in LIST_OF_PIECETYPES.iter() {
                if *ty == PieceType::Flag || *ty == PieceType::Bomb || *ty == PieceType::Landmine {
                    continue;
                }
                for _ in 0..ty.num_per_player() {
                    remaining.push(*ty);
                }
            }

            while !remaining.is_empty() {
                let sq_idx = rng.gen_range(0..available.len());
                let sq = available.remove(sq_idx);
                let ty_idx = rng.gen_range(0..remaining.len());
                let ty = remaining.remove(ty_idx);
                state.board[sq as usize] = Some(Piece { ty, color });
            }
        }

        state
    }

    pub fn is_startpos_legal(&self) -> bool {
        if self.board.len() != NUM_SQUARES {
            return false;
        }

        // piece counts per color
        let mut counts = [[0; NUM_PIECETYPES]; 2];
        for opt in &self.board {
            if let Some(p) = opt {
                counts[p.color as usize][p.ty as usize] += 1;
            }
        }
        for color in [Color::Red, Color::Black] {
            for (i, ty) in LIST_OF_PIECETYPES.iter().enumerate() {
                if counts[color as usize][i] != ty.num_per_player() {
                    return false;
                }
            }
        }

        for (idx, opt) in self.board.iter().enumerate() {
            let piece = match opt {
                Some(p) => p,
                None => continue,
            };
            let sq = idx as SquareIndex;

            // pieces only on Empty/Railroad/HQ
            match SQUARE_TO_SQUARETYPE[idx] {
                SquareType::Empty | SquareType::Railroad | SquareType::HQ => {}
                _ => return false,
            }

            let (side, hq, first_row, back_rows) = if piece.color == Color::Red {
                (&sq::RED_SIDE[..], &sq::RED_HQ[..], &sq::RED_FIRST_ROW[..], &sq::RED_BACK_ROWS[..])
            } else {
                (&sq::BLACK_SIDE[..], &sq::BLACK_HQ[..], &sq::BLACK_FIRST_ROW[..], &sq::BLACK_BACK_ROWS[..])
            };

            if !side.contains(&sq) {
                return false;
            }

            if piece.ty == PieceType::Bomb && first_row.contains(&sq) {
                return false;
            }

            if piece.ty == PieceType::Landmine && !back_rows.contains(&sq) {
                return false;
            }

            if piece.ty == PieceType::Flag && !hq.contains(&sq) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_start_is_legal() {
        let state = GameState::random_start(1234);
        assert!(state.is_startpos_legal());
    }

    #[test]
    fn test_illegal_position_detected() {
        let mut state = GameState::new();
        // Place a single red overall on an illegal square (frontline)
        state.board[sq::G1 as usize] = Some(Piece { ty: PieceType::Overall, color: Color::Red });
        assert!(!state.is_startpos_legal());
    }
}

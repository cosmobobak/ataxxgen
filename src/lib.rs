use std::{fmt::{self, Display, Formatter}, str::FromStr};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    white: u64,
    black: u64,
    walls: u64,
    ply: u8,
    halfmove: u8,
}

const RANK_8: u64 = 0xFF00_0000_0000_0000;
const FILE_H: u64 = 0x8080_8080_8080_8080;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    Single {
        to: Square,
    },
    Double {
        from: Square,
        to: Square,
    },
    Pass,
}

static SQUARE_NAMES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = SQUARE_NAMES.get(self.index()).copied();
        if let Some(name) = name {
            write!(f, "{name}")
        } else if self.0 == 64 {
            write!(f, "NO_SQUARE")
        } else {
            write!(f, "ILLEGAL: Square({})", self.0)
        }
    }
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = SQUARE_NAMES.get(self.index()).copied();
        if let Some(name) = name {
            write!(f, "{name}")
        } else if self.0 == 64 {
            write!(f, "NO_SQUARE")
        } else {
            write!(f, "ILLEGAL: Square({})", self.0)
        }
    }
}

impl FromStr for Square {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SQUARE_NAMES
            .iter()
            .position(|&name| name == s)
            .and_then(|index| -> Option<u8> { index.try_into().ok() })
            .map(Self::new)
            .ok_or("Invalid square name")
    }
}


impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "0000" {
            return Ok(Move::Pass);
        }

        match s.len() {
            2 => {
                if let Ok(sq) = Square::from_str(&s[0..2]) {
                    Ok(Self::Single { to: sq })
                } else {
                    Err("Invalid square name")
                }
            }
            4 => {
                if let Ok(from) = Square::from_str(&s[0..2]) {
                    if let Ok(to) = Square::from_str(&s[2..4]) {
                        Ok(Self::Double { from, to })
                    } else {
                        Err("invalid to-square name")
                    }
                } else {
                    Err("invalid from-square name")
                }
            }
            _ => Err("invalid move length"),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Move::Pass => write!(f, "0000"),
            Move::Single { to } => write!(f, "{}", to),
            Move::Double { from, to } => write!(f, "{}{}", from, to),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn to_char(self) -> char {
        match self {
            Player::White => 'x',
            Player::Black => 'o',
        }
    }

}

impl Default for Board {
    fn default() -> Board {
        Board::new()
    }
}

const BB_ALL: u64 = !(RANK_8 | FILE_H);

const fn shift_up(bb: u64) -> u64 {
    (bb << 8) & BB_ALL
}
const fn shift_down(bb: u64) -> u64 {
    (bb >> 8) & BB_ALL
}
const fn shift_left(bb: u64) -> u64 {
    (bb << 1) & BB_ALL
}
const fn shift_right(bb: u64) -> u64 {
    (bb >> 1) & BB_ALL
}
const fn expand(bb: u64) -> u64 {
    let vertical = shift_up(bb) | shift_down(bb) | bb;
    (vertical | shift_left(vertical) | shift_right(vertical)) & BB_ALL
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Square(u8);

impl Square {
    pub const A1: Self = Self(0);
    pub const B1: Self = Self(1);
    pub const C1: Self = Self(2);
    pub const D1: Self = Self(3);
    pub const E1: Self = Self(4);
    pub const F1: Self = Self(5);
    pub const G1: Self = Self(6);
    pub const H1: Self = Self(7);
    pub const A2: Self = Self(8);
    pub const B2: Self = Self(9);
    pub const C2: Self = Self(10);
    pub const D2: Self = Self(11);
    pub const E2: Self = Self(12);
    pub const F2: Self = Self(13);
    pub const G2: Self = Self(14);
    pub const H2: Self = Self(15);
    pub const A3: Self = Self(16);
    pub const B3: Self = Self(17);
    pub const C3: Self = Self(18);
    pub const D3: Self = Self(19);
    pub const E3: Self = Self(20);
    pub const F3: Self = Self(21);
    pub const G3: Self = Self(22);
    pub const H3: Self = Self(23);
    pub const A4: Self = Self(24);
    pub const B4: Self = Self(25);
    pub const C4: Self = Self(26);
    pub const D4: Self = Self(27);
    pub const E4: Self = Self(28);
    pub const F4: Self = Self(29);
    pub const G4: Self = Self(30);
    pub const H4: Self = Self(31);
    pub const A5: Self = Self(32);
    pub const B5: Self = Self(33);
    pub const C5: Self = Self(34);
    pub const D5: Self = Self(35);
    pub const E5: Self = Self(36);
    pub const F5: Self = Self(37);
    pub const G5: Self = Self(38);
    pub const H5: Self = Self(39);
    pub const A6: Self = Self(40);
    pub const B6: Self = Self(41);
    pub const C6: Self = Self(42);
    pub const D6: Self = Self(43);
    pub const E6: Self = Self(44);
    pub const F6: Self = Self(45);
    pub const G6: Self = Self(46);
    pub const H6: Self = Self(47);
    pub const A7: Self = Self(48);
    pub const B7: Self = Self(49);
    pub const C7: Self = Self(50);
    pub const D7: Self = Self(51);
    pub const E7: Self = Self(52);
    pub const F7: Self = Self(53);
    pub const G7: Self = Self(54);
    pub const H7: Self = Self(55);
    pub const A8: Self = Self(56);
    pub const B8: Self = Self(57);
    pub const C8: Self = Self(58);
    pub const D8: Self = Self(59);
    pub const E8: Self = Self(60);
    pub const F8: Self = Self(61);
    pub const G8: Self = Self(62);
    pub const H8: Self = Self(63);
    pub const NO_SQUARE: Self = Self(64);

    pub const fn from_rank_file(rank: u8, file: u8) -> Self {
        let inner = rank * 8 + file;
        debug_assert!(inner <= 64);
        Self(inner)
    }

    pub const fn new(inner: u8) -> Self {
        debug_assert!(inner <= 64);
        Self(inner)
    }

    pub const fn flip_rank(self) -> Self {
        Self(self.0 ^ 0b111_000)
    }

    pub const fn flip_file(self) -> Self {
        Self(self.0 ^ 0b000_111)
    }

    pub const fn relative_to(self, side: Player) -> Self {
        if matches!(side, Player::White) {
            self
        } else {
            self.flip_rank()
        }
    }

    /// The file that this square is on.
    pub const fn file(self) -> u8 {
        self.0 % 8
    }
    /// The rank that this square is on.
    pub const fn rank(self) -> u8 {
        self.0 / 8
    }

    pub fn distance(a: Self, b: Self) -> u8 {
        std::cmp::max(a.file().abs_diff(b.file()), a.rank().abs_diff(b.rank()))
    }

    pub const fn signed_inner(self) -> i8 {
        #![allow(clippy::cast_possible_wrap)]
        self.0 as i8
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }

    pub const fn inner(self) -> u8 {
        self.0
    }

    pub const fn add(self, offset: u8) -> Self {
        #![allow(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss
        )]
        let res = self.0 + offset;
        debug_assert!(res < 64, "Square::add overflowed");
        Self(res)
    }

    pub const fn add_beyond_board(self, offset: u8) -> Self {
        #![allow(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss
        )]
        let res = self.0 + offset;
        debug_assert!(res < 65, "Square::add_beyond_board overflowed");
        Self(res)
    }

    pub const fn sub(self, offset: u8) -> Self {
        #![allow(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss
        )]
        let res = self.0 - offset;
        debug_assert!(res < 64, "Square::sub overflowed");
        Self(res)
    }

    pub const fn on_board(self) -> bool {
        self.0 < 64
    }

    pub const fn as_set(self) -> u64 {
        1 << self.0
    }

    pub fn pawn_push(self, side: Player) -> Self {
        if side == Player::White {
            self.add(8)
        } else {
            self.sub(8)
        }
    }

    pub fn pawn_right(self, side: Player) -> Self {
        if side == Player::White {
            self.add(9)
        } else {
            self.sub(7)
        }
    }

    pub fn pawn_left(self, side: Player) -> Self {
        if side == Player::White {
            self.add(7)
        } else {
            self.sub(9)
        }
    }

    #[rustfmt::skip]
    pub const fn le(self, other: Self) -> bool { self.0 <= other.0 }
    #[rustfmt::skip]
    pub const fn ge(self, other: Self) -> bool { self.0 >= other.0 }
    #[rustfmt::skip]
    pub const fn lt(self, other: Self) -> bool { self.0 < other.0  }
    #[rustfmt::skip]
    pub const fn gt(self, other: Self) -> bool { self.0 > other.0  }

    pub fn all() -> impl Iterator<Item = Self> {
        (0..64).map(Self::new)
    }

    pub fn name(self) -> Option<&'static str> {
        SQUARE_NAMES.get(self.index()).copied()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            white: 1 << Square::A7.0 | 1 << Square::G1.0,
            black: 1 << Square::A1.0 | 1 << Square::G7.0,
            walls: RANK_8 | FILE_H,
            ply: 0,
            halfmove: 0,
        }
    }

    pub fn turn(&self) -> Player {
        if self.ply % 2 == 0 {
            Player::White
        } else {
            Player::Black
        }
    }

    pub fn make_move(&mut self, mv: Move) {
        match mv {
            Move::Pass => {}
            Move::Single { to } => {
                self.halfmove = 0;
                let to = to.as_set();
                let flip_zone = expand(to);
                if self.turn() == Player::White {
                    self.white ^= to;
                    let wiped_out = flip_zone & self.black;
                    self.black ^= wiped_out;
                    self.white |= wiped_out;
                } else {
                    self.black ^= to;
                    let wiped_out = flip_zone & self.white;
                    self.white ^= wiped_out;
                    self.black |= wiped_out;
                }
            }
            Move::Double { from, to } => {
                self.halfmove += 1;
                let from = from.as_set();
                let to = to.as_set();
                let flip_zone = expand(to);
                if self.turn() == Player::White {
                    self.white ^= from | to;
                    let wiped_out = flip_zone & self.black;
                    self.black ^= wiped_out;
                    self.white |= wiped_out;
                } else {
                    self.black ^= from | to;
                    let wiped_out = flip_zone & self.white;
                    self.white ^= wiped_out;
                    self.black |= wiped_out;
                }
            }
        }
        self.ply += 1;
    }

    pub fn generate_moves(&self, mut listener: impl FnMut(Move) -> bool) {
        if self.game_over() {
            return;
        }

        let (us, them) = match self.turn() {
            Player::White => (self.white, self.black),
            Player::Black => (self.black, self.white),
        };

        let empty = !(us | them | self.walls);

        let mut singles = expand(us) & empty;
        let mut any_generated = singles != 0;

        while singles != 0 {
            let to = Square::new(singles.trailing_zeros() as u8);
            singles &= singles - 1;
            if listener(Move::Single { to }) {
                return;
            }
        }

        let mut doubles_src = us;
        while doubles_src != 0 {
            let from = Square::new(doubles_src.trailing_zeros() as u8);
            doubles_src &= doubles_src - 1;
            let local_singles = expand(from.as_set());
            let mut doubles_tgt = expand(local_singles) & empty & !local_singles;
            any_generated |= doubles_tgt != 0;
            while doubles_tgt != 0 {
                let to = Square::new(doubles_tgt.trailing_zeros() as u8);
                doubles_tgt &= doubles_tgt - 1;
                if listener(Move::Double { from, to }) {
                    return;
                }
            }
        }

        if !any_generated {
            listener(Move::Pass);
        }
    }

    pub fn game_over(&self) -> bool {
        self.white == 0
            || self.black == 0
            || (self.white | self.black | self.walls) & BB_ALL == BB_ALL
            || self.halfmove >= 100
            || expand(expand(self.white | self.black)) & !((self.white | self.black) | self.walls) & BB_ALL == 0
    }

    pub fn player_at(&self, sq: Square) -> Option<Player> {
        if self.white & sq.as_set() != 0 {
            Some(Player::White)
        } else if self.black & sq.as_set() != 0 {
            Some(Player::Black)
        } else {
            None
        }
    }

    pub fn wall_at(&self, sq: Square) -> bool {
        self.walls & sq.as_set() != 0
    }

    pub fn fen(&self) -> String {
        let mut fen = String::new();

        for rank in (0u8..7).rev() {
            let mut file: u8 = 0;

            while file < 7 {
                let sq = Square::from_rank_file(rank, file);

                match self.player_at(sq) {
                    Some(p) => fen.push(p.to_char()),
                    None => {
                        if self.wall_at(sq) {
                            fen.push('-');
                        } else {
                            let mut empty_squares: u32 = 1;

                            while file < 6
                                && self.player_at(Square::from_rank_file(rank, file + 1)).is_none()
                            {
                                file += 1;
                                empty_squares += 1;
                            }

                            fen += empty_squares.to_string().as_str();
                        }
                    }
                }

                file += 1;
            }

            if rank > 0 {
                fen.push('/');
            }
        }

        fen + format!(
            " {} {} {}",
            self.turn().to_char(),
            self.halfmove,
            self.ply / 2 + 1
        )
        .as_str()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rank in (0u8..7).rev() {
            writeln!(f, " +---+---+---+---+---+---+---+")?;

            for file in 0u8..7 {
                let sq = Square::from_rank_file(rank, file);
                write!(
                    f,
                    " | {}",
                    if self.wall_at(sq) {
                        '-'
                    } else {
                        self.player_at(sq).map_or(' ', |p| p.to_char())
                    }
                )?;
            }

            writeln!(f, " | {}", rank + 1)?;
        }

        writeln!(f, " +---+---+---+---+---+---+---+")?;
        writeln!(f, "   a   b   c   d   e   f   g")?;
        writeln!(f)?;

        write!(
            f,
            "{} to move",
            if self.turn() == Player::White {
                "White"
            } else {
                "Black"
            }
        )
    }
}

pub fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    if depth == 1 {
        let mut count = 0;
        board.generate_moves(|_| {
            count += 1;
            false
        });
        return count;
    }

    let mut count = 0;
    board.generate_moves(|mv| {
        let mut board = *board;
        board.make_move(mv);
        count += perft(&board, depth - 1);
        false
    });

    count
}

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
        to: u8,
    },
    Double {
        from: u8,
        to: u8,
    },
    Pass,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
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
                let to = 1 << to;
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
                let from = 1 << from;
                let to = 1 << to;
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
            let to = singles.trailing_zeros() as u8;
            singles &= singles - 1;
            if listener(Move::Single { to }) {
                return;
            }
        }

        let mut doubles_src = us;
        while doubles_src != 0 {
            let from = doubles_src.trailing_zeros() as u8;
            doubles_src &= doubles_src - 1;
            let local_singles = expand(1 << from);
            let mut doubles_tgt = expand(local_singles) & empty & !local_singles;
            any_generated |= doubles_tgt != 0;
            while doubles_tgt != 0 {
                let to = doubles_tgt.trailing_zeros() as u8;
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
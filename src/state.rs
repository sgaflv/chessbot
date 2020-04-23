
use crate::bboard::*;
use crate::common::add_bit;
use crate::debug::Demo;
use std::fmt::Formatter;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub static PIECES: [Piece; 6] = [Piece::King, Piece::Pawn, Piece::Rook, Piece::Knight,Piece::Bishop, Piece::Queen];

impl Piece {
    pub fn all_values() -> &'static [Piece; 6] {
        &PIECES
    }

    pub fn idx(&self) -> usize {
        (*self) as usize
    }

    pub fn value(&self) -> i32 {
        match self {
            Piece::Pawn => 100,
            Piece::Rook => 500,
            Piece::Knight => 320,
            Piece::Bishop => 330,
            Piece::Queen => 900,
            Piece::King => 40000,
        }
    }


    pub fn to_string(&self, side: Side) -> String {
        match side {
            Side::White => {
                match self {
                    Piece::Pawn=> "Pawn".to_string(),
                    Piece::Rook => "Rook".to_string(),
                    Piece::Knight=> "Knight".to_string(),
                    Piece::Bishop=> "Bishop".to_string(),
                    Piece::Queen => "Queen".to_string(),
                    Piece::King => "King".to_string(),
                }
            }
            Side::Black => {
                match self {
                    Piece::Pawn=> "pawn".to_string(),
                    Piece::Rook => "rook".to_string(),
                    Piece::Knight=> "knight".to_string(),
                    Piece::Bishop=> "bishop".to_string(),
                    Piece::Queen => "queen".to_string(),
                    Piece::King => "king".to_string(),
                }
            }
        }
    }
    pub fn to_char(&self, side: Side) -> char {
        match side {
            Side::White => {
                match self {
                    Piece::Pawn=> 'P',
                    Piece::Rook => 'R',
                    Piece::Knight=> 'N',
                    Piece::Bishop=> 'B',
                    Piece::Queen => 'Q',
                    Piece::King => 'K',
                }
            }
            Side::Black => {
                match self {
                    Piece::Pawn => 'p',
                    Piece::Rook => 'r',
                    Piece::Knight => 'n',
                    Piece::Bishop => 'b',
                    Piece::Queen => 'q',
                    Piece::King => 'k',
                }
            }
        }
    }

    pub fn from_byte(c: &u8) -> (Piece, Side) {
        match *c {
            b'P' => (Piece::Pawn, Side::White),
            b'R' => (Piece::Rook, Side::White),
            b'N' => (Piece::Knight, Side::White),
            b'B' => (Piece::Bishop, Side::White),
            b'Q' => (Piece::Queen, Side::White),
            b'K' => (Piece::King, Side::White),
            b'p' => (Piece::Pawn, Side::Black),
            b'r' => (Piece::Rook, Side::Black),
            b'n' => (Piece::Knight, Side::Black),
            b'b' => (Piece::Bishop, Side::Black),
            b'q' => (Piece::Queen, Side::Black),
            b'k' => (Piece::King, Side::Black),
            _ => panic!()
        }
    }
}

static INITIAL_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub fn idx(&self) -> i32 {
        *self as i32
    }

    pub fn opposite(&self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }

    pub fn from_byte(c: u8) -> Side {
        match c {
            b'w' => Side::White,
            b'b' => Side::Black,
            _ => panic!("Unknown side character {}", c as char)
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Side::White => 'w',
            Side::Black => 'b',
        }
    }
}

use std::fmt;


impl fmt::Display for Side {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.to_char())
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct SideState {
    pub king_side_castle: bool,
    pub queen_side_castle: bool,

    pub boards: [BBoard; 6],

    pub all: BBoard,
}

impl SideState {
    pub fn new() -> SideState {
        SideState {
            king_side_castle: false,
            queen_side_castle: false,
            boards: [0u64; 6],

            all: 0u64,
        }
    }

    pub fn update(&mut self) {
        self.all = 0u64;
        for b in self.boards.iter() {
            self.all |= *b;
        }
    }

    pub fn castle_state(&self) -> (bool, bool) {
        (self.king_side_castle, self.queen_side_castle)
    }

    pub fn set_castle_state(&mut self, state: (bool, bool)) {
        self.king_side_castle = state.0;
        self.queen_side_castle = state.1;
    }

    pub fn set_king_side_castle(&mut self, value: bool) {
        self.king_side_castle = value;
    }

    pub fn set_queen_side_castle(&mut self, value: bool) {
        self.queen_side_castle = value;
    }

    pub fn get_mut_board(&mut self, piece: Piece) -> &mut BBoard {
        &mut self.boards[piece.idx()]
    }

    pub fn get_board(&self, piece: Piece) -> BBoard {
        self.boards[piece.idx()]
    }

    pub fn add_bit(&mut self, piece: Piece, to_add: BBoard) {
        let board = self.get_mut_board(piece);
        *board |= to_add;
        self.all |= to_add;
    }

    pub fn remove_bit(&mut self, to_remove: BBoard) -> Option<Piece> {
        let inv = !to_remove;

        if self.all & to_remove == 0 {
            return None;
        }

        self.all &= inv;

        if self.get_board(Piece::Pawn) & to_remove > 0 {
            *self.get_mut_board(Piece::Pawn) &= inv;
            return Some(Piece::Pawn);
        }

        if self.get_board(Piece::Rook) & to_remove > 0 {
            *self.get_mut_board(Piece::Rook) &= inv;
            return Some(Piece::Rook);
        }

        if self.get_board(Piece::Knight) & to_remove > 0 {
            *self.get_mut_board(Piece::Knight) &= inv;
            return Some(Piece::Knight);
        }

        if self.get_board(Piece::Bishop) & to_remove > 0 {
            *self.get_mut_board(Piece::Bishop) &= inv;
            return Some(Piece::Bishop);
        }

        if self.get_board(Piece::Queen) & to_remove > 0 {
            *self.get_mut_board(Piece::Queen) &= inv;
            return Some(Piece::Queen);
        }

        if self.get_board(Piece::King) & to_remove > 0 {
            *self.get_mut_board(Piece::King) &= inv;
            return Some(Piece::King);
        }

        panic!();
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct GameState {
    pub next_to_move: Side,
    pub en_passant: BBoard, // en-passant field from the last move

    pub white_state: SideState,
    pub black_state: SideState,

}

impl GameState {
    pub fn new_empty() -> GameState {
        GameState {
            next_to_move: Side::White,
            en_passant: 0,
            white_state: SideState::new(),
            black_state: SideState::new(),
        }
    }

    pub fn new_game() -> GameState {
        GameState::from_fen(INITIAL_BOARD)
    }

    pub fn get_mut_side_state(&mut self, side: Side) -> &mut SideState {
        match side {
            Side::White => &mut self.white_state,
            Side::Black => &mut self.black_state,
        }
    }

    pub fn get_mut_sides_state(&mut self, side: Side) -> (&mut SideState, &mut SideState) {
        match side {
            Side::White => (&mut self.white_state, &mut self.black_state),
            Side::Black => (&mut self.black_state, &mut self.white_state),
        }
    }


    pub fn get_side_state(&self, side: Side) -> &SideState {
        match side {
            Side::White => &self.white_state,
            Side::Black => &self.black_state,
        }
    }

    pub fn get_mut_board(&mut self, coord: &(Piece, Side)) -> &mut BBoard {
        match coord.1 {
            Side::White => {
                self.white_state.get_mut_board(coord.0)
            },
            Side::Black => {
                self.black_state.get_mut_board(coord.0)
            }
        }
    }

    pub fn get_board(&self, coord: &(Piece, Side)) -> BBoard {
        match coord.1 {
            Side::White => {
                self.white_state.get_board(coord.0)
            },
            Side::Black => {
                self.black_state.get_board(coord.0)
            }
        }
    }

    pub fn castle_state(&self, side: Side) -> (bool, bool) {
        let side_state = self.get_side_state(side);
        (side_state.king_side_castle, side_state.queen_side_castle)
    }

    pub fn set_castle_state(&mut self, side: Side, state: (bool, bool)) {
        let side_state = self.get_mut_side_state(side);
        side_state.set_castle_state(state);
    }

    pub fn to_fen(&self) -> String {

        let mut board = ['.'; 64];

        for piece in Piece::all_values().iter() {
            for side in [Side::White, Side::Black].iter() {

                let coord = (*piece, *side);
                let bit_board = self.get_board(&coord);

                let c = piece.to_char(*side);

                for i in 0..64 {
                    if (bit_board >> i) & 1 == 1 {
                        board[i] = c;
                    }
                }
            }
        }

        let mut result = String::new();
        let mut count = 0;
        for y in 0..8 {
            for x in 0..8 {
                let idx = (7 - y) * 8 + x;

                if board[idx] == '.' {
                    count += 1;
                    continue;
                } else if count > 0  {
                    result.push((count + b'0') as char);
                    count = 0;
                }

                result.push(board[idx]);
            }

            if count > 0  {
                result.push((count + b'0') as char);
            }
            count = 0;

            if y < 7 {
                result.push('/')
            }
        }

        result.push_str(format!(" {} {} {}", self.next_to_move, self.castle_string(), self.en_passant_string()).as_str());

        result
    }

    pub fn castle_string(&self) -> String {
        // get castle string
        let mut castle = String::from("");

        let wc = self.castle_state(Side::White);
        let bc = self.castle_state(Side::Black);

        if wc.0 {
            castle.push('K');
        }
        if wc.1 {
            castle.push('Q');
        }
        if bc.0 {
            castle.push('k');
        }
        if bc.1 {
            castle.push('q');
        }

        castle
    }

    pub fn en_passant_string(&self) -> String {
        bb_to_coord(self.en_passant)
    }

    pub fn from_fen(fen: &str) -> GameState {

        let mut state = GameState::new_empty();

        let split: Vec<&str> = fen.trim().split(" ").collect();

        if let Some(board) = split.get(0) {

            // parse board pieces
            let mut idx = 0u32;
            for c in board.as_bytes() {

                if idx > 63 {
                    panic!();
                }

                let x = idx % 8;
                let y = 7 - idx / 8;

                if *c == b'/' {
                    // just ignore this symbol for now
                    continue;
                } else if *c >= b'1' && *c <= b'8' {
                    idx += (*c - b'0') as u32;
                    continue;
                } else if b"PpRrNnBbQqKk".contains(c) {
                    let board: &mut BBoard = state.get_mut_board(&Piece::from_byte(&c));

                    add_bit(board, x, y);
                } else {
                    panic!("unrecognized FEN board input: {}", board);
                }

                idx = idx + 1;
            }
        }

        if let Some(side) = split.get(1) {
            state.next_to_move = Side::from_byte(side.as_bytes()[0]);
        }

        if let Some(castles) = split.get(2) {
            let (mut bk, mut wk, mut bq, mut wq) = (false, false, false, false);

            for c in castles.as_bytes() {
                match *c {
                    b'K' => wk = true,
                    b'k' => bk = true,
                    b'Q' => wq = true,
                    b'q' => bq = true,

                    _ => {}
                }
            }

            state.get_mut_side_state(Side::White).set_castle_state((wk,wq));
            state.get_mut_side_state(Side::Black).set_castle_state((bk,bq));

        }

        if let Some(en_passant) = split.get(3) {
            if en_passant.len() == 2 {
                let bytes = en_passant.as_bytes();
                let x = bytes[0];
                let y = bytes[1];

                let idx = (y - b'1') as u64 * 8 + (x - b'a') as u64;
                state.en_passant = 1u64 << idx;
            }
        }

        state.black_state.update();
        state.white_state.update();

        state
    }
}

impl Demo for GameState {

    fn demo(&self) {
        println!("{}", self.to_fen());

        let mut result = ['.'; 64];

        for piece in Piece::all_values().iter() {
            for side in [Side::White, Side::Black].iter() {

                let coord = (*piece, *side);
                let board = self.get_board(&coord);

                let c = piece.to_char(*side);

                for i in 0..64 {
                    if (board >> i) & 1 == 1 {
                        if result[i] != '.' {
                            println!("board conflict between {} and {} at index {}", result[i], c, i);
                        }
                        result[i] = c;
                    }
                }
            }
        }

        // print board
        for y in 0..8 {
            for x in 0..8 {
                let idx = (7 - y) * 8 + x;
                print!("{}", result[idx]);
            }

            println!();
        }

        println!("{} {} {}\n", self.next_to_move, self.castle_string(), self.en_passant_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_states() {
        let state1 = GameState::new_game();
        let state2 = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq");
        let state3 = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qkq - 0 1");
        let state4 = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
        let state5 = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 1");
        let state6 = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq a3 0 1");

        assert_eq!(state1, state2);
        assert_ne!(state2, state3);
        assert_ne!(state1, state3);
        assert_ne!(state1, state4);
        assert_ne!(state1, state5);
        assert_ne!(state1, state6);
    }

    #[test]
    fn test_enum_index() {
        let a = Piece::Queen;
        println!("{}", a.idx());
    }
}
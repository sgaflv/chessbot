pub mod bboard;
pub mod state;
pub mod debug;
pub mod evaluator;
pub mod common;
pub mod magic;
pub mod move_generator;
pub mod messaging;
pub mod piece_moves;

use bboard::*;
use debug::*;
use evaluator::*;
use state::*;

fn main() {

    let a: BBoard = bb_coord(0,0) | bb_coord(1, 1) | bb_coord(2, 2);
    a.demo();

    show_scores();

    let state = GameState::new_game();

    state.demo();
}

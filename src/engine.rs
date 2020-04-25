use crate::game_setup::ChessMove;
use crate::move_generator::MoveGenerator;
use crate::state::ChessState;
use rand::Rng;

pub struct ChessEngine {
    move_generator: MoveGenerator,
}

impl ChessEngine {

    pub fn new() -> ChessEngine {
        ChessEngine {
            move_generator: MoveGenerator::new()
        }
    }

    pub fn find_best_move(&self, state: &ChessState) -> Option<ChessMove> {
        let mut moves: Vec<ChessState> = Vec::new();

        self.move_generator.generate_moves(state, &mut moves);

        if moves.len() == 0 {
            // checkmate situation
            return None;
        }

        let mut rng = rand::thread_rng();

        let rnd: usize = rng.gen::<usize>() % moves.len();

        let random_state = moves.get(rnd).unwrap();

        let next_move = state.get_move(random_state);

        Some(next_move)
    }
}
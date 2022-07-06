use rand::{Rng, thread_rng};

use shared::message::PublicLeaderBoard;

#[derive(Debug, Clone)]
pub struct RandomTargetStrategy {
    pub(crate) current_name: String}

#[derive(Debug, Clone)]
pub struct TopTargetStrategy {
    pub(crate) current_name: String}

#[derive(Debug, Clone)]
pub struct BottomTargetStrategy {
    pub(crate) current_name: String}

#[derive(Debug, Clone)]
pub enum TargetStrategyType {
    RandomTargetStrategy(RandomTargetStrategy),
    TopTargetStrategy(TopTargetStrategy),
    BottomTargetStrategy(BottomTargetStrategy),
}

pub trait TargetStrategy {
    fn new(current_name: String) -> Self;
    fn next_target(self, public_leader_board: PublicLeaderBoard) -> String;
}

impl TargetStrategy for TopTargetStrategy {
    fn new(current_name: String) -> Self {
        TopTargetStrategy {current_name}
    }

    fn next_target(self, mut public_leader_board: PublicLeaderBoard) -> String {
        public_leader_board.sort_by(|a, b| b.score.cmp(&a.score));
        for player in public_leader_board {
            if player.name != self.current_name && player.is_active {
                return player.name.clone();
            }
        }
        panic!("No more players in the game");
    }
}

impl TargetStrategy for BottomTargetStrategy {
    fn new(current_name: String) -> Self {
        BottomTargetStrategy {current_name}
    }

    fn next_target(self, mut public_leader_board: PublicLeaderBoard) -> String {
        public_leader_board.sort_by(|a, b| a.score.cmp(&b.score));
        for player in public_leader_board {
            if player.name != self.current_name && player.is_active {
                return player.name.clone();
            }
        }
        panic!("No more players in the game");
    }
}

impl TargetStrategy for RandomTargetStrategy {
    fn new(current_name: String) -> Self {
        RandomTargetStrategy {current_name}
    }

    fn next_target(self, public_leader_board: PublicLeaderBoard) -> String {
        let public_leader_board = public_leader_board.iter()
        .filter(|player| player.name != self.current_name && player.is_active)
        .collect::<Vec<_>>();
        let mut rng = thread_rng();
        let target_index = rng.gen_range(0..public_leader_board.len());
        public_leader_board.get(target_index as usize).unwrap().name.to_string()
    }
}

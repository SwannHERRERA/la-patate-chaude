use rand::seq::IteratorRandom;

use shared::message::PublicLeaderBoard;

#[derive(Debug, Clone)]
pub struct RandomTargetStrategy {
    pub(crate) current_name: String,
}

#[derive(Debug, Clone)]
pub struct TopTargetStrategy {
    pub(crate) current_name: String,
}

#[derive(Debug, Clone)]
pub struct BottomTargetStrategy {
    pub(crate) current_name: String,
}

#[derive(Debug, Clone)]
pub enum TargetStrategyType {
    RandomTargetStrategy(RandomTargetStrategy),
    TopTargetStrategy(TopTargetStrategy),
    BottomTargetStrategy(BottomTargetStrategy),
}

pub trait TargetStrategy {
    fn new(current_name: String) -> Self;
    fn next_target(self, public_leader_board: &PublicLeaderBoard) -> String;
}

impl TargetStrategy for TopTargetStrategy {
    fn new(current_name: String) -> Self {
        TopTargetStrategy { current_name }
    }

    fn next_target(self, public_leader_board: &PublicLeaderBoard) -> String {
        return public_leader_board
            .iter()
            .filter(|player| player.name != self.current_name && player.is_active)
            .max_by(|a, b| a.score.cmp(&b.score))
            .expect("No more players in the game")
            .name
            .clone();
    }
}

impl TargetStrategy for BottomTargetStrategy {
    fn new(current_name: String) -> Self {
        BottomTargetStrategy { current_name }
    }

    fn next_target(self, public_leader_board: &PublicLeaderBoard) -> String {
        return public_leader_board
            .iter()
            .filter(|player| player.name != self.current_name && player.is_active)
            .min_by(|a, b| a.score.cmp(&b.score))
            .expect("No more players in the game")
            .name
            .clone();
    }
}

impl TargetStrategy for RandomTargetStrategy {
    fn new(current_name: String) -> Self {
        RandomTargetStrategy { current_name }
    }

    fn next_target(self, public_leader_board: &PublicLeaderBoard) -> String {
        let mut rng = rand::thread_rng();
        return public_leader_board
            .iter()
            .filter(|player| player.name != self.current_name && player.is_active)
            .choose(&mut rng)
            .expect("No more players in the game")
            .name
            .clone();
    }
}

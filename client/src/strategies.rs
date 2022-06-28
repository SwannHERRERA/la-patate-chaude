use shared::message::Message::PublicLeaderBoard;
use shared::message::PublicLeaderBoard;
use crate::strategies::TargetStrategyType::TopTargetStrategy;

enum TargetStrategyType { RandomTargetStrategy(PublicLeaderBoard), TopTargetStrategy(PublicLeaderBoard), BottomTargetStrategy(PublicLeaderBoard) }

trait TargetStrategy {
    fn new(public_leader_board: PublicLeaderBoard) -> Self;
    /// Résout le challenge
    fn next_target(&self) -> String;
}

impl TargetStrategy for TopTargetStrategy {
    fn new(public_leader_board: PublicLeaderBoard) -> Self {
        TopTargetStrategy(public_leader_board)
    }

    fn next_target(&self) -> String {
        public_leader_board.sort_by(|a, b| b.score.cmp(&a.score));
        for player in public_leader_board {
            if player.name != username && player.is_active {
                return player.name.clone();
            }
        }
        "".to_string()
    }
}
use serde::{Deserialize, Serialize};

use crate::{
    public_player::PublicPlayer,
    challenge::{ChallengeAnswer, ReportedChallengeResult, ChallengeType}, subscribe::SubscribeResult,
};

pub type PublicLeaderBoard = Vec<PublicPlayer>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Hello,
    Welcome { version: u8 },
    Subscribe { name: String },
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(ChallengeType),
    ChallengeResult {
        answer: ChallengeAnswer,
        next_target: String,
    },
    RoundSummary {
        challenge: String,
        chain: Vec<ReportedChallengeResult>,
    },
    StartGame {},
    EndOfGame {
        leader_board: Vec<PublicPlayer>,
    },
    EndOfCommunication,
}

#[derive(Debug, Clone)]
pub enum ResponseType {
    Broadcast,
    Unicast,
}

#[derive(Debug, Clone)]
pub struct MessageType {
    pub message: Message,
    pub message_type: ResponseType,
}

impl MessageType {
    pub fn boardcast(message: Message) -> MessageType {
        MessageType { message, message_type: ResponseType::Broadcast }
    }
    pub fn unicast(message: Message) -> MessageType {
        MessageType { message, message_type: ResponseType::Unicast }
    }
}


#[cfg(test)]
mod tests {
    use crate::subscribe::SubscribeError;

    use super::*;

    #[test]
    fn test_message_hello_serialization() {
        let message = Message::Hello;
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(serialized, "\"Hello\"");
    }

    #[test]
    fn test_welcome_serialization() {
        let message = Message::Welcome { version: 1 };
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(serialized, "{\"Welcome\":{\"version\":1}}");
    }

    #[test]
    fn test_subscribe_serialization() {
        let message = Message::Subscribe { name: "test".to_string() };
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(serialized, "{\"Subscribe\":{\"name\":\"test\"}}");
    }

    #[test]
    fn test_subscribe_result_success_serialization() {
        let message = Message::SubscribeResult(SubscribeResult::Ok);
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(serialized, "{\"SubscribeResult\":\"Ok\"}");
    }

    #[test]
    fn test_subscribe_result_failure_serialization() {
        let message = Message::SubscribeResult(SubscribeResult::Err(SubscribeError::InvalidName));
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(serialized, "{\"SubscribeResult\":{\"Err\":\"InvalidName\"}}");
    }
}
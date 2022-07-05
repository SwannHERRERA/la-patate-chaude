use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonstrousMazeOutput {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonstrousMazeMap {
    pub map: Vec<String>,
    pub map_height: usize,
    pub map_width: usize,
    pub player_position: Position,
    pub target_position: Position,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

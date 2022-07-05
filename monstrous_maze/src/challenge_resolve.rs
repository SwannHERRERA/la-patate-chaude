use crate::models::{MonstrousMazeInput, MonstrousMazeMap, MonstrousMazeOutput, Position};
use log::debug;

pub struct MonstrousMaze;

static PLAYER_TOKEN: char = 'Y';
static EXIT_TOKEN: char = 'X';
static FREE_TOKEN: char = ' ';

impl MonstrousMaze {
    pub fn resolve_monstrous_maze_challenge(
        monstrous_maze_input: MonstrousMazeInput,
    ) -> MonstrousMazeOutput {
        let endurance_left = monstrous_maze_input.endurance;
        let map = get_monstrous_maze_map_from_input(monstrous_maze_input);

        MonstrousMazeOutput {
            path: "".to_string(),
        }
    }
}

/*
- Find path in maze represented by a vec<String>
- Start from player position and must go to target position.
- Any character except ' ' is a wall and cannot be crossed
- Must return string with '>', '<', '^', 'v' chars, representing moves
- Use Dijkstra algorithm
*/
fn find

fn get_monstrous_maze_map_from_input(monstrous_maze_input: MonstrousMazeInput) -> MonstrousMazeMap {
    let map: Vec<String> = monstrous_maze_input
        .grid
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    assert!(map.len() > 0);

    let map_height = map.len();
    let map_width = map[0].len();
    let player_position = find_player_position_from_vec_map(&map);
    let target_position = find_exit_position_from_vec_map(&map);
    debug!(
        "Map height : {}, Map width : {}, player : {:?}, target : {:?}",
        map_height, map_width, player_position, target_position
    );

    MonstrousMazeMap {
        map,
        map_height,
        map_width,
        player_position,
        target_position,
    }
}

fn find_player_position_from_vec_map(map: &Vec<String>) -> Position {
    let player_position = find_char_position_in_vec_map(&PLAYER_TOKEN, map);
    if player_position.is_some() {
        return player_position.unwrap();
    }
    panic!("Player not found");
}

fn find_exit_position_from_vec_map(map: &Vec<String>) -> Position {
    let exit_position = find_char_position_in_vec_map(&EXIT_TOKEN, map);
    if exit_position.is_some() {
        return exit_position.unwrap();
    }
    panic!("Exit not found");
}

fn find_char_position_in_vec_map(char: &char, map: &Vec<String>) -> Option<Position> {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == *char {
                return Some(Position { x, y });
            }
        }
    }
    None
}

use std::collections::HashSet;

use log::debug;

use crate::models::{MonstrousMazeInput, MonstrousMazeMap, MonstrousMazeOutput, Position};

pub struct MonstrousMaze;

static PLAYER_TOKEN: char = 'Y';
static EXIT_TOKEN: char = 'X';
// static FREE_TOKEN: char = ' ';
static WALL_TOKENS: [&'static char; 11] = [
    &'┐', &'┴', &'┌', &'┤', &'└', &'├', &'┬', &'│', &'┘', &'─', &'┼',
];
static MONSTER_TOKEN: char = 'M';

impl MonstrousMaze {
    pub fn resolve_monstrous_maze_challenge(
        monstrous_maze_input: MonstrousMazeInput,
    ) -> MonstrousMazeOutput {
        let endurance_left = monstrous_maze_input.endurance;
        let map = get_monstrous_maze_map_from_input(monstrous_maze_input);

        let path = find_path(&map, &endurance_left, &String::new(), &HashSet::new());

        if path.is_some() {
            return MonstrousMazeOutput {
                path: path.unwrap(),
            };
        }
        panic!("No path found");
    }
}

fn find_path(
    map: &MonstrousMazeMap,
    endurance_left: &u8,
    current_path: &String,
    already_visited: &HashSet<Position>,
) -> Option<String> {
    if map.player_position == map.target_position {
        return Some(current_path.clone());
    }

    let mut new_endurance_left = endurance_left.clone();

    if is_player_on_monster_position(map) {
        new_endurance_left -= 1;
        debug!(
            "Player is on monster position, endurance left: {}",
            new_endurance_left
        );
    }

    if new_endurance_left == 0 {
        debug!("Endurance is 0, wrong path");
        return None;
    }

    let mut new_already_visited = already_visited.clone();
    new_already_visited.insert(map.player_position.clone());

    if can_go_up(map, already_visited) {
        let mut new_map = map.clone();
        let mut new_path = current_path.clone();
        new_map.player_position.y -= 1;
        new_path.push_str("^");

        let result = find_path(&new_map, endurance_left, &new_path, &new_already_visited);
        if result.is_some() {
            return result;
        }
    }

    if can_go_down(map, already_visited) {
        let mut new_map = map.clone();
        let mut new_path = current_path.clone();
        new_map.player_position.y += 1;
        new_path.push_str("v");

        let result = find_path(&new_map, endurance_left, &new_path, &new_already_visited);
        if result.is_some() {
            return result;
        }
    }

    if can_go_left(map, already_visited) {
        let mut new_map = map.clone();
        let mut new_path = current_path.clone();
        new_map.player_position.x -= 1;
        new_path.push_str("<");

        let result = find_path(&new_map, endurance_left, &new_path, &new_already_visited);
        if result.is_some() {
            return result;
        }
    }

    if can_go_right(map, already_visited) {
        let mut new_map = map.clone();
        let mut new_path = current_path.clone();
        new_map.player_position.x += 1;
        new_path.push_str(">");

        let result = find_path(&new_map, endurance_left, &new_path, &new_already_visited);
        if result.is_some() {
            return result;
        }
    }

    None
}

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

fn can_go_up(map: &MonstrousMazeMap, already_visited: &HashSet<Position>) -> bool {
    let player_position: &Position = &map.player_position;
    if player_position.y == 0
        || is_wall(
            &map.map[player_position.y - 1]
                .chars()
                .nth(player_position.x)
                .expect("Player position is out of bounds"),
        )
        || already_visited.contains(&Position {
            y: player_position.y - 1,
            x: player_position.x,
        })
    {
        return false;
    }

    true
}

fn can_go_down(map: &MonstrousMazeMap, already_visited: &HashSet<Position>) -> bool {
    let player_position: &Position = &map.player_position;
    if player_position.y == map.map_height - 1
        || is_wall(
            &map.map[player_position.y + 1]
                .chars()
                .nth(player_position.x)
                .expect("Player position is out of bounds"),
        )
        || already_visited.contains(&Position {
            y: player_position.y + 1,
            x: player_position.x,
        })
    {
        return false;
    }

    true
}

fn can_go_right(map: &MonstrousMazeMap, already_visited: &HashSet<Position>) -> bool {
    let player_position: &Position = &map.player_position;
    if player_position.x == map.map_width - 1
        || is_wall(
            &map.map[player_position.y]
                .chars()
                .nth(player_position.x + 1)
                .expect("Player position is out of bounds"),
        )
        || already_visited.contains(&Position {
            y: player_position.y,
            x: player_position.x + 1,
        })
    {
        return false;
    }

    true
}

fn can_go_left(map: &MonstrousMazeMap, already_visited: &HashSet<Position>) -> bool {
    let player_position: &Position = &map.player_position;
    if player_position.x == 0
        || is_wall(
            &map.map[player_position.y]
                .chars()
                .nth(player_position.x - 1)
                .expect("Player position is out of bounds"),
        )
        || already_visited.contains(&Position {
            y: player_position.y,
            x: player_position.x - 1,
        })
    {
        return false;
    }

    true
}

fn is_wall(char: &char) -> bool {
    WALL_TOKENS.contains(&char)
}

fn is_player_on_monster_position(map: &MonstrousMazeMap) -> bool {
    return map.map[map.player_position.y]
        .chars()
        .nth(map.player_position.x)
        .expect("Player position is out of bounds")
        == MONSTER_TOKEN;
}

// test module
#[cfg(test)]
mod tests {}

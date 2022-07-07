use std::collections::HashSet;

use log::debug;

use crate::models::{MonstrousMazeInput, MonstrousMazeMap, MonstrousMazeOutput, Position};

pub struct MonstrousMazeResolver;

static PLAYER_TOKEN: char = 'I';
static EXIT_TOKEN: char = 'X';
// static FREE_TOKEN: char = ' ';
static WALL_TOKEN: char = '#';
static MONSTER_TOKEN: char = 'M';

impl MonstrousMazeResolver {
    pub fn resolve_monstrous_maze_challenge(
        monstrous_maze_input: &MonstrousMazeInput,
    ) -> MonstrousMazeOutput {
        debug!("{:?}", monstrous_maze_input.grid);
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

    if let Some(path) = process_up_path(
        map,
        &new_endurance_left,
        &current_path,
        &new_already_visited,
    ) {
        return Some(path);
    }

    if let Some(path) = process_down_path(
        map,
        &new_endurance_left,
        &current_path,
        &new_already_visited,
    ) {
        return Some(path);
    }

    if let Some(path) = process_left_path(
        map,
        &new_endurance_left,
        &current_path,
        &new_already_visited,
    ) {
        return Some(path);
    }

    if let Some(path) = process_right_path(
        map,
        &new_endurance_left,
        &current_path,
        &new_already_visited,
    ) {
        return Some(path);
    }

    None
}

fn process_right_path(
    map: &MonstrousMazeMap,
    endurance_left: &u8,
    current_path: &String,
    already_visited: &HashSet<Position>,
) -> Option<String> {
    if can_go_right(map, already_visited) {
        let mut new_map = map.clone();
        let mut new_path = current_path.clone();
        new_map.player_position.x += 1;
        new_path.push_str(">");

        return find_path(&new_map, endurance_left, &new_path, &already_visited);
    }

    None
}

fn process_left_path(
    map: &MonstrousMazeMap,
    endurance_left: &u8,
    current_path: &String,
    already_visited: &HashSet<Position>,
) -> Option<String> {
    if can_go_left(map, already_visited) {
        let mut new_map = map.clone();
        let mut new_path = current_path.clone();
        new_map.player_position.x -= 1;
        new_path.push_str("<");

        return find_path(&new_map, endurance_left, &new_path, &already_visited);
    }

    None
}

fn process_up_path(
    map: &MonstrousMazeMap,
    endurance_left: &u8,
    current_path: &String,
    already_visited: &HashSet<Position>,
) -> Option<String> {
    if can_go_up(map, already_visited) {
        let mut new_map = map.clone();
        let mut new_path = current_path.clone();
        new_map.player_position.y -= 1;
        new_path.push_str("^");

        return find_path(&new_map, endurance_left, &new_path, &already_visited);
    }

    None
}

fn process_down_path(
    map: &MonstrousMazeMap,
    endurance_left: &u8,
    current_path: &String,
    already_visited: &HashSet<Position>,
) -> Option<String> {
    if can_go_down(map, already_visited) {
        let mut new_map = map.clone();
        let mut new_path = current_path.clone();
        new_map.player_position.y += 1;
        new_path.push_str("v");

        return find_path(&new_map, endurance_left, &new_path, &already_visited);
    }

    None
}

fn get_monstrous_maze_map_from_input(
    monstrous_maze_input: &MonstrousMazeInput,
) -> MonstrousMazeMap {
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
    *char == WALL_TOKEN
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
mod tests {
    use std::collections::HashSet;

    use utils::file_utils::read_file;

    use crate::challenge_resolve::{
        can_go_down, can_go_left, can_go_right, can_go_up, find_char_position_in_vec_map,
        find_exit_position_from_vec_map, find_player_position_from_vec_map,
        get_monstrous_maze_map_from_input, is_player_on_monster_position, MonstrousMazeResolver,
    };
    use crate::models::{MonstrousMazeInput, MonstrousMazeMap, Position};

    #[test]
    fn test_mazes_resolution() {
        let mazes: Vec<String> = read_file("data/mazes.txt")
            .split('\n')
            .map(|maze| maze.replace("\\n", "\n"))
            .collect();

        let answers: Vec<String> = read_file("data/mazes_answers.txt")
            .split('\n')
            .map(|answer| answer.to_string())
            .collect();

        mazes.iter().enumerate().for_each(|(index, maze)| {
            let answer =
                MonstrousMazeResolver::resolve_monstrous_maze_challenge(&MonstrousMazeInput {
                    endurance: 2,
                    grid: maze.to_string(),
                });
            assert_eq!(answer.path, answers[index]);
        })
    }

    #[test]
    fn test_is_player_on_monster_position() {
        let mut monstrous_maze_map = MonstrousMazeMap {
            map: vec![
                "# ###".to_string(),
                "#   #".to_string(),
                "# M #".to_string(),
                "#   #".to_string(),
                "#####".to_string(),
            ],
            map_height: 5,
            map_width: 5,
            player_position: Position { x: 1, y: 0 },
            target_position: Position { x: 0, y: 0 },
        };
        assert_eq!(is_player_on_monster_position(&monstrous_maze_map), false);

        monstrous_maze_map.player_position = Position { x: 2, y: 2 };
        assert_eq!(is_player_on_monster_position(&monstrous_maze_map), true);
    }

    #[test]
    fn test_can_go_left() {
        let mut monstrous_maze_map = MonstrousMazeMap {
            map: vec![
                "# ###".to_string(),
                "#   #".to_string(),
                "# M #".to_string(),
                "#   #".to_string(),
                "#####".to_string(),
            ],
            map_height: 5,
            map_width: 5,
            player_position: Position { x: 1, y: 0 },
            target_position: Position { x: 0, y: 0 },
        };
        let mut already_visited: HashSet<Position> = HashSet::new();
        assert_eq!(can_go_left(&monstrous_maze_map, &already_visited), false);

        monstrous_maze_map.player_position = Position { x: 2, y: 1 };
        assert_eq!(can_go_left(&monstrous_maze_map, &already_visited), true);

        already_visited.insert(Position { x: 1, y: 1 });
        assert_eq!(can_go_left(&monstrous_maze_map, &already_visited), false);
    }

    #[test]
    fn test_can_go_up() {
        let mut monstrous_maze_map = MonstrousMazeMap {
            map: vec![
                "# ###".to_string(),
                "#   #".to_string(),
                "# M #".to_string(),
                "#   #".to_string(),
                "#####".to_string(),
            ],
            map_height: 5,
            map_width: 5,
            player_position: Position { x: 1, y: 0 },
            target_position: Position { x: 0, y: 0 },
        };
        let mut already_visited: HashSet<Position> = HashSet::new();
        assert_eq!(can_go_up(&monstrous_maze_map, &already_visited), false);

        monstrous_maze_map.player_position = Position { x: 1, y: 2 };
        assert_eq!(can_go_up(&monstrous_maze_map, &already_visited), true);

        already_visited.insert(Position { x: 1, y: 1 });
        assert_eq!(can_go_up(&monstrous_maze_map, &already_visited), false);
    }

    #[test]
    fn test_can_go_right() {
        let mut monstrous_maze_map = MonstrousMazeMap {
            map: vec![
                "# ###".to_string(),
                "#   #".to_string(),
                "# M #".to_string(),
                "#   #".to_string(),
                "#####".to_string(),
            ],
            map_height: 5,
            map_width: 5,
            player_position: Position { x: 1, y: 0 },
            target_position: Position { x: 0, y: 0 },
        };
        let mut already_visited: HashSet<Position> = HashSet::new();
        assert_eq!(can_go_right(&monstrous_maze_map, &already_visited), false);

        monstrous_maze_map.player_position = Position { x: 2, y: 1 };
        assert_eq!(can_go_right(&monstrous_maze_map, &already_visited), true);

        already_visited.insert(Position { x: 3, y: 1 });
        assert_eq!(can_go_right(&monstrous_maze_map, &already_visited), false);
    }

    #[test]
    fn test_can_go_down() {
        let mut monstrous_maze_map = MonstrousMazeMap {
            map: vec![
                "# ###".to_string(),
                "#   #".to_string(),
                "# M #".to_string(),
                "#   #".to_string(),
                "#####".to_string(),
            ],
            map_height: 5,
            map_width: 5,
            player_position: Position { x: 1, y: 3 },
            target_position: Position { x: 0, y: 0 },
        };
        let mut already_visited: HashSet<Position> = HashSet::new();
        assert_eq!(can_go_down(&monstrous_maze_map, &already_visited), false);

        monstrous_maze_map.player_position = Position { x: 2, y: 1 };
        assert_eq!(can_go_down(&monstrous_maze_map, &already_visited), true);

        already_visited.insert(Position { x: 2, y: 2 });
        assert_eq!(can_go_down(&monstrous_maze_map, &already_visited), false);
    }

    #[test]
    fn test_find_char_position_in_vec_map() {
        let map = vec![
            "# ###".to_string(),
            "#   #".to_string(),
            "# M #".to_string(),
            "#   #".to_string(),
            "#####".to_string(),
        ];

        let result = find_char_position_in_vec_map(&'M', &map);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), Position { x: 2, y: 2 });

        let result = find_char_position_in_vec_map(&'I', &map);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_exit_position_from_vec_map() {
        let map = vec![
            "# ###".to_string(),
            "#   #".to_string(),
            "# M #".to_string(),
            "#   #".to_string(),
            "###X#".to_string(),
        ];

        assert_eq!(
            find_exit_position_from_vec_map(&map),
            Position { x: 3, y: 4 }
        );
    }

    #[test]
    #[should_panic]
    fn test_find_exit_position_from_vec_map_panic() {
        let map = vec![
            "# ###".to_string(),
            "#   #".to_string(),
            "# M #".to_string(),
            "#   #".to_string(),
            "#####".to_string(),
        ];

        find_exit_position_from_vec_map(&map);
    }

    #[test]
    fn test_find_player_position_from_vec_map() {
        let map = vec![
            "#I###".to_string(),
            "#   #".to_string(),
            "# M #".to_string(),
            "#   #".to_string(),
            "#####".to_string(),
        ];

        assert_eq!(
            find_player_position_from_vec_map(&map),
            Position { x: 1, y: 0 }
        );
    }

    #[test]
    #[should_panic]
    fn test_find_player_position_from_vec_map_panic() {
        let map = vec![
            "# ###".to_string(),
            "#   #".to_string(),
            "# M #".to_string(),
            "#   #".to_string(),
            "#####".to_string(),
        ];

        find_player_position_from_vec_map(&map);
    }

    #[test]
    fn test_get_monstrous_maze_map_from_input() {
        let monstrous_maze_input = MonstrousMazeInput {
            endurance: 2,
            grid: "#I###\n#   #\n# M #\n#   #\n###X#".to_string(),
        };
        let expected_monstrous_maze_map = MonstrousMazeMap {
            map: vec![
                "#I###".to_string(),
                "#   #".to_string(),
                "# M #".to_string(),
                "#   #".to_string(),
                "###X#".to_string(),
            ],
            map_height: 5,
            map_width: 5,
            player_position: Position { x: 1, y: 0 },
            target_position: Position { x: 3, y: 4 },
        };

        assert_eq!(
            get_monstrous_maze_map_from_input(&monstrous_maze_input),
            expected_monstrous_maze_map
        );
    }
}

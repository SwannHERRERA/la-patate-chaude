use std::collections::HashSet;

use log::error;
use rand::seq::SliceRandom;
use rand::Rng;

use utils::file_utils::read_mazes_file_macro;

use crate::challenge_resolve::{can_go_down, can_go_left, can_go_right, can_go_up, get_monstrous_maze_map_from_input, is_player_on_monster_position};
use crate::models::{MonstrousMazeInput, MonstrousMazeOutput};

pub fn generate_monstrous_maze_challenge() -> MonstrousMazeInput {
    let mut rng = rand::thread_rng();
    let mazes: Vec<String> = read_mazes_file_macro()
        .split('\n')
        .map(|maze| maze.replace("\\n", "\n"))
        .collect();

    let maze = mazes.choose(&mut rng).expect("Unable to choose maze");
    MonstrousMazeInput {
        grid: maze.clone(),
        endurance: rng.gen_range(2..=4),
    }
}

pub fn validate_maze_challenge(
    challenge_input: &MonstrousMazeInput,
    challenge_output: &MonstrousMazeOutput,
) -> bool {
    let mut maze_map = get_monstrous_maze_map_from_input(challenge_input);
    let mut endurance_left = challenge_input.endurance;
    let empty_hashset = HashSet::new();
    for direction in challenge_output.path.chars() {
        match direction {
            '^' => {
                if can_go_up(&maze_map, &empty_hashset) {
                    maze_map.player_position.y -= 1;
                } else {
                    error!("Unable to go up");
                    return false;
                }
            }
            'v' => {
                if can_go_down(&maze_map, &empty_hashset) {
                    maze_map.player_position.y += 1;
                } else {
                    error!("Unable to go down");
                    return false;
                }
            }
            '<' => {
                if can_go_left(&maze_map, &empty_hashset) {
                    maze_map.player_position.x -= 1;
                } else {
                    error!("Unable to go left");
                    return false;
                }
            }
            '>' => {
                if can_go_right(&maze_map, &empty_hashset) {
                    maze_map.player_position.x += 1;
                } else {
                    error!("Unable to go right");
                    return false;
                }
            }
            _ => {
                error!("Invalid direction");
                return false;
            }
        }
        if is_player_on_monster_position(&maze_map) {
            endurance_left -= 1;
        }

        if endurance_left <= 0 {
            error!("Endurance is 0");
            return false;
        }
    }
    maze_map.player_position == maze_map.target_position
}

#[cfg(test)]
mod tests {
    use utils::file_utils::read_file;
    use crate::challenge_generator::{generate_monstrous_maze_challenge, validate_maze_challenge};
    use crate::challenge_resolve::{get_monstrous_maze_map_from_input};
    use crate::models::{MonstrousMazeInput, MonstrousMazeOutput};

    #[test]
    fn test_generate_monstrous_maze_challenge() {
        for _ in 0..10 {
            let challenge_input = generate_monstrous_maze_challenge();
            assert!(challenge_input.grid.len() > 0);
            assert!(challenge_input.endurance > 0);
            get_monstrous_maze_map_from_input(&challenge_input);
        }
    }

    #[test]
    fn test_validate_maze_challenge() {
        let mazes: Vec<String> = read_file("data/mazes.txt")
            .split('\n')
            .map(|maze| maze.replace("\\n", "\n"))
            .collect();

        let answers: Vec<String> = read_file("data/mazes_answers.txt")
            .split('\n')
            .map(|answer| answer.to_string())
            .collect();

        mazes.iter().enumerate().for_each(|(index, maze)| {
            let input = MonstrousMazeInput {
                endurance: 2,
                grid: maze.to_string(),
            };
            let output = MonstrousMazeOutput{
                path: answers.get(index).expect("Unable to get answer").to_string(),
            };
            assert_eq!(validate_maze_challenge(&input, &output), true);
        })
    }

    #[test]
    fn test_validate_maze_challenge_fail() {
        let input = MonstrousMazeInput {
            endurance: 2,
            grid: "#I###\n#   #\n# M #\n#   #\n###X#".to_string(),
        };
        let output = MonstrousMazeOutput {
            path: "^>v<".to_string(),
        };

        assert_eq!(validate_maze_challenge(&input, &output), false);

        let output = MonstrousMazeOutput {
            path: "vvvvv".to_string(),
        };

        assert_eq!(validate_maze_challenge(&input, &output), false);

        let output = MonstrousMazeOutput {
            path: "^>v<^>v<".to_string(),
        };

        assert_eq!(validate_maze_challenge(&input, &output), false);

        let output = MonstrousMazeOutput {
            path: "v>v<^>v<^>v<".to_string(),
        };

        assert_eq!(validate_maze_challenge(&input, &output), false);
    }
}

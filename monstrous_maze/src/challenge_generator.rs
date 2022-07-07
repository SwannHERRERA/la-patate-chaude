use rand::Rng;
use rand::seq::SliceRandom;

use utils::file_utils::read_mazes_file_macro;

use crate::models::MonstrousMazeInput;

pub fn generate_monstrous_maze_challenge() -> MonstrousMazeInput {
    let mut rng = rand::thread_rng();
    let mazes: Vec<String> = read_mazes_file_macro()
        .split('\n')
        .map(|maze| maze.replace("\\n", "\n"))
        .collect();

    let maze = mazes.choose(&mut rng).expect("Unable to choose maze");
    MonstrousMazeInput{
        grid: maze.clone(),
        endurance: rng.gen_range(2..=4),
    }
}

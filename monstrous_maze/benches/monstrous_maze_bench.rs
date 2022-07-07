use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand;
use rand::Rng;

use monstrous_maze::challenge_resolve::MonstrousMazeResolver;
use monstrous_maze::models::MonstrousMazeInput;
use utils::file_utils::read_file;

fn criterion_benchmark_1(c: &mut Criterion) {
    let mazes: Vec<String> = read_file("data/mazes.txt")
        .split('\n')
        .map(|maze| maze.replace("\\n", "\n"))
        .collect();

    let mut rand = rand::thread_rng();

    c.bench_function("Benchmark monstrous maze 100 different mazes", |b| {
        b.iter(|| {
            MonstrousMazeResolver::resolve_monstrous_maze_challenge(black_box(
                &MonstrousMazeInput {
                    endurance: 2,
                    grid: mazes[rand.gen_range(0..mazes.len())].clone(),
                },
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark_1);
criterion_main!(benches);

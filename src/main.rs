use clap::Parser;
use std::fs;
use toml;
use tubes::*;

mod finder;

#[derive(Parser, Clone)]
struct Cli {
    #[clap(parse(from_os_str))]
    tubes_file_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let filepath = args.tubes_file_path.as_os_str().to_str().unwrap();
    let contents = fs::read_to_string(args.tubes_file_path.clone())
        .expect("Something went wrong while reading the file");
    let tube_array: TubeArray = toml::from_str(&contents).unwrap();
    let initial_state = TubeState::from_tube_array(tube_array);
    println!("Reading tubes from {}.", filepath);
    println!("\n-- Initial State ------------\n");
    println!("{}", initial_state);
    let solved_state = solve_bfs(&initial_state);
    println!("-- Moves --------------------\n");
    for (idx, action) in solved_state.actions.iter().enumerate() {
        println!("step {}: {}", idx + 1, action);
    }
    println!();
    println!("-- Solved State -------------\n");
    println!("{}", solved_state.state);
}

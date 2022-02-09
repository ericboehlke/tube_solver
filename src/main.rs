use clap::{AppSettings, Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::io::Write;
use tubes::*;

mod finder;

#[derive(Parser, Clone)]
#[clap(name = "tubes")]
#[clap(about = "A program to solve all of your tube game related problems.", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Solve {
        #[clap(parse(from_os_str))]
        tubes_file_path: std::path::PathBuf,
    },
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Scan {
        #[clap(parse(from_os_str))]
        tubes_image_path: std::path::PathBuf,
        #[clap(parse(from_os_str))]
        tubes_output_file_path: std::path::PathBuf,
    },
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Solve { tubes_file_path} => {
            let filepath = tubes_file_path.as_os_str().to_str().unwrap();
            let contents = fs::read_to_string(tubes_file_path.clone())
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
        Commands::Scan { tubes_image_path, tubes_output_file_path} => {
            println!("scanning");
            let level_img = image::open(&Path::new(tubes_image_path)).unwrap();
            let level_img = finder::crop_level(&level_img);
            let tube_centers = finder::find_tubes(&level_img);
            let tubes = finder::extract_tube_colors(&level_img, tube_centers.clone());
            let mut output = fs::File::create(tubes_output_file_path).unwrap();
            let serialized_tubes = toml::to_string(&tubes.to_tube_array()).unwrap();
            let _ = write!(output, "{}", serialized_tubes);
        }
    }
}

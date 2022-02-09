# Tubes

This is some code used to solve the Water Sort Puzzle iPhone game.
It turns out the whole game is just navigating a graph so it can be solved with graph search algorithms such as breadth first search.

# How to Run

1. First make sure you have Rust. https://www.rust-lang.org/tools/install
2. Then clone this repo using `git clone https://github.com/ericboehlke/tube_solver.git`.
3. Finally, navigate to the root of this repo on your computer and run `cargo run --release solve levels/1.toml`. This will compile the code in `main.rs` and run the solver on level 1.

# Creating a Tube File from a Screenshot

1. Take a screenshot of the initial game state and save it on your computer.
2. Use `cargo run --release scan <path/to/screenshot.png> output.toml`.
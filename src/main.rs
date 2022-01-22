use tubes::{self, solve_bfs};

fn main() {
    let tube1 = tubes::Tube::new(tubes::Color::Orange, tubes::Color::Empty, tubes::Color::Empty, tubes::Color::Empty);
    let tube2 = tubes::Tube::new(tubes::Color::Orange, tubes::Color::Orange, tubes::Color::Orange, tubes::Color::Empty);
    let initial_state = tubes::TubeState { tubes: vec![tube1, tube2] };

    let action_list = solve_bfs(&initial_state);
    for a in action_list {
        println!("{:?}", a);
    }
}

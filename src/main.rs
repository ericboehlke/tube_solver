use tubes::{self, Color, Tube, TubeState, solve_bfs, EMPTY_TUBE};

fn main() {
    println!("Level 1");
    let tube1 = Tube::new(Color::Orange, Color::Empty, Color::Empty, Color::Empty);
    let tube2 = Tube::new(Color::Orange, Color::Orange, Color::Orange, Color::Empty);
    let initial_state = TubeState { tubes: vec![tube1, tube2] };

    let action_list = solve_bfs(&initial_state);
    for a in action_list {
        println!("{:?}", a);
    }
    // expected: TransferAction { send_idx: 0, recv_idx: 1 }

    println!("Level 2");
    let tube1 = Tube::new(Color::Blue, Color::Orange, Color::Blue, Color::Orange);
    let tube2 = Tube::new(Color::Orange, Color::Blue, Color::Orange, Color::Blue);
    let tube3 = EMPTY_TUBE;
    let initial_state = TubeState { tubes: vec![tube1, tube2, tube3] };

    let action_list = solve_bfs(&initial_state);
    for a in action_list {
        println!("{:?}", a);
    }
    // expected:
    // TransferAction { send_idx: 0, recv_idx: 2 }
    // TransferAction { send_idx: 1, recv_idx: 0 }
    // TransferAction { send_idx: 1, recv_idx: 2 }
    // TransferAction { send_idx: 0, recv_idx: 1 }
    // TransferAction { send_idx: 0, recv_idx: 2 }
    // TransferAction { send_idx: 1, recv_idx: 0 }
    // TransferAction { send_idx: 1, recv_idx: 2 }
}

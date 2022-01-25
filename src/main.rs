use tubes::{self, LiquidColor, Tube, TubeState, solve_bfs, EMPTY_TUBE};

fn main() {
    println!("Level 1");
    let tube1 = Tube::new(LiquidColor::Orange, LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty);
    let tube2 = Tube::new(LiquidColor::Orange, LiquidColor::Orange, LiquidColor::Orange, LiquidColor::Empty);
    let initial_state = TubeState { tubes: vec![tube1, tube2] };

    let action_list = solve_bfs(&initial_state);
    for a in action_list {
        println!("{:?}", a);
    }
    // expected: TransferAction { send_idx: 0, recv_idx: 1 }

    println!("Level 2");
    let tube1 = Tube::new(LiquidColor::Blue, LiquidColor::Orange, LiquidColor::Blue, LiquidColor::Orange);
    let tube2 = Tube::new(LiquidColor::Orange, LiquidColor::Blue, LiquidColor::Orange, LiquidColor::Blue);
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

    println!("Level 3");
    let tube1 = Tube::new(LiquidColor::Blue, LiquidColor::Orange, LiquidColor::Red, LiquidColor::Blue);
    let tube2 = Tube::new(LiquidColor::Orange, LiquidColor::Orange, LiquidColor::Red, LiquidColor::Blue);
    let tube3 = Tube::new(LiquidColor::Red, LiquidColor::Blue, LiquidColor::Orange, LiquidColor::Red);
    let tube4 = EMPTY_TUBE;
    let tube5 = EMPTY_TUBE;
    let initial_state = TubeState { tubes: vec![tube1, tube2, tube3, tube4, tube5] };

    let action_list = solve_bfs(&initial_state);
    for a in action_list {
        println!("{:?}", a);
    }
    // expected:
}

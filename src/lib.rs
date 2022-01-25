use std::collections::VecDeque;
use std::fmt;
use colored::*;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LiquidColor {
    Empty,
    Orange,
    Blue,
    Red,
}

// used for setting the color in the debug formatter
impl fmt::Display for LiquidColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiquidColor::Empty => write!(f, "black"),
            LiquidColor::Orange => write!(f, "yellow"),
            LiquidColor::Blue => write!(f, "blue"),
            LiquidColor::Red => write!(f, "red"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tube {
    layers: [LiquidColor; 4]
}

impl Tube {
    /// Returns true if all of the colors in the tube are empty
    ///
    /// ```
    /// use tubes::LiquidColor;
    /// use tubes::Tube;
    /// let empty_tube = Tube::new(LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty);
    /// let half_tube = Tube::new(LiquidColor::Orange, LiquidColor::Orange, LiquidColor::Empty, LiquidColor::Empty);
    /// assert_eq!(empty_tube.isempty(), true);
    /// assert_eq!(half_tube.isempty(), false);
    /// ```
    pub fn isempty(&self) -> bool {
        return 4 == self.howempty();
    }

    
    pub fn new(layer0: LiquidColor, layer1: LiquidColor, layer2: LiquidColor, layer3: LiquidColor) -> Tube {
        let new_tube = Tube { layers: [layer3, layer2, layer1, layer0] };
        if !new_tube.checkrep() {
            panic!("invalid tube");
        }
        return new_tube;
    }

    fn checkrep(&self) -> bool {
        let mut non_empty_flag = false;
        for layer in self.layers {
            if layer != LiquidColor::Empty {
                non_empty_flag = true;
            } else {
                if non_empty_flag {
                    return false
                }
            }
        }
        return true
    }

    /// Returns the volume and color of the top color of liquid
    ///
    /// ```
    /// use tubes::LiquidColor;
    /// use tubes::Tube;
    /// let tube1 = Tube::new(LiquidColor::Orange, LiquidColor::Blue, LiquidColor::Orange, LiquidColor::Blue);
    /// let tube2 = Tube::new(LiquidColor::Orange, LiquidColor::Orange, LiquidColor::Blue, LiquidColor::Blue);
    /// assert_eq!(tube1.topcolor(), (1, LiquidColor::Blue));
    /// assert_eq!(tube2.topcolor(), (2, LiquidColor::Blue));
    /// ```
    pub fn topcolor(&self) -> (i32, LiquidColor) {
        let mut top_color = LiquidColor::Empty;
        let mut color_count = 0;
        for layer in self.layers {
            if layer != LiquidColor::Empty {
                if top_color == LiquidColor::Empty {
                    top_color = layer;
                    color_count += 1;
                } else {
                    if layer == top_color {
                        color_count += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        return (color_count, top_color);
    }

    pub fn howempty(&self) -> i32 {
        let mut empty_count = 0;
        for layer in self.layers {
            if layer == LiquidColor::Empty {
                empty_count += 1;
            } else {
                break;
            }
        }
        return empty_count;
    }

    pub fn issolved(&self) -> bool {
        let empty_count = self.howempty();
        let (color_count, _top_color) = self.topcolor();
        return empty_count == 4 || color_count == 4;
    }

    pub fn pourtube(&self) -> Tube {
        let mut new_tube = *self;
        let (color_count, _color) = self.topcolor();
        let empty_count = self.howempty();
        for i in empty_count..empty_count+color_count {
            new_tube.layers[i as usize] = LiquidColor::Empty;
        }
        return new_tube;
    }

    pub fn addcolor(&self, color_count: i32, color: LiquidColor) -> (bool, Tube) {
        let empty_count = self.howempty();
        // Fail because recieve tube doesn't have enough empty space
        if color_count > empty_count {
            return (false, EMPTY_TUBE);
        }
        let mut new_tube = *self;
        for i in empty_count-color_count..empty_count {
            new_tube.layers[i as usize] = color;
        }
        return (true, new_tube);
    }
}

pub const EMPTY_TUBE: Tube = Tube { layers: [LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty] };

struct TubeTransferResult {
    success: bool, 
    send_tube: Tube, 
    recieve_tube: Tube
}

/// Transfers liquid from the send tube to the recieve tube following the rules of the game
///
/// The top block of colored liquid in the send tube will be transfered to the top of the 
/// recieve tube if the rules of the game allow for it. If the transfer is successful, the 
/// Success flag in TubeTransferResult will be true and a copy of the send and recieve
/// tubes will be the next elements repectively. If the transfer fails the Success flag 
/// will be false and no guarentees are made to the content of the Tubes
fn transfer(send: Tube, recv: Tube) -> TubeTransferResult {
    // Fail because send tube is empty
    if send.isempty() {
        return TubeTransferResult { success: false, send_tube: send, recieve_tube: recv };
    }
    let (send_color_count, send_color) = send.topcolor();
    let (_recv_color_count, recv_color) = recv.topcolor();
    // Fail because cannot pour onto different color
    if recv_color != send_color && recv_color != LiquidColor::Empty {
        return TubeTransferResult { success: false, send_tube: send, recieve_tube: recv };
    }
    let new_send = send.pourtube();
    let (recv_success, new_recv) = recv.addcolor(send_color_count, send_color);
    // Fail because addcolor failed
    if !recv_success {
        return TubeTransferResult { success: false, send_tube: send, recieve_tube: recv };
    }
    return TubeTransferResult { success: true, send_tube: new_send, recieve_tube: new_recv };
}

#[cfg(test)]
mod transfer_tests {
    use super::*;

    #[test]
    fn test_empty_transfer() {
        let tube1 = Tube::new(LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty);
        let tube2 = Tube::new(LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty);
        let transfer_result = transfer(tube1, tube2);
        let transfer_success = transfer_result.success;
        assert_eq!(transfer_success, false);
    }

    #[test]
    fn test_half_to_empty_transfer() {
        let tube1 = Tube::new(LiquidColor::Orange, LiquidColor::Orange, LiquidColor::Empty, LiquidColor::Empty);
        let tube2 = Tube::new(LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty);
        let transfer_result = transfer(tube1, tube2);
        assert_eq!(transfer_result.success, true);
        assert_eq!(transfer_result.send_tube, tube2);
        assert_eq!(transfer_result.recieve_tube, tube1);
    }

    #[test]
    fn test_empty_to_half_transfer() {
        let tube1 = Tube::new(LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty);
        let tube2 = Tube::new(LiquidColor::Orange, LiquidColor::Orange, LiquidColor::Empty, LiquidColor::Empty);
        let transfer_result = transfer(tube1, tube2);
        assert_eq!(transfer_result.success, false);
    }

    #[test]
    fn test_orange_to_blue_transfer() {
        let tube1 = Tube::new(LiquidColor::Orange, LiquidColor::Orange, LiquidColor::Empty, LiquidColor::Empty);
        let tube2 = Tube::new(LiquidColor::Blue, LiquidColor::Blue, LiquidColor::Empty, LiquidColor::Empty);
        let transfer_result = transfer(tube1, tube2);
        assert_eq!(transfer_result.success, false);
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TransferAction {
    send_idx: i32,
    recv_idx: i32,
}

#[derive(PartialEq, Clone)]
pub struct TubeState {
    pub tubes: Vec<Tube>,
}

impl TubeState {
    pub fn issolved(&self) -> bool {
        for tube in &self.tubes {
            if !tube.issolved() {
                return false;
            }
        }
        return true;
    }
}

impl fmt::Debug for TubeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text_lines = Vec::new();
        for _ in 0..4 { text_lines.push(String::from("  ")) }
        for mut t in self.tubes.clone() {
            for (i, l) in t.layers.iter_mut().enumerate() {
                text_lines[i].push_str(&"||  ".color(l.to_string()).to_string());
            }
        }
        let mut title = String::from("--Tubes--");
        title.push_str(&"-".repeat((1.max(text_lines[0].len() as i32 - 8 as i32)) as usize));
        write!(f, "{}\n{}\n{}\n{}\n{}\n", title, text_lines[0], text_lines[1], text_lines[2], text_lines[3])
    }
}

#[cfg(test)]
mod tube_state_test {
    use super::*;

    #[test]
    fn test_level_2_issolved() {
        let tube1 = Tube::new(LiquidColor::Blue, LiquidColor::Orange, LiquidColor::Blue, LiquidColor::Orange);
        let tube2 = Tube::new(LiquidColor::Orange, LiquidColor::Blue, LiquidColor::Orange, LiquidColor::Blue);
        let tube3 = EMPTY_TUBE;
        let state = TubeState { tubes: vec![tube1, tube2, tube3] };
        assert!(!tube1.issolved());
        assert!(!tube2.issolved());
        assert!(tube3.issolved());
        assert!(!state.issolved());
    }
}

/// Returns the neighboring states that can be reached with one transfer
///
/// Given a state of tubes this function returns all of the states that can be reached
/// by transfering the contents of one tube into another according to the rules of the 
/// transfer function.
pub fn neighbors(state: &TubeState) -> Vec<(TransferAction, TubeState)> {
    let mut neighboring_states = Vec::new();
    for (si, send_tube) in state.tubes.iter().enumerate() {
        for (ri, recv_tube) in state.tubes.iter().enumerate() {
            // Cannot transfer a tube into itself
            if si == ri { continue };
            let transfer_result = transfer(*send_tube, *recv_tube);
            if transfer_result.success {
                let mut neighboring_state = state.clone();
                neighboring_state.tubes[si] = transfer_result.send_tube;
                neighboring_state.tubes[ri] = transfer_result.recieve_tube;
                neighboring_states.push((TransferAction { send_idx: si as i32, recv_idx: ri as i32 }, neighboring_state));
            }
        }
    }
    return neighboring_states
}

#[cfg(test)]
mod neighbors_tests {
    use super::*;

    #[test]
    fn test_half_and_empty_neighbors() {
        let tube1 = Tube::new(LiquidColor::Orange, LiquidColor::Orange, LiquidColor::Empty, LiquidColor::Empty);
        let tube2 = Tube::new(LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty);
        let state = TubeState { tubes: vec![tube1, tube2] };
        let neighboring_states = neighbors(&state);
        let expected_state = TubeState { tubes: vec![tube2, tube1] };
        let transfer_action = TransferAction { send_idx: 0, recv_idx: 1 };
        assert_eq!(neighboring_states.len(), 1);
        assert!(neighboring_states.contains(&(transfer_action, expected_state)));
    }

    #[test]
    fn test_neighbors_3_tubes() {
        let oe_tube = Tube::new(LiquidColor::Orange, LiquidColor::Orange, LiquidColor::Empty, LiquidColor::Empty);
        let be_tube = Tube::new(LiquidColor::Blue, LiquidColor::Blue, LiquidColor::Empty, LiquidColor::Empty);
        let ee_tube = Tube::new(LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty, LiquidColor::Empty);
        let state = TubeState { tubes: vec![oe_tube, be_tube, ee_tube] };
        let neighboring_states = neighbors(&state);
        let expected_state2 = TubeState { tubes: vec![ee_tube, be_tube, oe_tube] };
        let expected_state4 = TubeState { tubes: vec![oe_tube, ee_tube, be_tube] };
        let transfer_action2 = TransferAction { send_idx: 0, recv_idx: 2 };
        let transfer_action4 = TransferAction { send_idx: 1, recv_idx: 2 };
        println!("{:?}", neighboring_states);
        assert_eq!(neighboring_states.len(), 2);
        assert!(neighboring_states.contains(&(transfer_action2, expected_state2)));
        assert!(neighboring_states.contains(&(transfer_action4, expected_state4)));
    }
}

pub struct TubeStateNode {
    actions: Vec<TransferAction>,
    state: TubeState,
}

/// Solves the game of tubes using bfs
pub fn solve_bfs(initial_state: &TubeState) -> Vec<TransferAction> {
    let mut explored: Vec<TubeState> = Vec::new();
    let mut q: VecDeque<TubeStateNode> = VecDeque::new();
    explored.push(initial_state.clone());
    q.push_back(TubeStateNode { actions: vec![TransferAction { send_idx: 0, recv_idx: 0 }], 
                                state: initial_state.clone() });
    let mut solution_state = None;
    while !q.is_empty() {
        let v = q.pop_front();
        match v {
            Some(y) => { 
                if y.state.issolved() {
                    solution_state = Some(y);
                    break;        
                } else {
                    for (action, state) in neighbors(&y.state) {
                        if !explored.contains(&&state) {
                            explored.push(state.clone());
                            let mut actions = y.actions.clone();
                            actions.push(action);
                            q.push_back(TubeStateNode { actions: actions, 
                                                        state: state.clone() });
                        }
                    }
                }
            },
            None => { break }
        }
    }
    match solution_state {
        None => {
            return vec![TransferAction {send_idx: 0, recv_idx: 0}];
        },
        Some(solution) => {
            let mut solution_actions = solution.actions.clone();
            solution_actions.remove(0);
            return solution_actions;
        }
    }
}

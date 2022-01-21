#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Empty,
    Orange,
    Blue,
}


/// Colors start from bottom to top
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tube (pub Color, pub Color, pub Color, pub Color);

impl Tube {
    /// Returns true if all of the colors in the tube are empty
    ///
    /// ```
    /// use tubes::Color;
    /// use tubes::Tube;
    /// let empty_tube = Tube(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
    /// let half_tube = Tube(Color::Orange, Color::Orange, Color::Empty, Color::Empty);
    /// assert_eq!(empty_tube.isempty(), true);
    /// assert_eq!(half_tube.isempty(), false);
    /// ```
    pub fn isempty(&self) -> bool {
        if self.0 == Color::Empty &&
            self.1 == Color::Empty &&
            self.2 == Color::Empty &&
            self.3 == Color::Empty {
                return true;
        } else {
            return false;
        }
    }
}

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
fn transfer(send: Tube, recieve: Tube) -> TubeTransferResult {
    if send.isempty() {
        return TubeTransferResult { success: false, send_tube: send, recieve_tube: recieve };
    } else {
        return TubeTransferResult { success: true, send_tube: recieve, recieve_tube: send };
    }
}

#[cfg(test)]
mod transfer_tests {
    use super::*;

    #[test]
    fn test_empty_transfer() {
        let tube1 = Tube(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let tube2 = Tube(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let transfer_result = transfer(tube1, tube2);
        let transfer_success = transfer_result.success;
        assert_eq!(transfer_success, false);
    }

    #[test]
    fn test_half_to_empty_transfer() {
        let tube1 = Tube(Color::Orange, Color::Orange, Color::Empty, Color::Empty);
        let tube2 = Tube(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let transfer_result = transfer(tube1, tube2);
        assert_eq!(transfer_result.success, true);
        assert_eq!(transfer_result.send_tube, tube2);
        assert_eq!(transfer_result.recieve_tube, tube1);
    }

    #[test]
    fn test_empty_to_half_transfer() {
        let tube1 = Tube(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let tube2 = Tube(Color::Orange, Color::Orange, Color::Empty, Color::Empty);
        let transfer_result = transfer(tube1, tube2);
        assert_eq!(transfer_result.success, false);
    }

    #[test]
    #[ignore]
    fn test_orange_to_blue_transfer() {
        let tube1 = Tube(Color::Orange, Color::Orange, Color::Empty, Color::Empty);
        let tube2 = Tube(Color::Blue, Color::Blue, Color::Empty, Color::Empty);
        let transfer_result = transfer(tube1, tube2);
        let ee_tube = Tube(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let bo_tube = Tube(Color::Blue, Color::Blue, Color::Orange, Color::Orange);
        assert_eq!(transfer_result.success, true);
        assert_eq!(transfer_result.send_tube, ee_tube);
        assert_eq!(transfer_result.recieve_tube, bo_tube);
    }
}

/// Returns the neighboring states that can be reached with one transfer
///
/// Given a state of tubes this function returns all of the states that can be reached
/// by transfering the contents of one tube into another according to the rules of the 
/// transfer function.
pub fn neighbors(state: Vec<Tube>) -> Vec<Vec<Tube>> {
    let mut neighboring_states = Vec::new();
    for (si, send_tube) in state.iter().enumerate() {
        for (ri, recv_tube) in state.iter().enumerate() {
            // Cannot transfer a tube into itself
            if si == ri { continue };
            let transfer_result = transfer(*send_tube, *recv_tube);
            if transfer_result.success {
                let mut neighboring_state = state.clone();
                neighboring_state[si] = transfer_result.send_tube;
                neighboring_state[ri] = transfer_result.recieve_tube;
                neighboring_states.push(neighboring_state);
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
        let tube1 = Tube(Color::Orange, Color::Orange, Color::Empty, Color::Empty);
        let tube2 = Tube(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let state = vec![tube1, tube2];
        let neighboring_states = neighbors(state);
        let expected_state = vec![tube2, tube1];
        assert_eq!(neighboring_states.len(), 1);
        assert!(neighboring_states.contains(&expected_state));
    }

    #[test]
    #[ignore]
    fn test_neighbors_3_tubes() {
        let oe_tube = Tube(Color::Orange, Color::Orange, Color::Empty, Color::Empty);
        let be_tube = Tube(Color::Blue, Color::Blue, Color::Empty, Color::Empty);
        let ee_tube = Tube(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let state = vec![oe_tube, be_tube, ee_tube];
        let neighboring_states = neighbors(state);
        let bo_tube = Tube(Color::Blue, Color::Blue, Color::Orange, Color::Orange);
        let ob_tube = Tube(Color::Orange, Color::Orange, Color::Blue, Color::Blue);
        let expected_state1 = vec![ee_tube, bo_tube, ee_tube];
        let expected_state2 = vec![ee_tube, be_tube, oe_tube];
        let expected_state3 = vec![ob_tube, ee_tube, ee_tube];
        let expected_state4 = vec![oe_tube, ee_tube, be_tube];
        println!("{:?}", neighboring_states);
        assert_eq!(neighboring_states.len(), 4);
        assert!(neighboring_states.contains(&expected_state1));
        assert!(neighboring_states.contains(&expected_state2));
        assert!(neighboring_states.contains(&expected_state3));
        assert!(neighboring_states.contains(&expected_state4));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Empty,
    Orange,
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
}

/// Returns the neighboring states that can be reached with one transfer
///
/// Given a state of tubes this function returns all of the states that can be reached
/// by transfering the contents of one tube into another according to the rules of the 
/// transfer function.
pub fn neighbors(state: Vec<Tube>) -> Vec<Vec<Tube>> {
    for send_tube in state.iter() {
        for recv_tube in state.iter() {
            // Cannot transfer a tube into itself
            if send_tube == recv_tube { continue };
            println!("Send tube {:?}, Recv tube {:?}", send_tube, recv_tube);
        }
    }
    return vec![state];
}

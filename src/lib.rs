#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Empty,
    Orange,
}


/// Colors start from bottom to top
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tube {
    a: Color,
    b: Color,
    c: Color,
    d: Color,
}

impl Tube {
    /// Tube constructor
    ///
    /// The colors start from the bottom to the top
    pub fn new(bot: Color, low: Color, hii: Color, top: Color) -> Self {
        Tube { a:bot, b:low, c:hii, d:top }
    }

    /// Returns true if all of the colors in the tube are empty
    ///
    /// ```
    /// use tubes::Color;
    /// use tubes::Tube;
    /// let empty_tube = Tube::new(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
    /// let half_tube = Tube::new(Color::Orange, Color::Orange, Color::Empty, Color::Empty);
    /// assert_eq!(empty_tube.isempty(), true);
    /// assert_eq!(half_tube.isempty(), false);
    /// ```
    pub fn isempty(&self) -> bool {
        if self.a == Color::Empty &&
            self.b == Color::Empty &&
            self.c == Color::Empty &&
            self.d == Color::Empty {
                return true;
        } else {
            return false;
        }
    }
}

/// Success, Send Tube, Recieve Tube
struct TubeTransferResult (bool, Tube, Tube);

/// Transfers liquid from the send tube to the recieve tube following the rules of the game
///
/// The top block of colored liquid in the send tube will be transfered to the top of the 
/// recieve tube if the rules of the game allow for it. If the transfer is successful, the 
/// Success flag in TubeTransferResult will be true and a copy of the send and recieve
/// tubes will be the next elements repectively. If the transfer fails the Success flag 
/// will be false and no guarentees are made to the content of the Tubes
fn transfer(send: Tube, recieve: Tube) -> TubeTransferResult {
    if send.isempty() {
        return TubeTransferResult(false, send, recieve);
    } else {
        return TubeTransferResult(true, recieve, send);
    }
}

#[cfg(test)]
mod transfer_tests {
    use super::*;

    #[test]
    fn test_empty_transfer() {
        let tube1 = Tube::new(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let tube2 = Tube::new(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let transfer_result = transfer(tube1, tube2);
        let transfer_success = transfer_result.0;
        assert_eq!(transfer_success, false);
    }

    #[test]
    fn test_half_to_empty_transfer() {
        let tube1 = Tube::new(Color::Orange, Color::Orange, Color::Empty, Color::Empty);
        let tube2 = Tube::new(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let transfer_result = transfer(tube1, tube2);
        assert_eq!(transfer_result.0, true);
        assert_eq!(transfer_result.1, tube2);
        assert_eq!(transfer_result.2, tube1);
    }

    #[test]
    fn test_empty_to_half_transfer() {
        let tube1 = Tube::new(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let tube2 = Tube::new(Color::Orange, Color::Orange, Color::Empty, Color::Empty);
        let transfer_result = transfer(tube1, tube2);
        assert_eq!(transfer_result.0, false);
    }
}


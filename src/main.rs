fn main() {
    println!("Hello, world!");
}

enum Color {
    Empty,
    Orange,
}


// Colors start from bottom to top
struct Tube (Color, Color, Color, Color);

// Success, Send Tube, Revieve Tube
struct TubeTransferResult (bool, Tube, Tube);

/// Transfers liquid from the send tube to the recieve tube following the rules of the game
///
/// The top block of colored liquid in the send tube will be transfered to the top of the 
/// recieve tube if the rules of the game allow for it. If the transfer is successful, the 
/// Success flag in TubeTransferResult will be true and a copy of the send and recieve
/// tubes will be the next elements repectively. If the transfer fails the Success flag 
/// will be false and no guarentees are made to the content of the Tubes
fn transfer(send: Tube, recieve: Tube) -> TubeTransferResult {
    return TubeTransferResult(false, send, recieve);
}

#[cfg(test)]
mod transfer_tests {
    use super::*;

    #[test]
    fn test_empty_transfer() {
        let tube1 = Tube(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let tube2 = Tube(Color::Empty, Color::Empty, Color::Empty, Color::Empty);
        let transfer_result = transfer(tube1, tube2);
        let transfer_success = transfer_result.0;
        assert_eq!(transfer_success, false);
    }
}


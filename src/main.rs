use tubes;

fn main() {
    println!("Hello, world!");
    let tube1 = tubes::Tube(tubes::Color::Orange, tubes::Color::Empty, tubes::Color::Empty, tubes::Color::Empty);
    let tube2 = tubes::Tube(tubes::Color::Orange, tubes::Color::Orange, tubes::Color::Orange, tubes::Color::Empty);
    let state = vec![tube1, tube2];
    tubes::neighbors(state);
}

use day07::p2;

fn main() {
    let input: &str = include_str!("../input.txt");
    let result = p2::process(input);
    println!("{}", result);
}

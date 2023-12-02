use day01::p1;

fn main() {
    let input: &str = include_str!("../input.txt");
    let result = p1::process(input);
    println!("{}", result);
}

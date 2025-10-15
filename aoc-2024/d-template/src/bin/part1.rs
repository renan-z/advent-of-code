use {{ crate_name }}::part1::process;

fn main(){
    let file = include_str!("../../input/input1.txt");
    let result = process(file);
    println!("{result}");
}
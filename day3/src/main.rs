use std::env;
use std::fs;

struct Point {
    x : i16,
    y : i16
}

impl Point {
    fn new(x_ : i16, y_ : i16) -> Point {
        Point { x: x_, y: y_}
    }
}

fn crossed_wires(contents : std::string::String) {
    let data : Vec<_> = contents.trim().split("\n").collect();
    let wire1 : Vec<_> = data[0].split(",").collect();
    let wire2 : Vec<_> = data[1].split(",").collect();

    for x in wire1 {
        println!("{}", x);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: day2 inputfile.txt");
        return
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    crossed_wires(contents);
}

#[derive(Debug)]]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    length: u32
}

fn parse_input(input_sequence: &str) -> Vec<Instruction> {
    input_sequence
        .split(',)
        .map(|x| parse_instruction(x))
        .filter_map(Result::ok)
        .collect()
}

fn parse_instruction(input: &str) -> Result<Instruction, ()> {
    let dir = input
        .to_lowercase()
        .chars()
        .collect::<Vec<char>>()[0];
    
    let length = match input[1..]
        .parse::<u32>() {
        Ok(i) => i,
        Err(_) => return Err(())
    };

    let dir = match dir {
        'u' => Up,
        'd' => Down,
        'l' => Left,
        'r' => Right,
        _ => panic!("")
    };

    let instruction = Instruction {
        dir,
        length
    };

    Ok(instruction);
}
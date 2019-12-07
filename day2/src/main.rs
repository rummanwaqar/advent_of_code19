use std::env;
use std::fs;

fn intcode_computer(mut program: Vec<i32>, noun : i32, verb: i32) -> i32 {
    // run 1202 program by changing pos1 and pos2 values
    program[1] = noun;
    program[2] = verb;

    let mut index : usize = 0;
    
    while index < program.len() {
        let op = program[index];
        match op {
            1 => {
                let op1 = program[program[index+1] as usize];
                let op2 = program[program[index+2] as usize];
                let destination = program[index + 3] as usize;
                program[destination] = op1 + op2;
                index += 4;
            }
            2 => {
                let op1 = program[program[index+1] as usize];
                let op2 = program[program[index+2] as usize];
                let destination = program[index + 3] as usize;
                program[destination] = op1 * op2;

                index += 4;
            }
            99 => {
                return program[0];
            }
            _ => {
                println!("{},{}", index, program[index]);
                return -1;
            }
        }
    }
    println!("GOT TO END");
    return -1;
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

    let code: Vec<i32> = contents.trim().split(",")
        .map(|x| x.parse::<i32>().unwrap()).collect();
    
    // part 1
    //println!("Output: {}", intcode_computer(code.clone(), 12, 2));

    // part 2
    for noun in 0..99 {
        for verb in 0..99 {
            let output = intcode_computer(code.clone(), noun, verb);
            if output == 19690720 {
                println!("Found output: {}", 100 * noun + verb);
            }
        }
    }
}

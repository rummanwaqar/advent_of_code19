use std::env;
use std::fs;
use std::io;

fn get_parameter(instruction : i32, base_index : usize, parameter_number : u8, program : &Vec<i32>) -> i32 {
    // paramater number will start at 1 (+ 1 because first two digits are instructions)
    // let mode = (instruction >> (parameter_number + 1)) & 1;
    let mode = instruction / (10_i32.pow((parameter_number + 1) as u32)) % 10;
    match mode {
        0 => { // by reference
            program[program[base_index + parameter_number as usize] as usize]
        }
        1 => { // by value
            program[base_index + parameter_number as usize]
        }
        _ => {
            println!("Invalid mode for instruction: {} paramater no: {}", instruction, parameter_number);
            0
        }
    }
}

fn intcode_computer(mut program: Vec<i32>) {
    let mut pointer : usize = 0;

    while pointer < program.len() {
        let instruction = program[pointer];
        let op = instruction % 100;
        match op {
            1 | 2 | 7 | 8 => { // handle op with format a something b = c
                let op1 = get_parameter(instruction, pointer, 1, &program);
                let op2 = get_parameter(instruction, pointer, 2, &program);
                let dest_ref = program[pointer + 3] as usize;
                match op {
                    1 => { // addition
                        program[dest_ref] = op1 + op2;
                    }
                    2 => { // multiplication
                        program[dest_ref] = op1 * op2;
                    }
                    7 => { // less than
                        program[dest_ref] = (op1 < op2) as i32;
                    }
                    8 => { // equals
                        program[dest_ref] = (op1 == op2) as i32;
                    }
                    _ => {

                    }
                }
                if op == 1 {
                    
                } else if op == 2 {
                    
                }
                pointer += 4; 
            }
            3 => { // input
                let dest_ref = program[pointer + 1] as usize;
                println!("Input Required:");
                let mut input_text = String::new();
                io::stdin()
                    .read_line(&mut input_text)
                    .expect("failed to read from stdin");
                match input_text.trim().parse::<i32>() {
                    Ok(i) => {
                        program[dest_ref] = i;
                        println!("{} -> {}", i, dest_ref);
                    }
                    Err(..) => {
                        println!("Input was not an integer. Halting!");
                        return
                    }
                };
                pointer += 2;
            }
            4 => { // output
                let op1 = get_parameter(instruction, pointer, 1, &program);
                println!("Output: {}", op1);
                pointer += 2;
            }
            5 | 6 => { // jump-if
                let op1 = get_parameter(instruction, pointer, 1, &program);
                let dest_ref = get_parameter(instruction, pointer, 2, &program) as usize;
                if (op == 5 && op1 != 0) || (op == 6 && op1 == 0) {
                    pointer = dest_ref;
                } else {
                    pointer += 3;
                }
            }
            99 => {
                println!("Exiting cleanly");
                break;
            }
            _ => {
                println!("Invalid op {} at pointer {}", op, pointer);
            }
        }
    }
    println!("Memory map at Exit {:?}", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: day5 inputfile.txt");
        return
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let code: Vec<i32> = contents.trim().split(",")
        .map(|x| x.parse::<i32>().unwrap()).collect();    
    intcode_computer(code);
}

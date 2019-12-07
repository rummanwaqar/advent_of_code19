use std::env;
use std::fs;

// Part A
// fn fuel_required(mass : i32) -> i32 {
//     mass / 3 - 2
// }

// Part B
fn fuel_required(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        return 0;
    }
    fuel + fuel_required(fuel)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: day1 inputfile");
        return
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut fuel_sum : i32 = 0;
    for num in contents.lines() {
        let mass : i32 = num.parse().unwrap();
        fuel_sum = fuel_sum + fuel_required(mass);
    }
    println!("Fuel: {}", fuel_sum);
}

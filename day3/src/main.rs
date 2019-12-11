use std::cmp;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: day3 inputfile.txt");
        return
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let data = contents.trim().split('\n').collect::<Vec<_>>();
    let wire1 = parse_wire_input(data[0]);
    let wire2 = parse_wire_input(data[1]);

    // part A
    let mut min_dist : i32 = std::i32::MAX;
    for x in &wire1 {
        for y in &wire2 {
            match get_intersection(*x, *y) {
                Ok(i) => {
                    let distance : i32 = i.0.abs() + i.1.abs();
                    if distance != 0 && distance < min_dist {
                        min_dist = distance;
                    }
                }
                Err(_e) => (),
            };
        }
        
    }
    println!("min distance: {}", min_dist);

    // part B
    let mut min_steps : i32 = 0;
    let mut break_loops = false;
    let mut steps1 = 0;
    for x in &wire1 {
        let mut steps2 = 0;
        for y in &wire2 {
            match get_intersection(*x, *y) {
                Ok(i) => {
                    if i.0 != 0 && i.1 != 0 {
                        steps1 += ((x.0).0 - i.0).abs() + ((x.0).1 - i.1).abs();
                        steps2 += ((y.0).0 - i.0).abs() + ((y.0).1 - i.1).abs();
                        min_steps = steps1 + steps2;
                        break_loops = true;
                        break;
                    }
                }
                Err(_e) => (),
            };
            steps2 += y.3;
        }
        if break_loops {
            break;
        }
        steps1 += x.3;
    }
    println!("min steps: {}", min_steps);
}

fn get_intersection(line1: Line, line2: Line) -> Result<Point, ()> {
    if line1.2 == line2.2 { // parallel / colinear
        // println!("COLINEAR");
        return Err(());
    } else { // perpendicular
        let ref hor : Line;
        let ref ver : Line;
        if line1.2 == Direction::Ver {
            ver = &line1;
            hor = &line2;
        } else {
            ver = &line2;
            hor = &line1;
        }
        // check intersection of x
        let min_x = cmp::min((hor.0).0, (hor.1).0);
        let max_x = cmp::max((hor.0).0, (hor.1).0);
        let min_y = cmp::min((ver.0).1, (ver.1).1);
        let max_y = cmp::max((ver.0).1, (ver.1).1);
        if (ver.0).0 >= min_x && (ver.0).0 <= max_x &&
            (hor.0).1 >= min_y && (hor.0).1 <= max_y {
            // intersection
            return Ok(Point((ver.0).0, (hor.0).1));
        }
    }
    Err(())
}

fn parse_wire_input(input_sequence: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut prev = Point(0, 0);
    for instruction in input_sequence.split(',') {
        let dir = instruction.chars().collect::<Vec<char>>()[0];
        let len = instruction[1..].parse::<i32>().unwrap();
        let new_point : Point;
        match dir {
            'U' => {
                new_point = Point(prev.0, prev.1 + len);
            }
            'D' => {
                new_point = Point(prev.0, prev.1 - len);
            }
            'R' => {
                new_point = Point(prev.0 + len, prev.1);
            }
            'L' => {
                new_point = Point(prev.0 - len, prev.1);
            }
            _ => {
                panic!("Oh no unknown instruction type");
            }
        }
        lines.push(Line(prev, new_point, 
            if dir == 'U' || dir == 'D' {Direction::Ver} else {Direction::Hor},
            len
        ));
        prev = new_point;
    }
    lines
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Ver,
    Hor
}

#[derive(Copy, Clone, Debug)]
// x, y
struct Point(i32, i32);

#[derive(Copy, Clone, Debug)]
// p1, p2, hor/ver, length
struct Line(Point, Point, Direction, i32);
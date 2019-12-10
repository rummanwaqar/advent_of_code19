fn is_increasing(password : std::string::String) -> bool {
    // a is used as a dummy initialization
    let mut previous = 'a';
    for x in password.chars() {
        if previous != 'a' {
            if x < previous {
                return false;
            }
        }
        previous = x;
    }
    true
}

fn contains_adjacent_pair(password : std::string::String) -> bool {
    let mut previous = 'a';
    let mut two_in_row = false;
    for x in password.chars() {
        if previous != 'a' {
            if previous == x {
                two_in_row = !two_in_row;
            }
        }
        previous = x;
    }
    two_in_row
}

// count number of possible passwords
fn count_passwords(start_range : u32, end_range : u32) -> u32 {
    println!("{}", contains_adjacent_pair("123344".to_string()));
    // for password in start_range..end_range {
    //     // 1063
    //     let chars :  = password.chars();
    //     is_increasing(password.to_string());
    //     contains_adjacent_pair(password.to_string());
    //     // is_increasing(password.to_string());
    //     // 
    //     // let mut previous : char = 'a';
    //     // let mut has_double = false;
    //     // let mut is_inc = true;
    //     // for x in password.to_string().chars() {
    //     //     if previous != 'a' {
    //     //         if previous == x {
    //     //             has_double = true;
    //     //         }
    //     //         if x < previous {
    //     //             is_inc = false;
    //     //             break;
    //     //         }
    //     //     }
    //     //     previous = x;
    //     // }
    //     // if is_inc && has_double {
    //     //     count += 1;
    //     // }
    // }
    123
}

fn main() {
    println!("Answer: {}", count_passwords(246540, 787419))
}

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
    let mut current_count = 1;
    for x in password.chars() {
        if previous != 'a' {
            if previous == x {
                current_count += 1;
            } else {
                if current_count == 2 {
                    return true;
                }
                current_count = 1;
            }
        }
        previous = x;
    }
    if current_count == 2 {
        return true;
    }
    false
}

// count number of possible passwords
fn count_passwords(start_range : u32, end_range : u32) -> u32 {
    let mut count = 0;
    for password in start_range..end_range {
        if is_increasing(password.to_string()) && 
            contains_adjacent_pair(password.to_string()) {
                count += 1;
            }
        }
    count
}

fn main() {
    println!("Answer: {}", count_passwords(246540, 787419))
}

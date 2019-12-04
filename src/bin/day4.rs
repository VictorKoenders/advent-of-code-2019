fn main(){
    println!("Part 1: {} options", generate_passwords(357253, 892942).len());
    println!("Part 2: {} options", generate_passwords_v2(357253, 892942).len());
}


fn generate_passwords(min: u32, max: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for i in min+1..max {
        if is_valid_password(i) {
            result.push(i);
        }
    }
    result
}

fn is_valid_password(num: u32) -> bool {
    let num_str = num.to_string();
    let bytes = num_str.as_bytes();

    // 1. password is a 6-digit number
    if bytes.len() != 6 {
        return false;
    }


    let mut prev = bytes[0];
    let mut has_adjacent = false;
    for byte in &bytes[1..] {
        // 4. going from left to right, the digits never decrease
        if *byte < prev {
            return false;
        }
        if prev == *byte {
            has_adjacent = true;
        }
        prev = *byte;
    }

    // 3. two adjacent digits are the same
    has_adjacent
}

fn generate_passwords_v2(min: u32, max: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for i in min+1..max {
        if is_valid_password_v2(i) {
            result.push(i);
        }
    }
    result
}


fn is_valid_password_v2(num: u32) -> bool {
    let num_str = num.to_string();
    let bytes = num_str.as_bytes();

    // 1. password is a 6-digit number
    if bytes.len() != 6 {
        return false;
    }


    let mut prev = bytes[0];
    let mut has_adjacent = false;
    for byte in &bytes[1..] {
        // 4. going from left to right, the digits never decrease
        if *byte < prev {
            return false;
        }
        if prev == *byte {
            has_adjacent = true;
        }
        prev = *byte;
    }

    if !has_adjacent {
        // 3. two adjacent digits are the same
        return false;
    }

    // check if we have at least 1 pair of exactly adjacent values
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        let repeating_count = bytes[i+1..].iter().take_while(|ch| **ch == c).count();

        if repeating_count == 1 {
            return true;
        }
        i += 1 + repeating_count;
    }

    false
}

#[test]
fn test_valid_password(){
    assert!(is_valid_password(111123));
    assert!(is_valid_password(111111));
    assert!(!is_valid_password(135679));
    assert!(!is_valid_password(223450));
    assert!(!is_valid_password(123789));
}

#[test]
fn test_valid_password_v2(){
    assert!(is_valid_password_v2(112233));
    assert!(is_valid_password_v2(111122));
    assert!(!is_valid_password_v2(123444));
}

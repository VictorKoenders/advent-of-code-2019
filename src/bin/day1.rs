extern crate aoc;

pub fn main() {
    let input = aoc::input!();
    let mut sum = 0;
    for num in input.lines().filter_map(|l| l.parse::<isize>().ok()) {
        let value = calc_fuel_value(num);
        sum += value;
    }
    println!("Part 1: {}", sum);

    let mut sum = 0;
    for num in input.lines().filter_map(|l| l.parse::<isize>().ok()) {
        let value = calc_fuel_value_recursive(num);
        sum += value;
    }
    println!("Part 2: {}", sum);
}

fn calc_fuel_value(input: isize) -> isize {
    (input / 3) - 2
}

fn calc_fuel_value_recursive(input: isize) -> isize {
    let mut result = calc_fuel_value(input);
    let mut output = 0;
    while result > 0 {
        output += result;
        result = calc_fuel_value(result);
    }
    output
}

#[test]
fn test_calc_fuel_value() {
    assert_eq!(2, calc_fuel_value(12));
    assert_eq!(2, calc_fuel_value(14));
    assert_eq!(654, calc_fuel_value(1969));
    assert_eq!(33583, calc_fuel_value(100_756));
}

#[test]
fn test_calc_fuel_value_recursive() {
    assert_eq!(2, calc_fuel_value_recursive(12));
    assert_eq!(966, calc_fuel_value_recursive(1969));
    assert_eq!(50346, calc_fuel_value_recursive(100_756));
}

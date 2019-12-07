extern crate aoc;

fn main() {
    let input = aoc::input!();
    let input = input
        .split(',')
        .map(|n| n.trim().parse::<usize>().expect("Could not parse num"))
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", evaluate_with_input(input.clone(), 12, 2));

    'outer: for noun in 0..=99 {
        for verb in 0..=99 {
            if evaluate_with_input(input.clone(), noun, verb) == 19_690_720 {
                println!("Found value at noun {}, verb {}", noun, verb);
                println!("Part 2: {}", noun * 100 + verb);
                break 'outer;
            }
        }
    }
}

fn evaluate_with_input(mut program: Vec<usize>, noun: usize, verb: usize) -> usize {
    program[1] = noun;
    program[2] = verb;

    evaluate(&mut program);
    program[0]
}

#[test]
fn test_evaluate_with_input() {
    let input = aoc::input!();
    let input = input
        .split(',')
        .map(|n| n.trim().parse::<usize>().expect("Could not parse num"))
        .collect::<Vec<_>>();

    assert_eq!(5_434_663, evaluate_with_input(input, 12, 2));
}

fn evaluate(input: &mut Vec<usize>) {
    let mut program_counter = 0;
    loop {
        let instruction = input[program_counter];
        match instruction {
            1 => {
                // add
                let left_address = input[program_counter + 1];
                let right_address = input[program_counter + 2];
                let target_address = input[program_counter + 3];

                let left = input[left_address];
                let right = input[right_address];

                let value = left + right;

                input[target_address] = value;
                program_counter += 4;
            }
            2 => {
                // multiply
                let left_address = input[program_counter + 1];
                let right_address = input[program_counter + 2];
                let target_address = input[program_counter + 3];

                let left = input[left_address];
                let right = input[right_address];

                let value = left * right;

                input[target_address] = value;
                program_counter += 4;
            }
            99 => {
                // finished
                return;
            }
            x => {
                panic!(
                    "Unknown opcode {} (program counter = {})",
                    x, program_counter
                );
            }
        }
    }
}

#[test]
fn test_evaluate() {
    let mut input = vec![1, 0, 0, 0, 99];
    evaluate(&mut input);
    assert_eq!(&[2, 0, 0, 0, 99], input.as_slice());

    let mut input = vec![2, 3, 0, 3, 99];
    evaluate(&mut input);
    assert_eq!(&[2, 3, 0, 6, 99], input.as_slice());

    let mut input = vec![2, 4, 4, 5, 99, 0];
    evaluate(&mut input);
    assert_eq!(&[2, 4, 4, 5, 99, 9801], input.as_slice());

    let mut input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    evaluate(&mut input);
    assert_eq!(&[30, 1, 1, 4, 2, 5, 6, 0, 99], input.as_slice());
}

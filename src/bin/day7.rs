extern crate aoc;
extern crate itertools;

use aoc::intcode::{Io, Memory, SimpleIo, OPERATIONS};
use itertools::Itertools;

fn main() {
    let input = aoc::input!()
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<isize>>();

    let result = find_biggest_permutation(input.clone(), 5);
    println!("Booster strength: {} (sequence {:?})", result.0, result.1);

    let result = find_biggest_permutation_p2(input);
    println!("Part 2: {} (sequence {:?})", result.0, result.1);
}

#[test]
fn test_inputs() {
    let result = find_biggest_permutation(
        vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ],
        5,
    );
    assert_eq!(43210, result.0);
    assert_eq!(&[4, 3, 2, 1, 0][..], result.1.as_slice());

    let result = find_biggest_permutation(
        vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ],
        5,
    );
    assert_eq!(54321, result.0);
    assert_eq!(&[0, 1, 2, 3, 4][..], result.1.as_slice());

    let result = find_biggest_permutation(
        vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ],
        5,
    );
    assert_eq!(65210, result.0);
    assert_eq!(&[1, 0, 4, 3, 2][..], result.1.as_slice());
}

fn find_biggest_permutation(program: Vec<isize>, count: isize) -> (isize, Vec<isize>) {
    let mut largest = (0, Vec::new());
    for io_input in (0..count).permutations(count as usize) {
        let mut val = 0;
        for amplifier in &io_input {
            let mut io = SimpleIo::new(vec![*amplifier, val]);
            let mut memory = Memory::new(program.clone());
            run_program(&mut memory, &mut io);

            let outputs = io.outputs();
            assert_eq!(outputs.len(), 1);
            assert_eq!(io.remaining_input_count(), 0);
            val = outputs[0];
        }
        if val > largest.0 {
            largest = (val, io_input);
        }
    }

    largest
}

#[test]
fn test_inputs_p2() {
    let result = find_biggest_permutation_p2(vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ]);
    assert_eq!(139629729, result.0);
    assert_eq!(&[9, 8, 7, 6, 5][..], result.1.as_slice());

    let result = find_biggest_permutation_p2(vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ]);
    assert_eq!(18216, result.0);
    assert_eq!(&[9, 7, 8, 5, 6][..], result.1.as_slice());
}

fn find_biggest_permutation_p2(program: Vec<isize>) -> (isize, Vec<isize>) {
    let mut largest = (0, Vec::new());
    for io_input in (5..10).permutations(5) {
        let mut programs = io_input
            .iter()
            .map(|&i| {
                let io = SimpleIo::new(vec![i]);
                let memory = Memory::new(program.clone());
                (io, memory)
            })
            .collect::<Vec<_>>();
        let programs_len = programs.len();
        let mut current_program = 0;
        programs[0].0.add_input(0);

        'main_loop: loop {
            let (io, memory) = &mut programs[current_program];
            let op = memory.current_and_increment();
            if op == 99 {
                break;
            }
            for operation in OPERATIONS {
                if operation(op, memory, io) {
                    if (op % 100) == 4 {
                        // write operation, add it to the next program's input, then switch to that program
                        let last_output = io.outputs().last().cloned().unwrap();
                        let next_program_index = (current_program + 1) % programs_len;
                        programs[next_program_index].0.add_input(last_output);
                        current_program = next_program_index;
                    }
                    continue 'main_loop;
                }
            }
            panic!("Unknwn operation {}, exiting", op);
        }

        // get the output of the last program (E)

        let output = programs
            .last()
            .unwrap()
            .0
            .outputs()
            .last()
            .cloned()
            .unwrap();
        if output > largest.0 {
            largest = (output, io_input);
        }
    }

    largest
}

fn run_program(memory: &mut Memory, io: &mut dyn Io) {
    'main_loop: loop {
        let op = memory.current_and_increment();
        if op == 99 {
            break 'main_loop;
        }
        for operation in OPERATIONS {
            if operation(op, memory, io) {
                continue 'main_loop;
            }
        }
        println!("Unknwn operation {}, exiting", op);
        break;
    }
}

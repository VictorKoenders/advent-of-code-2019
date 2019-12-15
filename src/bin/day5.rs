extern crate aoc;
use aoc::intcode::{Io, Memory, SimpleIo, OPERATIONS};

fn main() {
    let input = aoc::input!();
    let input = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<isize>>();
    let mut memory = Memory::new(input.clone());
    let mut io = SimpleIo::new(vec![1]);
    run_program(&mut memory, &mut io);
    println!("part 1: {:?}", io.outputs());

    let mut memory = Memory::new(input);
    let mut io = SimpleIo::new(vec![5]);
    run_program(&mut memory, &mut io);
    println!("part 1: {:?}", io.outputs());
}

#[test]
fn test_io() {
    let mut memory = Memory::new(vec![3, 0, 4, 0, 99]);
    let mut io = Io::new(vec![50]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![50], io.outputs);

    let mut memory = Memory::new(vec![1002, 4, 3, 4, 33]);
    let mut io = Io::new(vec![]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![1002, 4, 3, 4, 99], memory.mem);

    let mut memory = Memory::new(vec![1101, 100, -1, 4, 0]);
    let mut io = Io::new(vec![]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![1101, 100, -1, 4, 99], memory.mem);
}

#[test]
fn test_conditional_flow() {
    // Output 0 if input != 8
    let mut memory = Memory::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
    let mut io = Io::new(vec![6]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![0], io.outputs);

    // Output 1 if input == 8
    let mut memory = Memory::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
    let mut io = Io::new(vec![8]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![1], io.outputs);

    // Output 0 if input >= 8
    let mut memory = Memory::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    let mut io = Io::new(vec![16]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![0], io.outputs);

    // Output 0 if input >= 8
    let mut memory = Memory::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    let mut io = Io::new(vec![8]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![0], io.outputs);

    // Output 1 if input < 8
    let mut memory = Memory::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    let mut io = Io::new(vec![6]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![1], io.outputs);

    // Output 0 if input != 8
    let mut memory = Memory::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    let mut io = Io::new(vec![6]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![0], io.outputs);

    // Output 1 if input == 8
    let mut memory = Memory::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    let mut io = Io::new(vec![8]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![1], io.outputs);

    // Output 0 if input >= 8
    let mut memory = Memory::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    let mut io = Io::new(vec![16]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![0], io.outputs);

    // Output 0 if input >= 8
    let mut memory = Memory::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    let mut io = Io::new(vec![8]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![0], io.outputs);

    // Output 1 if input < 8
    let mut memory = Memory::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    let mut io = Io::new(vec![6]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![1], io.outputs);

    // output 999 if input < 8
    let mut memory = Memory::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);
    let mut io = Io::new(vec![6]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![999], io.outputs);

    // output 1000 if input == 8
    let mut memory = Memory::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);
    let mut io = Io::new(vec![8]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![1000], io.outputs);

    // output 1001 if input > 8
    let mut memory = Memory::new(vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ]);
    let mut io = Io::new(vec![15]);
    run_program(&mut memory, &mut io);
    assert_eq!(vec![1001], io.outputs);
}

fn run_program(memory: &mut Memory, io: &mut dyn Io) {
    'main_loop: loop {
        let op = memory.current_and_increment();
        for operation in OPERATIONS {
            if operation(op, memory, io) {
                continue 'main_loop;
            }
        }
        println!("Unknwn operation {}, exiting", op);
        break;
    }
}

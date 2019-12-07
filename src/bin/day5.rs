extern crate aoc;

fn main() {
    let input = aoc::input!();
    let input = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<isize>>();
    let mut memory = Memory::new(input.clone());
    let mut io = Io::new(vec![1]);
    run_program(&mut memory, &mut io);
    println!("part 1: {:?}", io.outputs);

    let mut memory = Memory::new(input);
    let mut io = Io::new(vec![5]);
    run_program(&mut memory, &mut io);
    println!("part 1: {:?}", io.outputs);
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

fn run_program(memory: &mut Memory, io: &mut Io) {
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

struct Io {
    inputs: Vec<isize>,
    outputs: Vec<isize>,
}

impl Io {
    pub fn new(inputs: Vec<isize>) -> Self {
        Self {
            inputs,
            outputs: Vec::new(),
        }
    }

    pub fn next_input(&mut self) -> isize {
        self.inputs.remove(0)
    }

    pub fn output(&mut self, val: isize) {
        self.outputs.push(val);
    }
}

pub struct Memory {
    mem: Vec<isize>,
    index: usize,
}

impl Memory {
    pub fn new(mem: Vec<isize>) -> Self {
        Self { mem, index: 0 }
    }

    pub fn current_and_increment(&mut self) -> isize {
        let val = self.mem[self.index];
        self.index += 1;
        val
    }

    pub fn current_as_parameter_and_increment(
        &mut self,
        operation: isize,
        parameter_index: usize,
    ) -> isize {
        let address = self.current_and_increment();
        ParameterMode::from_op_digits(operation, parameter_index + 2).get_value(address, self)
    }

    pub fn jump_to(&mut self, address: isize) {
        self.index = address as usize;
    }

    pub fn set(&mut self, address: isize, value: isize) {
        self.mem[address as usize] = value;
    }

    pub fn get(&self, address: isize) -> isize {
        self.mem[address as usize]
    }
}

pub enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    pub fn from_op_digits(op: isize, digit_index: usize) -> ParameterMode {
        let val = (op / (10isize.pow(digit_index as u32))) % 10;
        match val {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!(
                "Unknown parameter mode (op {}, digit {}, expected val {})",
                op, digit_index, val
            ),
        }
    }

    pub fn get_value(self, address: isize, memory: &Memory) -> isize {
        match self {
            ParameterMode::Position => memory.get(address),
            ParameterMode::Immediate => address,
        }
    }
}

static OPERATIONS: &[fn(isize, &mut Memory, &mut Io) -> bool] = &[
    add,
    multiply,
    read,
    write,
    jump_if_true,
    jump_if_false,
    less_than,
    equals,
];

fn add(operation: isize, memory: &mut Memory, _: &mut Io) -> bool {
    if (operation % 100) != 1 {
        return false;
    }
    let left = memory.current_as_parameter_and_increment(operation, 0);
    let right = memory.current_as_parameter_and_increment(operation, 1);
    let address = memory.current_and_increment();

    memory.set(address, left + right);

    true
}

fn multiply(operation: isize, memory: &mut Memory, _: &mut Io) -> bool {
    if (operation % 100) != 2 {
        return false;
    }
    let left = memory.current_as_parameter_and_increment(operation, 0);
    let right = memory.current_as_parameter_and_increment(operation, 1);
    let address = memory.current_and_increment();

    memory.set(address, left * right);

    true
}

fn read(operation: isize, memory: &mut Memory, io: &mut Io) -> bool {
    if (operation % 100) != 3 {
        return false;
    }

    let val = io.next_input();
    let address = memory.current_and_increment();
    memory.set(address, val);

    true
}

fn write(operation: isize, memory: &mut Memory, io: &mut Io) -> bool {
    if (operation % 100) != 4 {
        return false;
    }

    let value = memory.current_as_parameter_and_increment(operation, 0);
    io.output(value);

    true
}

fn jump_if_true(operation: isize, memory: &mut Memory, _: &mut Io) -> bool {
    if (operation % 100) != 5 {
        return false;
    }
    let value = memory.current_as_parameter_and_increment(operation, 0);
    let address = memory.current_as_parameter_and_increment(operation, 1);

    if value != 0 {
        memory.jump_to(address);
    }

    true
}

fn jump_if_false(operation: isize, memory: &mut Memory, _: &mut Io) -> bool {
    if (operation % 100) != 6 {
        return false;
    }
    let value = memory.current_as_parameter_and_increment(operation, 0);
    let address = memory.current_as_parameter_and_increment(operation, 1);

    if value == 0 {
        memory.jump_to(address);
    }

    true
}

fn less_than(operation: isize, memory: &mut Memory, _: &mut Io) -> bool {
    if (operation % 100) != 7 {
        return false;
    }
    let left = memory.current_as_parameter_and_increment(operation, 0);
    let right = memory.current_as_parameter_and_increment(operation, 1);
    let target = memory.current_and_increment();

    if left < right {
        memory.set(target, 1);
    } else {
        memory.set(target, 0);
    }

    true
}

fn equals(operation: isize, memory: &mut Memory, _: &mut Io) -> bool {
    if (operation % 100) != 8 {
        return false;
    }
    let left = memory.current_as_parameter_and_increment(operation, 0);
    let right = memory.current_as_parameter_and_increment(operation, 1);
    let target = memory.current_and_increment();

    if left == right {
        memory.set(target, 1);
    } else {
        memory.set(target, 0);
    }

    true
}

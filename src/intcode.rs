pub trait Io {
    fn add_input(&mut self, input: isize);
    fn next_input(&mut self) -> isize;
    fn remaining_input_count(&self) -> usize;
    fn output(&mut self, val: isize);
    fn outputs(&self) -> &[isize];
}

#[derive(Debug)]
pub struct SimpleIo {
    inputs: Vec<isize>,
    outputs: Vec<isize>,
}

impl SimpleIo {
    pub fn new(inputs: Vec<isize>) -> Self {
        Self {
            inputs,
            outputs: Vec::new(),
        }
    }
}

impl Io for SimpleIo {
    fn add_input(&mut self, input: isize) {
        self.inputs.push(input);
    }

    fn next_input(&mut self) -> isize {
        self.inputs.remove(0)
    }

    fn remaining_input_count(&self) -> usize {
        self.inputs.len()
    }

    fn output(&mut self, val: isize) {
        self.outputs.push(val);
    }

    fn outputs(&self) -> &[isize] {
        &self.outputs
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

pub static OPERATIONS: &[fn(isize, &mut Memory, &mut dyn Io) -> bool] = &[
    add,
    multiply,
    read,
    write,
    jump_if_true,
    jump_if_false,
    less_than,
    equals,
];

fn add(operation: isize, memory: &mut Memory, _: &mut dyn Io) -> bool {
    if (operation % 100) != 1 {
        return false;
    }
    let left = memory.current_as_parameter_and_increment(operation, 0);
    let right = memory.current_as_parameter_and_increment(operation, 1);
    let address = memory.current_and_increment();

    memory.set(address, left + right);

    true
}

fn multiply(operation: isize, memory: &mut Memory, _: &mut dyn Io) -> bool {
    if (operation % 100) != 2 {
        return false;
    }
    let left = memory.current_as_parameter_and_increment(operation, 0);
    let right = memory.current_as_parameter_and_increment(operation, 1);
    let address = memory.current_and_increment();

    memory.set(address, left * right);

    true
}

fn read(operation: isize, memory: &mut Memory, io: &mut dyn Io) -> bool {
    if (operation % 100) != 3 {
        return false;
    }

    let val = io.next_input();
    let address = memory.current_and_increment();
    memory.set(address, val);

    true
}

fn write(operation: isize, memory: &mut Memory, io: &mut dyn Io) -> bool {
    if (operation % 100) != 4 {
        return false;
    }

    let value = memory.current_as_parameter_and_increment(operation, 0);
    io.output(value);

    true
}

fn jump_if_true(operation: isize, memory: &mut Memory, _: &mut dyn Io) -> bool {
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

fn jump_if_false(operation: isize, memory: &mut Memory, _: &mut dyn Io) -> bool {
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

fn less_than(operation: isize, memory: &mut Memory, _: &mut dyn Io) -> bool {
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

fn equals(operation: isize, memory: &mut Memory, _: &mut dyn Io) -> bool {
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

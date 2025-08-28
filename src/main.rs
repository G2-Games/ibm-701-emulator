mod accumulator;

/// 36 bits out of a 64 bit type
pub const FULL_WORD_MASK: u64 = 0xFFFFFFFFF;
/// The upper 18 bits of a 36 bit section of a 64 bit type
pub const LO_WORD_MASK: u64 = 0x3FFFF;
/// The lower 18 bits of a 36 bit section of a 64 bit type
pub const HI_WORD_MASK: u64 = 0xFFFFC0000;

pub const LO_WORD_SIGN_MASK: u64 = 0x20000;
pub const LO_WORD_VALUE_MASK: u64 = LO_WORD_MASK >> 1;
pub const FULL_WORD_VALUE_MASK: u64 = FULL_WORD_MASK >> 1;

use std::fmt::Display;

use accumulator::{get_full_word, get_half_word, to_full_word, to_half_word, Accumulator};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive as _;

fn main() {
    let instructions = [
        Instruction::new(true,  Opcode::R_ADD,  4),
        Instruction::new(true,  Opcode::ADD,    6),
        //Instruction::new(false, Opcode::A_LEFT, 1),
        Instruction::new(true,  Opcode::STORE,  10),
        Instruction::new(false, Opcode::STOP,   0)
        //Instruction::new(true,  Opcode::TR,     0),
    ];
    let instructions = pack_instructions(&instructions);

    let mut emulator = Emulator::new();

    emulator.memory[..instructions.len()].copy_from_slice(&instructions);
    emulator.memory[4 / 2] = 0x7FFFFFFFC;
    emulator.memory[6 / 2] = 7;

    // loop {
    //     emulator.step();
    //     std::thread::sleep(std::time::Duration::from_millis(500));
    // }
    emulator.run();

    emulator.print_debug();
    emulator.print_full_memory();
}

fn pack_instructions(inst_list: &[Instruction]) -> Vec<u64> {
    let mut inst_bits = Vec::new();

    for inst_pair in inst_list.chunks(2) {
        let mut new_value = 0;
        new_value |= inst_pair[0].as_bits_low();
        if inst_pair.len() == 2 {
            new_value |= inst_pair[1].as_bits_high();
        } else {
            new_value |= HI_WORD_MASK;
        }
        inst_bits.push(new_value);
    }

    inst_bits
}

#[allow(clippy::zero_prefixed_literal, clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone, Copy, FromPrimitive)]
enum Opcode {
    STOP    = 00, // Stop and Transfer
    TR      = 01, // Transfer
    TR_OV   = 02, // Transfer on Overflow
    TR_PLUS = 03, // Transfer on Plus
    TR_ZERO = 04, // Transfer on Zero
    SUB     = 05, // Subtract
    R_SUB   = 06, // Reset and Subtract
    SUB_AB  = 07, // Subtract Absolute Value
    NO_OP   = 08, // No Operation
    ADD     = 09, // Add
    R_ADD   = 10, // Reset and Add
    ADD_AB  = 11, // Add Absolute Value
    STORE   = 12, // Store
    STORE_A = 13, // Store Address
    STORE_MQ = 14, // Store Contents of MQ Register
    LOAD_MQ = 15, // Load MQ Register
    MPY     = 16, // Multiply
    MPY_R   = 17, // Multiply and Round
    DIV     = 18, // Divide
    ROUND   = 19, // Round
    L_LEFT  = 20, // Long Left Shift
    L_RIGHT = 21, // Long Right Shift
    A_LEFT  = 22, // Accumulator Left Shift
    A_RIGHT = 23, // Accumulator Right Shift
    READ    = 24, // Prepare to Read
    READ_B  = 25, // Prepare to Read Backward
    WRITE   = 26, // Prepare to Write
    WRITE_EF = 27, // Write End of File
    REWIND  = 28, // Rewind Tape
    SET_DR  = 29, // Set Drum Address
    SENSE   = 30, // Sense and Skip or Control
    COPY    = 31, // Copy and Skip
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: Opcode,
    address: u16,
    sign: bool,
}

impl Instruction {
    fn new(sign: bool, opcode: Opcode, address: u16) -> Self {
        Self {
            opcode,
            address,
            sign,
        }
    }

    fn address_signed(&self) -> i16 {
        if !self.sign {
            self.address as i16
        } else {
            -(self.address as i16)
        }
    }

    fn as_bits_high(&self) -> u64 {
        self.as_bits_low() << 18
    }

    fn as_bits_low(&self) -> u64 {
        let mut output = 0;
        output |= (self.sign as u64) << 17;
        output |= (self.opcode as u64) << 12;
        output |= self.address as u64;

        output
    }

    fn from_bits_high(bytes: u64) -> Self {
        Self::from_bits_low(bytes >> 18)
    }

    fn from_bits_low(bytes: u64) -> Self {
        let sign = bytes & 0b100000000000000000 != 0;

        let opcode = (bytes & 0b011111000000000000) >> 12;
        let opcode = Opcode::from_u64(opcode).unwrap();

        let address = (bytes & 0b000000111111111111) as u16;

        Instruction {
            opcode,
            address,
            sign,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sign = "+";
        if self.sign {
            sign = "-";
        }

        let opcode = format!("{:?}", self.opcode);

        write!(f, "{} | {:<10} | {}", sign, opcode, self.address)
    }
}

#[derive(Debug, Clone, Copy)]
struct Emulator {
    instruction_counter: u16,

    memory_register: i64,
    accumulator_register: Accumulator,
    multiplier_quotient_register: i64,

    memory: [u64; 2048],

    halt: bool,
}

impl Emulator {
    fn new() -> Self {
        Self {
            instruction_counter: 0,
            accumulator_register: Accumulator::default(),
            multiplier_quotient_register: 0,
            memory_register: 0,

            memory: [FULL_WORD_MASK; 2048],
            halt: false,
        }
    }

    fn increment_instruction(&mut self) {
        self.instruction_counter += 1;

        if self.instruction_counter >= 2u16.pow(12) {
            self.instruction_counter = 0;
        }
    }

    fn run(&mut self) {
        // Un-halt the machine, because we just told it to run
        self.halt = false;

        while !self.halt && self.instruction_counter < 4096 {
            self.step();
        }
    }

    fn step(&mut self) -> bool {
        let counter_value = self.instruction_counter as usize;

        // Move on to the next instruction (unless this is modified...)
        self.increment_instruction();

        let inst_value = self.memory[counter_value / 2];

        let instruction = if counter_value.is_multiple_of(2) {
            Instruction::from_bits_low(inst_value)
        } else {
            Instruction::from_bits_high(inst_value)
        };

        println!(" {:<6} | {}", self.instruction_counter, instruction);

        self.execute(instruction);

        false
    }

    fn read_memory(&self, location: i16) -> i64 {
        if location < 0 {
            get_full_word(self.memory[location.unsigned_abs() as usize / 2])
        } else {
            get_half_word(self.memory[location.unsigned_abs() as usize / 2], location.abs() % 2 == 0)
        }
    }

    fn write_memory(&mut self, location: i16, value: i64) {
        if location < 0 {
            self.memory[location.unsigned_abs() as usize / 2] = to_full_word(value);
        } else {
            self.memory[location.unsigned_abs() as usize / 2] = to_half_word(value, location.abs() % 2 == 0);
        }
    }

    fn print_debug(&self) {
        println!(">----<");
        println!(" IC: {}", self.instruction_counter);
        println!("MEM: {:b}", to_full_word(self.memory_register));
        println!(
            "ACC: {{ value: {:b}, overflow: {}, sign: {} }}",
            self.accumulator_register.value,
            self.accumulator_register.overflow,
            self.accumulator_register.sign,
        );
        println!(" MQ: {:?}", self.multiplier_quotient_register);
        println!(">----<");
    }

    fn print_instructions(&self) {
        for value in self.memory.iter().flat_map(|v| [v & LO_WORD_MASK, v & HI_WORD_MASK]) {
            println!("{}", Instruction::from_bits_low(value));
        }
    }

    fn print_full_memory(&self) {
        for (rn, row) in self.memory.chunks(16).enumerate() {
            for (cn, value) in row.iter().enumerate() {

                let mut color1 = "";
                let mut color2 = "";
                if *value != 0xFFFFFFFFF {
                    color1 = "\x1b[93m";
                    color2 = "\x1b[0m";
                }
                if rn + cn == self.instruction_counter as usize {
                    color1 = "\x1b[92m";
                    color2 = "\x1b[0m";
                }

                print!("{color1}{:09X}{color2} ", value)
            }
            println!()
        }

        println!("Green:  Instruction Counter location\nOrange: Nonzero values");
    }

    fn print_address(&self, sign: bool, addr: usize) {
        let value = self.memory[addr / 2];

        let mut sign = "+";
        if value < 0 {
            sign = "-";
        }

        println!("\x1b[1m {:<6} | {} |            | {}\x1b[0m <-- DEBUG", addr, sign, value);
    }

    fn execute(&mut self, inst: Instruction) {
        match inst.opcode {
            Opcode::STOP => {
                self.instruction_counter = inst.address;
                self.halt = true;
            },
            Opcode::TR => {
                self.instruction_counter = inst.address;
            },
            Opcode::TR_OV => todo!(),
            Opcode::TR_PLUS => todo!(),
            Opcode::TR_ZERO => todo!(),
            Opcode::SUB => todo!(),
            Opcode::R_SUB => todo!(),
            Opcode::SUB_AB => todo!(),
            Opcode::NO_OP => todo!(),
            Opcode::ADD => {
                self.memory_register = self.read_memory(inst.address_signed());

                self.accumulator_register.add(self.memory_register);
            },
            Opcode::R_ADD => {
                self.memory_register = self.read_memory(inst.address_signed());

                self.accumulator_register.reset();
                self.accumulator_register.insert(self.memory_register);
            },
            Opcode::ADD_AB => todo!(),
            Opcode::STORE => {
                self.write_memory(inst.address_signed(), self.accumulator_register.value_as_i64());
            },
            Opcode::STORE_A => todo!(),
            Opcode::STORE_MQ => todo!(),
            Opcode::LOAD_MQ => todo!(),
            Opcode::MPY => todo!(),
            Opcode::MPY_R => todo!(),
            Opcode::DIV => todo!(),
            Opcode::ROUND => todo!(),
            Opcode::L_LEFT => todo!(),
            Opcode::L_RIGHT => todo!(),
            Opcode::A_LEFT => {
                self.accumulator_register.value <<= inst.address as usize;
            },
            Opcode::A_RIGHT => {
                self.accumulator_register.value >>= inst.address as usize;
            },
            Opcode::READ => todo!(),
            Opcode::READ_B => todo!(),
            Opcode::WRITE => todo!(),
            Opcode::WRITE_EF => todo!(),
            Opcode::REWIND => todo!(),
            Opcode::SET_DR => todo!(),
            Opcode::SENSE => todo!(),
            Opcode::COPY => todo!(),
        }
    }
}


enum Half {
    Upper,
    Lower,
}

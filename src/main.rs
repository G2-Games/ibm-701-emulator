use std::fmt::Display;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive as _;

fn main() {
    let instructions = [
        Instruction::new(true,  Opcode::R_ADD,  1492),
        Instruction::new(true,  Opcode::ADD,    1588),
        Instruction::new(false, Opcode::A_LEFT, 1),
        Instruction::new(true,  Opcode::STORE,  1812)
    ];

    let mut emulator = Emulator::new();

    for inst in instructions {
        println!("{inst}");
        emulator.execute(inst);
    }

    dbg!(emulator);
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

    fn as_bytes(&self) -> u64 {
        let mut output = 0;
        output |= (self.sign as u64) << 36;
        output |= (self.opcode as u64) << 30;
        output |= (self.address as u64) << 18;

        output
    }

    fn from_bytes(bytes: u64) -> Self {
        let sign = bytes & 0b1000000000000000000000000000000000000 != 0;

        let opcode = (bytes & 0b011111000000000000000000000000000000) >> 30;
        let opcode = Opcode::from_u64(opcode).unwrap();

        let address = ((bytes & 0b000000111111111111000000000000000000) >> 18) as u16;

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

        write!(f, " {} | {:<10} | {}", sign, opcode, self.address)
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Accumulator {
    pub q: bool,
    pub p: bool,
    pub value: i64,
}

impl Accumulator {
    fn reset(&mut self) {
        self.q = false;
        self.p = false;
        self.value = 0;
    }
}

#[derive(Debug, Clone, Copy)]
struct Emulator {
    instruction_counter: u16,

    //memory_register: i64,
    accumulator_register: Accumulator,
    multiplier_quotient_register: i64,

    memory: [i64; 4096],
}

impl Emulator {
    fn new() -> Self {
        Self {
            instruction_counter: 0,
            accumulator_register: Accumulator::default(),
            multiplier_quotient_register: 0,
            memory: [0i64; 4096],
        }
    }

    fn execute(&mut self, inst: Instruction) {
        match inst.opcode {
            Opcode::STOP => {
                self.instruction_counter = inst.address;
            },
            Opcode::TR => todo!(),
            Opcode::TR_OV => todo!(),
            Opcode::TR_PLUS => todo!(),
            Opcode::TR_ZERO => todo!(),
            Opcode::SUB => todo!(),
            Opcode::R_SUB => todo!(),
            Opcode::SUB_AB => todo!(),
            Opcode::NO_OP => todo!(),
            Opcode::ADD => {
                let mut addr = inst.address as i64;
                if inst.sign {
                    addr = -addr;
                }

                self.accumulator_register.value += addr
            },
            Opcode::R_ADD => {
                let mut addr = inst.address as i64;
                if inst.sign {
                    addr = -addr;
                }

                self.accumulator_register.value = addr
            },
            Opcode::ADD_AB => todo!(),
            Opcode::STORE => {
                let loc = inst.address as usize;
                if inst.sign {
                    self.memory[loc / 2] = self.accumulator_register.value;
                } else {
                    todo!("Half-Word store is not yet implemented");
                }
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
            Opcode::A_RIGHT => todo!(),
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

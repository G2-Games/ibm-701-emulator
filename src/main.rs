//! # Cool thing

use modular_bitfield::{bitfield, prelude::*};
use tape::Tape;

mod tape;
mod float;

fn main() {
    let mut tape = Tape::new('J');

    let inst = Instruction::new_nop();

    for i in 0..100 {
        tape.write(&inst.to_word());
    }

    while tape.rewind().is_some() {}

    const STEP: usize = 10;
    for i in (0..200).step_by(STEP) {
        println!("{}", i);
        for x in i..i + STEP {
            let word = tape.read();
            let inst = Instruction::from_bytes(word);
            println!("{:?}", inst);
        }
    }
}

#[bitfield]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    /// Sign of instruction (automatically supplied by SpeedCo I)
    sign: B1,
    #[skip]
    zero1: B1,
    /// R-code
    r_code: B3,
    #[skip]
    zero2: B1,
    /// OP₁
    op_1: Operation1,
    /// OP₂
    op_2: Operation2,
    /// D
    d: B10,
    #[skip]
    zero3: B1,
    /// A
    a: B10,
    /// B
    b: B10,
    #[skip]
    zero4: B2,
    /// L
    l: B3,
    /// C
    c: B10,
}

impl Instruction {
    fn new_nop() -> Self {
        Self::new()
            .with_op_1(Operation1::NoOp)
            .with_op_2(Operation2::NoOp)
    }

    fn to_word(&self) -> [u8; 9] {
        self.into_bytes().try_into().unwrap()
    }
}

// About Write Tape instructions:
//
// The block of information stored in electrostatic cells A to
// B is written on the designated tape. The most recent
// previous instruction affecting the same tape must be
// either WRITE or REWIND.

// About Read Forward Tape instructions:
//
// The first B — A + 1 words of the next block of informa­
// tion stored on the designated tape are read and stored in
// electrostatic cells A to B. (Notes 1, 3, and 4.)

#[derive(BitfieldSpecifier)]
#[bits = 12]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Operation1 {
    /// Add
    ///
    /// Q(A) + Q(B) = Q(C)
    Add = 658,

    /// Subtract
    ///
    /// Q(A) + Q(B) = Q(C)
    Sub = 696,

    /// Add absolute
    ///
    /// Q(A) + |Q(B)| = Q(C)
    AddAb = 699,

    /// Absolute add
    ///
    /// |Q(A)| + |Q(B)| = Q(C)
    AbAdd = 703,

    /// Subtract absolute
    ///
    /// Q(A) - |Q(B)| = Q(C)
    SubAb = 707,

    /// Absolute subtract
    ///
    /// |Q(A)| - |Q(B)| = Q(C)
    AbSub = 711,

    /// Multiply
    ///
    /// \[Q(A)] × \[Q(B)] = Q(C)
    Mpy = 715,

    /// Negative multiply
    ///
    /// -\[Q(A)] × \[Q(B)] = Q(C)
    NgMpy = 731,

    /// Divide
    ///
    /// \[Q(A)] ÷ \[Q(B)] = Q(C)
    Div = 734,

    /// Negative divide
    ///
    /// -\[Q(A)] ÷ \[Q(B)] = Q(C)
    NgDiv = 748,

    /// Square root
    ///
    /// √Q(A) = Q(C)
    Sqrt = 782,

    /// Sine
    ///
    /// sin\[Q(A)] = Q(C)
    Sine = 780,

    /// Arc tangent
    ///
    /// tan⁻¹\[Q(A)] = Q(C)
    Artan = 781,

    /// Exponential
    ///
    /// e^\[Q(A)] = Q(C)
    Exp = 783,

    /// Logarithm
    ///
    /// logₑ\[Q(A)] = Q(C)
    Ln = 784,

    /// Move
    ///
    /// The block of information stored in electrostatic cells A to B is stored
    /// in electrostatic cells C to C + B - A. See Appendix E.
    Move = 690,

    // WRITE
    /// Write tape J
    WrtpJ = 532,

    /// Write tape K
    WrtpK = 533,

    /// Write tape L
    WrtpL = 534,

    /// Write tape M
    WrtpM = 535,

    // READ FORWARD
    /// Read forward tape J
    RftpJ = 435,

    /// Read forward tape K
    RftpK = 437,

    /// Read forward tape L
    RftpL = 439,

    /// Read forward tape M
    RftpM = 441,

    // SKIP FORWARD
    /// Skip forward tape J
    SftpJ = 556,

    /// Skip forward tape K
    SftpK = 557,

    /// Skip forward tape L
    SftpL = 558,

    /// Skip forward tape M
    SftpM = 559,

    // SKIP BACKWARD
    /// Skip forward tape J
    SbtpJ = 546,

    /// Skip forward tape K
    SbtpK = 547,

    /// Skip forward tape L
    SbtpL = 548,

    /// Skip forward tape M
    SbtpM = 549,

    // REWIND
    /// Skip forward tape J
    RwtpJ = 572,

    /// Skip forward tape K
    RwtpK = 574,

    /// Skip forward tape L
    RwtpL = 576,

    /// Skip forward tape M
    RwtpM = 578,

    // END FILE
    /// Skip forward tape J
    EftpJ = 564,

    /// Skip forward tape K
    EftpK = 566,

    /// Skip forward tape L
    EftpL = 568,

    /// Skip forward tape M
    EftpM = 570,

    /// Write drum P
    WrdrP = 497,
    /// Write drum Q
    WrdrQ = 498,

    /// Read forward drum P
    RfdrP = 526,
    /// Read forward drum Q
    RfdrQ = 529,

    /// Print
    ///
    /// The printer paper is ejected. (See Appendix C.)
    Print = 580,

    /// Eject
    Eject = 767,

    /// No operation
    #[default]
    NoOp = 571,
}

#[derive(BitfieldSpecifier)]
#[bits = 8]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Operation2 {
    /// Transfer
    Tr = 104,
    /// Transfer plus
    TrPl = 109,
    /// Transfer minus
    TrMn = 115,
    /// Transfer zero
    TrZ = 112,
    /// Sense and transfer P
    SnTrP = 117,
    /// Sense and transfer Q
    SnTrQ = 120,

    // Transfer and Increase
    /// Transfer and Increase Ra
    TiA = 128,
    /// Transfer and Increase Rb
    TiB = 126,
    TiC = 125,
    TiAB = 130,
    TiBC = 127,
    TiAC = 129,
    TiABC = 131,

    TdA = 135,
    TdB = 133,
    TdC = 132,
    TdAB = 137,
    TdBC = 134,
    TdAC = 136,
    TdABC = 138,

    SetRA = 139,
    SetRB = 250,
    SetRC = 145,

    SkRA = 152,
    SkRB = 159,
    SkRC = 162,

    RAddA = 199,
    RAddB = 202,
    RAddC = 205,
    RAddD = 208,

    AddA = 177,
    AddB = 184,
    AddC = 190,
    AddD = 193,

    SubA = 211,
    SubB = 216,
    SubC = 221,
    SubD = 226,

    StA = 251,
    StB = 252,
    StC = 235,
    StD = 244,

    Skip = 165,

    PrCh = 232,
    StCh = 253,

    EChTr = 254,

    Stop = 123,

    #[default]
    NoOp = 000,
}

fn main() {
    println!("Hello, world!");
}

struct Instruction {
    sign: bool,
    r_code: u8,
    op_1: u16,
    op_2: u8,
    d: u16,
    a: u16,
    b: u16,
}

// About Write Tape instructions:
//
// The block of information stored in electrostatic cells A to
// B is written on the designated tape. The most recent
// previous instruction affecting the same tape must be
// either WRITE or REWIND.

// About Read Forward Tape instructions:
//
// The first B — A -j- 1 words of the next block of informa­
// tion stored on the designated tape are read and stored in
// electrostatic cells A to B. (Notes 1, 3, and 4.)

enum Operation {
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
    /// \[Q(A)] ÷ \[Q(B)] = Q(C)
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
    /// tan⁻¹\[Q(A)] = Q(C)
    Ln = 784,

    /// Move
    ///
    /// The block of information stored in electrostatic cells A to
    /// B is stored in electrostatic cells C to C T B — A. See
    /// Appendix E.
    Move = 690,

    /// Write tape J
    WrtpJ = 532,

    /// Write tape K
    WrtpK = 533,

    /// Write tape L
    WrtpL = 534,

    /// Write tape M
    WrtpM = 535,

    /// Read forward tape J
    RftpJ = 435,

    /// Read forward tape K
    RftpK = 437,

    /// Read forward tape L
    RftpL = 439,

    /// Read forward tape M
    RftpM = 441,
}

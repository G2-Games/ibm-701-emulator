use modular_bitfield::{bitfield, prelude::*};

#[bitfield]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Float {
    /// Sign of the quantity
    quantity_sign: B1,
    /// Fractional part
    fractional: B35,
    /// Sign of the exponent
    exponent_sign: B1,
    /// Exponent
    exponent: B17,
    #[skip]
    unused: B18,
}

use crate::{FULL_WORD_MASK, FULL_WORD_VALUE_MASK, LO_WORD_SIGN_MASK, LO_WORD_VALUE_MASK};

#[derive(Debug, Default, Clone, Copy)]
pub struct Accumulator {
    pub sign: bool,
    /// Overflow indication
    pub overflow: bool,
    /// Remaining 35 bits
    pub value: u64,
}

impl Accumulator {
    const FULL_ACCUMULATOR_SIZE_MASK: i64 = 0b1111111111111111111111111111111111111;

    pub fn insert(&mut self, value: i64) {
        self.sign = value < 0;
        self.value &= value.unsigned_abs() & FULL_ACCUMULATOR_SIZE_MASK;
    }

    pub fn value_as_i64(&self) -> i64 {
        if !self.sign {
            self.value as i64
        } else {
            -(self.value as i64)
        }
    }

    pub fn add(&mut self, value: i64) {
        let pq_initial_state = Self::test_pq(self.value);

        let result = (self.value_as_i64() + value) & Self::FULL_ACCUMULATOR_SIZE_MASK;
        self.insert(result);

        self.overflow = test_pq(result) != pq_initial_state;
    }

    fn test_pq(value: u64) -> (bool, bool) {
        let p_state = (value & LO_WORD_SIGN_MASK << 18) > 0;
        let q_state = (value & LO_WORD_SIGN_MASK << 19) > 0;

        (p_state, q_state)
    }

    pub fn reset(&mut self) {
        self.sign = false;
        self.value = 0;
    }
}

#[must_use]
pub fn get_full_word(raw: u64) -> i64 {
    let sign = raw & LO_WORD_SIGN_MASK << 18;
    let value = (raw & FULL_WORD_VALUE_MASK) as i64;

    if sign != 0 {
        -value
    } else {
        value
    }
}

#[must_use]
pub fn get_half_word(mut raw: u64, portion: bool) -> i64 {
    raw &= FULL_WORD_MASK;

    if portion { // Get the high portion
        raw >>= 18;
    }

    let sign = raw & LO_WORD_SIGN_MASK;
    let value = (raw & LO_WORD_VALUE_MASK) as i64;

    if sign != 0 {
        -value
    } else {
        value
    }
}

pub fn to_full_word(value: i64) -> u64 {
    let mut output = if value < 0 {
        LO_WORD_SIGN_MASK << 18
    } else {
        0
    };

    output |= value.unsigned_abs() & FULL_WORD_VALUE_MASK;

    output
}

pub fn to_half_word(value: i64, portion: bool) -> u64 {
    let mut output = if value < 0 {
        LO_WORD_SIGN_MASK
    } else {
        0
    };
    output |= value.unsigned_abs() & LO_WORD_VALUE_MASK;

    if portion {
        output <<= 18;
    }

    output
}

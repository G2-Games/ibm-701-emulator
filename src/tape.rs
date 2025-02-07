use std::ops::Range;

/// There is also a tape-storage section which includes four magnetic tape
/// units. Each tape, which may be up to 1400 feet long, is wound on a reel. The
/// tape itself is a non-metallic, oxide-coated band one-half inch wide. Binary
/// information is recorded on tape by means of magnetized spots. A block of
/// words recorded consecutively on a tape is called a record or a unit record.
/// The amount of information contained on each tape depends on the lengths of
/// the individual records, because there is a certain amount of space between
/// successive records to allow for starting and stopping the tape. It is
/// possible to store approximately 140,000 words on each tape. The machine can
/// read or write on a tape only through the medium of electrostatic storage. On
/// the average, about 10 milliseconds are needed for the tape to accelerate to
/// its reading or writing speed, after which the reading or writing of a unit
/// record takes place at the rate of 625 words per second. Because the tapes
/// are removable, a library of standard programming and mathematical tables may
/// be kept on tapes.
struct Tape {
    id: char,
    position: usize,
    records: Box<[u8; 140_000]>
}

const WORD_SIZE: usize = 9;

impl Tape {
    fn new(id: char) -> Self {
        let records = Box::new([0u8; 140_000]);

        Self {
            id,
            position: 0,
            records,
        }
    }

    const fn position(&self) -> usize {
        self.position
    }

    const fn position_range(&self) -> Range<usize> {
        self.position()..self.position() + 9
    }

    fn write(&mut self, word: &[u8; WORD_SIZE]) {
        let range = self.position_range();
        self.records[range].copy_from_slice(word);
    }

    fn read(&mut self) -> [u8; WORD_SIZE] {
        let range = self.position_range();
        self.records[range].try_into().unwrap()
    }

    fn erase(&mut self) {
        let range = self.position_range();
        self.records[range].fill(0);
    }

    fn rewind() {

    }
}

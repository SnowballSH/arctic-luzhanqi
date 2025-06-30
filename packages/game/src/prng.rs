pub struct PseudoRng(u64);

impl PseudoRng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// xorshift64* implementation
    pub fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        // xorshift64*
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x as u32).wrapping_mul(0x9E3779B1)
    }

    pub fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
        let len = range.end - range.start;
        if len == 0 {
            return range.start;
        }
        (self.next_u32() as usize % len) + range.start
    }
}

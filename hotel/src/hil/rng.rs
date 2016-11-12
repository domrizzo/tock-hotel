/// A Random Number Generator, based on the Rng trait from the rand crate.
pub trait Rng {
    /// Returns 4 bytes of random data.  Types that implement the Rng trait
    /// only need to implement this function.
    fn next_u32(&mut self) -> u32;

    /// Fills an arbitrarily sized buffer with random data.  The default
    /// implementation is based on next_u32().  Implementations that can fulfill
    /// the requirements more efficiently may wish to override this implementation
    /// for performance.
    fn fill_bytes(&mut self, buf: &mut [u8]) {
        let mut rem = 0;
        let mut data = 0;
        for byte in buf.iter_mut() {
            if rem == 0 {
                data = self.next_u32();
                rem = 4;
            }

            *byte = (data & 0xff) as u8;
            data >>= 8;
            rem -= 1;
        }
    }
}

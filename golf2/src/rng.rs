use hotel::hil::rng::Rng;
use kernel::{AppId, AppSlice, Driver, Shared};
use kernel::common::take_cell::TakeCell;

/// Driver for a random number generator, using the Rng trait.
pub struct RngDriver<'a, G: Rng + 'a> {
    rng: TakeCell<&'a mut G>,
}

impl<'a, G: Rng + 'a> RngDriver<'a, G> {
    /// Creates a new RngDriver.
    pub fn new(rng: &'a mut G) -> RngDriver<'a, G> {
        RngDriver { rng: TakeCell::new(rng) }
    }
}

impl<'a, G: Rng + 'a> Driver for RngDriver<'a, G> {
    /// Fills an application-provided buffer with random bytes.
    fn allow(&self, _: AppId, _: usize, mut slice: AppSlice<Shared, u8>) -> isize {
        self.rng
            .map(|rng| {
                rng.fill_bytes(slice.as_mut());
                0
            })
            .unwrap_or(-1)
    }
}

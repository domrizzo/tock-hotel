use core::cell::Cell;
use hil::digest::{Client, DigestEngine, DigestMode, DigestError};
use kernel::common::volatile_cell::VolatileCell;
use super::KEYMGR0_BASE_ADDRESS;

#[repr(C)]
struct Registers {
    cfg_msglen_lo: VolatileCell<u32>, // 0x400
    cfg_msglen_hi: VolatileCell<u32>, // 0x404
    cfg_en: VolatileCell<u32>, // 0x408
    cfg_wr_en: VolatileCell<u32>, // 0x40C
    trig: VolatileCell<u32>, // 0x410
    _padding_414: [u8; 0x440 - 0x414], // 0x414
    input_fifo: U32OrU8, // 0x440
    sts_h: [VolatileCell<u32>; 8], // 0x444
    key_w: [VolatileCell<u32>; 8], // 0x464
    sts: VolatileCell<u32>, // 0x484
    itcr: VolatileCell<u32>, // 0x488
    itop: VolatileCell<u32>, // 0x48C
    use_hidden_key: VolatileCell<u32>, // 0x490
    use_cert: VolatileCell<u32>, // 0x494
    cert_override: VolatileCell<u32>, // 0x498
    rand_stall_ctl: VolatileCell<u32>, // 0x49C
    execute_count_state: VolatileCell<u32>, // 0x4A0
    execute_count_max: VolatileCell<u32>, // 0x4A4
    cert_revoke_ctrl: [VolatileCell<u32>; 3], // 0x4A8
}

struct U32OrU8(u32);

impl U32OrU8 {
    pub fn write_u32(&self, word: u32) {
        unsafe {
            ::core::intrinsics::volatile_store(&self.0 as *const _ as *const u32 as *mut u32, word)
        }
    }

    pub fn write_u8(&self, byte: u8) {
        unsafe {
            ::core::intrinsics::volatile_store(&self.0 as *const _ as *const u8 as *mut u8, byte)
        }
    }
}

const KEYMGR0_SHA_REGS: *mut Registers = (KEYMGR0_BASE_ADDRESS + 0x400) as *mut Registers;

#[allow(unused)]
enum ShaTrigMask {
    Go = 0x1,
    Reset = 0x2,
    Step = 0x4,
    Stop = 0x8,
}

#[allow(unused)]
enum ShaCfgEnMask {
    BigEndian = 0x01,
    Sha1 = 0x02,

    BusError = 0x08,
    Livestream = 0x10,
    Hmac = 0x20,

    IntEnDone = 0x1_0000,
    IntMaskDone = 0x2_0000,
}

pub struct ShaEngine {
    regs: *mut Registers,
    current_mode: Cell<Option<DigestMode>>,
    client: Cell<Option<&'static Client>>,
}

impl ShaEngine {
    const unsafe fn new(regs: *mut Registers) -> ShaEngine {
        ShaEngine {
            regs: regs,
            current_mode: Cell::new(None),
            client: Cell::new(None),
        }
    }

    pub fn set_client(&self, client: &'static Client) {
        self.client.set(Some(client));
    }

    pub fn handle_interrupt(&self) {
        let regs = unsafe { &*self.regs };
        self.current_mode.get().map(|mode| {
            let mut long_arr = [0; 256 / 8];
            let buf = match mode {
                DigestMode::Sha1 => &mut long_arr[0..160 / 8],
                DigestMode::Sha256 => long_arr.as_mut(),
            };
            for (i, b) in buf.iter_mut().enumerate() {
                *b = regs.sts_h[i % regs.sts_h.len()].get();
            }
            self.client.get().map(|client| client.done(buf));
        });

        // Mark interrupt handled
        regs.itop.set(0);
    }
}

pub static mut KEYMGR0_SHA: ShaEngine = unsafe { ShaEngine::new(KEYMGR0_SHA_REGS) };

impl DigestEngine for ShaEngine {
    fn initialize(&self, mode: DigestMode) -> Result<(), DigestError> {
        let regs = unsafe { &*self.regs };

        // Compile-time check for DigestMode exhaustiveness
        match mode {
            DigestMode::Sha1 |
            DigestMode::Sha256 => (),
        };
        self.current_mode.set(Some(mode));

        regs.trig.set(ShaTrigMask::Stop as u32);

        let mut flags = ShaCfgEnMask::Livestream as u32 | ShaCfgEnMask::IntEnDone as u32;
        match mode {
            DigestMode::Sha1 => flags |= ShaCfgEnMask::Sha1 as u32,
            DigestMode::Sha256 => (),
        }
        regs.cfg_en.set(flags);

        regs.trig.set(ShaTrigMask::Go as u32);

        Ok(())
    }

    fn update(&self, data: &[u8]) -> Result<(), DigestError> {
        let regs = unsafe { &*self.regs };

        if self.current_mode.get().is_none() {
            return Err(DigestError::NotConfigured);
        }

        // We have a &[u8] but we want to write a word (32 bits) at a time, so
        // we break it up into 4-byte chunks.
        for word in data.chunks(4) {
            if word.len() < 4 {
                // If there is less than a word left, we have to write one byte
                // at a time otherwise the SHA engine will include the zero
                // padding as the length (this relies on the architecture
                // allowing byte-aligned writes, e.g. the `strb` instruction)
                for b in word.iter() {
                    regs.input_fifo.write_u8(*b)
                }
            } else {
                // `word` is a `Chunk<&u8>` of size at most 4, which we want to
                // convert into a `u32`. This little bit functional code might seem
                // gnarly, but it's not so bad:
                //
                // 1. We `map` over the `Chunk<&u8>` and convert the elements to
                //    `u32`s to make the types work out. This gives us a
                //    `Chunk<u32>`.
                // 2. We `fold` over the `Chunk<u32>`, giving it a tuple with an
                //    accumulator and the current bit-offset inside the accumulator,
                //    both initialized to 0.
                // 3. In each iteration of the `fold`, we add the current byte to
                //    the accumlator, shifting it by the current offset, and
                //    increment the offset.
                // 4. We end up with `(accm, offset <= 32)`, but we only want
                //    `accm`, so we get field `0` out of the tuple.
                let d = word.iter()
                    .map(|b| *b as u32)
                    .enumerate()
                    .fold(0, |accm, (i, byte)| accm | (byte << (i * 8)));
                regs.input_fifo.write_u32(d);
            }
        }

        Ok(())
    }

    fn finalize(&self) -> Result<(), DigestError> {
        let regs = unsafe { &*self.regs };

        match self.current_mode.get() {
            None => Err(DigestError::NotConfigured),
            Some(_) => {
                // Tell hardware we're done streaming and then wait for the hash calculation to
                // finish.
                regs.cfg_en.set(regs.cfg_en.get() | 0b1 << 16);
                println!("Setting...");
                regs.trig.set(ShaTrigMask::Stop as u32);

                Ok(())
            }
        }
    }
}

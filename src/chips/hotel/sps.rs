//! SPI Slave (SPS)

use common::take_cell::TakeCell;
use common::volatile_cell::VolatileCell;
use hil::spi_master::{ClockPhase,ClockPolarity,DataOrder};

pub trait SpsClient {
    fn rx_received(&self, buf: &'static [u8], len: usize);
}

struct FifoRegisters {
    size: VolatileCell<u32>,
    rptr: VolatileCell<u32>,
    wptr: VolatileCell<u32>,
    threshold: VolatileCell<u32>,
}

struct Registers {
    control: VolatileCell<u32>,
    dummy_word: VolatileCell<u32>,
    status01: VolatileCell<u32>,
    status23: VolatileCell<u32>,
    status45: VolatileCell<u32>,
    status67: VolatileCell<u32>,
    control01: VolatileCell<u32>,
    control23: VolatileCell<u32>,
    control45: VolatileCell<u32>,
    control67: VolatileCell<u32>,
    fifo_control: VolatileCell<u32>,
    txfifo: FifoRegisters,
    rxfifo: FifoRegisters,
    over_ride: VolatileCell<u32>,
    value: VolatileCell<u32>,
    interrupt_state: VolatileCell<u32>,
    clear_interrupt_state: VolatileCell<u32>,
    _itcr: VolatileCell<u32>,
    _itop: VolatileCell<u32>,
    interrupt_control: VolatileCell<u32>
}

const SPS_BASE_ADDRESS: *const Registers = 0x40510000 as *const Registers;
const SPS_RAM_ADDRESS: *mut [u8; 2048]   = 0x40511000 as *mut [u8; 2048];

pub struct SPS {
    registers: *const Registers,
    client: TakeCell<&'static SpsClient>
}

impl SPS {
    pub unsafe fn new() -> SPS {
        SPS { registers: SPS_BASE_ADDRESS, client: TakeCell::empty() }
    }

    pub fn configure(&self, cpha: ClockPhase, cpol: ClockPolarity,
                  data_order: DataOrder) {
        use hil::spi_master::ClockPhase::{SampleLeading, SampleTrailing};
        use hil::spi_master::ClockPolarity::{IdleLow, IdleHigh};
        use hil::spi_master::DataOrder::{MSBFirst, LSBFirst};

        let regs = unsafe { &*(self.registers) };
        // Disable all interrupts
        regs.interrupt_control.set(0);

        regs.control.set(
            // Mode (Generic: 0, Swetland: 1, EEPROM: 2)
              0
            // Clock phase
            | match cpha { SampleLeading => 0, SampleTrailing => 1} << 2
            // Clock polarity
            | match cpol { IdleLow => 0, IdleHigh => 1}  << 3 // CPOL
            // Idle level
            | 0 << 4
            // TX bit order
            | match data_order { LSBFirst => 0, MSBFirst => 1 } << 5
            // RX bit order
            | match data_order { LSBFirst => 0, MSBFirst => 1 } << 6
            );

        // Set default transfer word to 0xff (when TXFIFO is empty)
        regs.dummy_word.set(0xff);

        // Reset TX and RX fifos
        // [5,4,3]           [2,1,0]
	    // RX{DIS, EN, RST} TX{DIS, EN, RST}
        regs.fifo_control.set(0b1001);

        // Wait for fifo reset bits to self clear
        while regs.fifo_control.get() & 0b1001 != 0 {}

        let fifos: &mut [u8; 2048] = unsafe { &mut *SPS_RAM_ADDRESS };
        regs.rxfifo.rptr.set(1024);
        regs.rxfifo.wptr.set(1024);

        fifos[0..10].clone_from_slice(b"fdsabdefds");
        regs.txfifo.wptr.set(10);

        // Enable TX FIFO
        regs.fifo_control.set(0b10);

        /*panic!("TX WPTR: 0x{:x}\n\t \
                TX RPTR: 0x{:x}\n\t \
                TX SIZE: 0x{:x}\n\n\t \
                RX WPTR: 0x{:x}\n\t \
                RX RPTR: 0x{:x}\n\t \
                RX SIZE: 0x{:x}",
                regs.txfifo.wptr.get(), regs.txfifo.rptr.get(), regs.txfifo.size.get(),
                regs.rxfifo.wptr.get(), regs.rxfifo.rptr.get(), regs.rxfifo.size.get());*/
    }

    pub fn set_client(&self, client: &'static SpsClient) {
        self.client.replace(client);
    }
}


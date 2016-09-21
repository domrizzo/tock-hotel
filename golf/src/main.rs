#![crate_name = "golf"]
#![no_std]
#![no_main]
#![feature(lang_items)]

#[macro_use(static_init)]
extern crate common;
extern crate drivers;
extern crate hotel;
extern crate hil;
extern crate main;

#[macro_use]
pub mod io;

use main::{Chip, MPU, Platform};

unsafe fn load_processes() -> &'static mut [Option<main::process::Process<'static>>] {
    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
    }

    const NUM_PROCS: usize = 2;

    #[link_section = ".app_memory"]
    static mut MEMORIES: [[u8; 8192]; NUM_PROCS] = [[0; 8192]; NUM_PROCS];

    static mut processes: [Option<main::process::Process<'static>>; NUM_PROCS] = [None, None];

    let mut addr = &_sapps as *const u8;
    for i in 0..NUM_PROCS {
        // The first member of the LoadInfo header contains the total size of each process image. A
        // sentinel value of 0 (invalid because it's smaller than the header itself) is used to
        // mark the end of the list of processes.
        let total_size = *(addr as *const usize);
        if total_size == 0 {
            break;
        }

        let process = &mut processes[i];
        let memory = &mut MEMORIES[i];
        *process = Some(main::process::Process::create(addr, total_size, memory));
        // TODO: panic if loading failed?

        addr = addr.offset(total_size as isize);
    }

    if *(addr as *const usize) != 0 {
        panic!("Exceeded maximum NUM_PROCS.");
    }

    &mut processes
}

pub struct Golf {
    console: &'static drivers::console::Console<'static, hotel::uart::UART>,
    gpio: &'static drivers::gpio::GPIO<'static, hotel::gpio::Pin>,
    timer: &'static drivers::timer::TimerDriver<'static, hotel::timels::Timels>,
    digest: &'static drivers::digest::DigestDriver<'static, hotel::crypto::sha::ShaEngine>,
}

#[no_mangle]
pub unsafe fn reset_handler() {
    hotel::init();

    let timerhs = {
        use hotel::pmu::*;
        use hotel::timeus::Timeus;
        Clock::new(PeripheralClock::Bank1(PeripheralClock1::TimeUs0Timer)).enable();
        Clock::new(PeripheralClock::Bank1(PeripheralClock1::TimeLs0)).enable();
        let timer = Timeus::new(0);
        timer
    };

    timerhs.start();
    let start = timerhs.now();

    {
        use hotel::pmu::*;
        Clock::new(PeripheralClock::Bank0(PeripheralClock0::Gpio0)).enable();
        let pinmux = &mut *hotel::pinmux::PINMUX;
        pinmux.diob0.select.set(hotel::pinmux::Function::Gpio0Gpio0);

        pinmux.gpio0_gpio1.select.set(hotel::pinmux::SelectablePin::Dioa8);
        pinmux.dioa8.select.set(hotel::pinmux::Function::Gpio0Gpio1);
        pinmux.dioa8.control.set(1 << 2 | 1 << 4);

        pinmux.dioa0.select.set(hotel::pinmux::Function::Uart0Tx);
        pinmux.dioa11.control.set(1 << 2 | 1 << 4);
        pinmux.uart0_rx.select.set(hotel::pinmux::SelectablePin::Dioa11);
    }

    let console = static_init!(
        drivers::console::Console<'static, hotel::uart::UART>,
        drivers::console::Console::new(&hotel::uart::UART0,
                                       &mut drivers::console::WRITE_BUF,
                                       main::container::Container::create()),
        24);
    hotel::uart::UART0.set_client(console);
    console.initialize();

    let gpio_pins = static_init!(
        [&'static hotel::gpio::Pin; 2],
        [&hotel::gpio::PORT0.pins[0], &hotel::gpio::PORT0.pins[1]],
        8);

    let gpio = static_init!(
        drivers::gpio::GPIO<'static, hotel::gpio::Pin>,
        drivers::gpio::GPIO::new(gpio_pins),
        20);
    for pin in gpio_pins.iter() {
        pin.set_client(gpio)
    }

    let timer = static_init!(
        drivers::timer::TimerDriver<'static, hotel::timels::Timels>,
        drivers::timer::TimerDriver::new(
            &hotel::timels::Timels0, main::container::Container::create()),
        12);
    hotel::timels::Timels0.set_client(timer);

    let digest = static_init!(
        drivers::digest::DigestDriver<'static, hotel::crypto::sha::ShaEngine>,
        drivers::digest::DigestDriver::new(
                &mut hotel::crypto::sha::KEYMGR0_SHA,
                main::Container::create()),
        16);

    let platform = static_init!(Golf, Golf {
        console: console,
        gpio: gpio,
        timer: timer,
        digest: digest,
    }, 16);

    hotel::usb::USB0.init(&mut hotel::usb::OUT_DESCRIPTORS,
                          &mut hotel::usb::OUT_BUFFERS,
                          &mut hotel::usb::IN_DESCRIPTORS,
                          &mut hotel::usb::IN_BUFFERS,
                          hotel::usb::PHY::A,
                          None,
                          Some(0x0011),
                          Some(0x7788));

    let end = timerhs.now();

    println!("Hello from Rust! Initialization took {} tics.",
             end.wrapping_sub(start));

    let mut chip = hotel::chip::Hotel::new();
    chip.mpu().enable_mpu();


    main::main(platform, &mut chip, load_processes());
}

impl Platform for Golf {
    fn with_driver<F, R>(&mut self, driver_num: usize, f: F) -> R
        where F: FnOnce(Option<&main::Driver>) -> R
    {
        match driver_num {
            0 => f(Some(self.console)),
            1 => f(Some(self.gpio)),
            2 => f(Some(self.digest)),
            3 => f(Some(self.timer)),
            _ => f(None),
        }
    }
}
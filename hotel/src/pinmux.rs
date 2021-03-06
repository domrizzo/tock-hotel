use kernel::common::volatile_cell::VolatileCell;

pub struct Pin {
    pub select: VolatileCell<Function>,
    pub control: VolatileCell<u32>,
}

pub struct Peripheral {
    pub select: VolatileCell<SelectablePin>,
}

pub struct Registers {
    pub diom0: Pin,
    pub diom1: Pin,
    pub diom2: Pin,
    pub diom3: Pin,
    pub diom4: Pin,

    pub dioa0: Pin,
    pub dioa1: Pin,
    pub dioa2: Pin,
    pub dioa3: Pin,
    pub dioa4: Pin,
    pub dioa5: Pin,
    pub dioa6: Pin,
    pub dioa7: Pin,
    pub dioa8: Pin,
    pub dioa9: Pin,
    pub dioa10: Pin,
    pub dioa11: Pin,
    pub dioa12: Pin,
    pub dioa13: Pin,
    pub dioa14: Pin,

    pub diob0: Pin,
    pub diob1: Pin,
    pub diob2: Pin,
    pub diob3: Pin,
    pub diob4: Pin,
    pub diob5: Pin,
    pub diob6: Pin,
    pub diob7: Pin,

    pub resetb: Pin,
    pub vio0: Pin,
    pub vio1: Pin,

    pub gpio0_gpio0: Peripheral,
    pub gpio0_gpio1: Peripheral,
    pub gpio0_gpio2: Peripheral,
    pub gpio0_gpio3: Peripheral,
    pub gpio0_gpio4: Peripheral,
    pub gpio0_gpio5: Peripheral,
    pub gpio0_gpio6: Peripheral,
    pub gpio0_gpio7: Peripheral,
    pub gpio0_gpio8: Peripheral,
    pub gpio0_gpio9: Peripheral,
    pub gpio0_gpio10: Peripheral,
    pub gpio0_gpio11: Peripheral,
    pub gpio0_gpio12: Peripheral,
    pub gpio0_gpio13: Peripheral,
    pub gpio0_gpio14: Peripheral,
    pub gpio0_gpio15: Peripheral,

    pub gpio1_gpio0: Peripheral,
    pub gpio1_gpio1: Peripheral,
    pub gpio1_gpio2: Peripheral,
    pub gpio1_gpio3: Peripheral,
    pub gpio1_gpio4: Peripheral,
    pub gpio1_gpio5: Peripheral,
    pub gpio1_gpio6: Peripheral,
    pub gpio1_gpio7: Peripheral,
    pub gpio1_gpio8: Peripheral,
    pub gpio1_gpio9: Peripheral,
    pub gpio1_gpio10: Peripheral,
    pub gpio1_gpio11: Peripheral,
    pub gpio1_gpio12: Peripheral,
    pub gpio1_gpio13: Peripheral,
    pub gpio1_gpio14: Peripheral,
    pub gpio1_gpio15: Peripheral,

    pub i2c0_scl: Peripheral,
    pub i2c0_sda: Peripheral,
    pub i2c1_scl: Peripheral,
    pub i2c1_sda: Peripheral,
    pub i2cs0_scl: Peripheral,
    pub i2cs0_sda: Peripheral,
    pub pmu_brownout_det: Peripheral,
    pub pmu_testbus0: Peripheral,
    pub pmu_testbus1: Peripheral,
    pub pmu_testbus2: Peripheral,
    pub pmu_testbus3: Peripheral,
    pub pmu_testbus4: Peripheral,
    pub pmu_testbus5: Peripheral,
    pub pmu_testbus6: Peripheral,
    pub pmu_testbus7: Peripheral,
    pub rtc0_rtc_clk_test: Peripheral,
    pub spi1_spiclk: Peripheral,
    pub spi1_spicsb: Peripheral,
    pub spi1_spimiso: Peripheral,
    pub spi1_spimosi: Peripheral,
    pub sps0_testbus0: Peripheral,
    pub sps0_testbus1: Peripheral,
    pub sps0_testbus2: Peripheral,
    pub sps0_testbus3: Peripheral,
    pub sps0_testbus4: Peripheral,
    pub sps0_testbus5: Peripheral,
    pub sps0_testbus6: Peripheral,
    pub sps0_testbus7: Peripheral,
    pub temp0_tst_adc_clk: Peripheral,
    pub temp0_tst_adc_hi_ser: Peripheral,
    pub temp0_tst_adc_lo_ser: Peripheral,
    pub temp0_tst_adc_vld_ser: Peripheral,
    pub trng0_trng_ro_div: Peripheral,
    pub trng0_trng_ro_ref_div: Peripheral,
    pub uart0_cts: Peripheral,
    pub uart0_rts: Peripheral,
    pub uart0_rx: Peripheral,
    pub uart0_tx: Peripheral,
    pub uart1_cts: Peripheral,
    pub uart1_rts: Peripheral,
    pub uart1_rx: Peripheral,
    pub uart1_tx: Peripheral,
    pub uart2_cts: Peripheral,
    pub uart2_rts: Peripheral,
    pub uart2_rx: Peripheral,
    pub uart2_tx: Peripheral,
    pub usb0_ext_dm_pullup_en: Peripheral,
    pub usb0_ext_dp_rpu1_enb: Peripheral,
    pub usb0_ext_dp_rpu2_enb: Peripheral,
    pub usb0_ext_fs_edge_sel: Peripheral,
    pub usb0_ext_rx_dmi: Peripheral,
    pub usb0_ext_rx_dpi: Peripheral,
    pub usb0_ext_rx_rcv: Peripheral,
    pub usb0_ext_suspendb: Peripheral,
    pub usb0_ext_tx_dmo: Peripheral,
    pub usb0_ext_tx_dpo: Peripheral,
    pub usb0_ext_tx_oeb: Peripheral,
    pub volt0_tst_neg_glitch_det: Peripheral,
    pub volt0_tst_pos_glitch_det: Peripheral,
    pub xo0_testbus0: Peripheral,
    pub xo0_testbus1: Peripheral,
    pub xo0_testbus2: Peripheral,
    pub xo0_testbus3: Peripheral,
    pub xo0_testbus4: Peripheral,
    pub xo0_testbus5: Peripheral,
    pub xo0_testbus6: Peripheral,
    pub xo0_testbus7: Peripheral,
}

pub const PINMUX: *mut Registers = 0x40060000 as *mut Registers;

#[repr(u32)]
pub enum SelectablePin {
    Vio1 = 1,
    Vio0 = 2,
    Diob7 = 3,
    Diob6 = 4,
    Diob5 = 5,
    Diob4 = 6,
    Diob3 = 7,
    Diob2 = 8,
    Diob1 = 9,
    Diob0 = 10,
    Dioa14 = 11,
    Dioa13 = 12,
    Dioa12 = 13,
    Dioa11 = 14,
    Dioa10 = 15,
    Dioa9 = 16,
    Dioa8 = 17,
    Dioa7 = 18,
    Dioa6 = 19,
    Dioa5 = 20,
    Dioa4 = 21,
    Dioa3 = 22,
    Dioa2 = 23,
    Dioa1 = 24,
    Dioa0 = 25,
    Diom4 = 26,
    Diom3 = 27,
    Diom2 = 28,
    Diom1 = 29,
    Diom0 = 30,
}

#[repr(u32)]
pub enum Function {
    Default = 0,
    Gpio0Gpio0 = 1,
    Gpio0Gpio1 = 2,
    Gpio0Gpio2 = 3,
    Gpio0Gpio3 = 4,
    Gpio0Gpio4 = 5,
    Gpio0Gpio5 = 6,
    Gpio0Gpio6 = 7,
    Gpio0Gpio7 = 8,
    Gpio0Gpio8 = 9,
    Gpio0Gpio9 = 10,
    Gpio0Gpio10 = 11,
    Gpio0Gpio11 = 12,
    Gpio0Gpio12 = 13,
    Gpio0Gpio13 = 14,
    Gpio0Gpio14 = 15,
    Gpio0Gpio15 = 16,
    Gpio1Gpio0 = 17,
    Gpio1Gpio1 = 18,
    Gpio1Gpio2 = 19,
    Gpio1Gpio3 = 20,
    Gpio1Gpio4 = 21,
    Gpio1Gpio5 = 22,
    Gpio1Gpio6 = 23,
    Gpio1Gpio7 = 24,
    Gpio1Gpio8 = 25,
    Gpio1Gpio9 = 26,
    Gpio1Gpio10 = 27,
    Gpio1Gpio11 = 28,
    Gpio1Gpio12 = 29,
    Gpio1Gpio13 = 30,
    Gpio1Gpio14 = 31,
    Gpio1Gpio15 = 32,
    I2C0Scl = 33,
    I2C0Sda = 34,
    I2C1Scl = 35,
    I2C1SDA = 36,
    I2cs0Scl = 37,
    I2cs0Sda = 38,
    PmuBrownoutDet = 39,
    PmuTestbus0 = 40,
    PmuTestbus1 = 41,
    PmuTestbus2 = 42,
    PmuTestbus3 = 43,
    PmuTestbus4 = 44,
    PmuTestbus5 = 45,
    PmuTestbus6 = 46,
    PmuTestbus7 = 47,
    Rtc0RtcClkTest = 48,
    Spi1Spiclk = 49,
    Spi1Spicsb = 50,
    Spi1Spimiso = 51,
    Spi1Spimosi = 52,
    Sps0Testbus0 = 53,
    Sps0Testbus1 = 54,
    Sps0Testbus2 = 55,
    Sps0Testbus3 = 56,
    Sps0Testbus4 = 57,
    Sps0Testbus5 = 58,
    Sps0Testbus6 = 59,
    Sps0Testbus7 = 60,
    Temp0TstAdcClk = 61,
    Temp0TstAdcHiSer = 62,
    Temp0TstAdcLoSer = 63,
    Temp0TstAdcVldSer = 64,
    Trng0TrngRoDiv = 65,
    Trng0TrngRoRefDiv = 66,
    Uart0Cts = 67,
    Uart0Rts = 68,
    Uart0Rx = 69,
    Uart0Tx = 70,
    Uart1Cts = 71,
    Uart1Rts = 72,
    Uart1Rx = 73,
    Uart1Tx = 74,
    Uart2Cts = 75,
    Uart2Rts = 76,
    Uart2Rx = 77,
    Uart2Tx = 78,
    Usb0ExtDmPullupEn = 79,
    Usb0ExtDpRpu1Enb = 80,
    Usb0ExtDpRpu2Enb = 81,
    Usb0ExtFsEdgeSel = 82,
    Usb0ExtRxDmi = 83,
    Usb0ExtRxDpi = 84,
    Usb0ExtRxRcv = 85,
    Usb0ExtSuspendb = 86,
    Usb0ExtTxDmo = 87,
    Usb0ExtTxDpo = 88,
    Usb0ExtTxOeb = 89,
    Volt0TstNegGlitchDet = 90,
    Volt0TstPosGlitchDet = 91,
    Xo0Testbus0 = 92,
    Xo0Testbus1 = 93,
    Xo0Testbus2 = 94,
    Xo0Testbus3 = 95,
    Xo0Testbus4 = 96,
    Xo0Testbus5 = 97,
    Xo0testbus6 = 98,
    Xo0Testbus7 = 99,
}

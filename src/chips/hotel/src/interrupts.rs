use unhandled_interrupt;

#[no_mangle]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub static INTERRUPT_TABLE: [unsafe extern fn(); 203] =
    [ unhandled_interrupt, // CRYPTO0_BREAK_INT,
      unhandled_interrupt, // CRYPTO0_DMEM_PTRS_OVERFLOW_INT
      unhandled_interrupt, // CRYPTO0_DONE_WIPE_SECRETS_INT
      unhandled_interrupt, // CRYPTO0_DRF_PTRS_OVERFLOW_INT
      unhandled_interrupt, // CRYPTO0_HOST_CMD_DONE_INT
      unhandled_interrupt, // CRYPTO0_HOST_CMD_RECV_INT
      unhandled_interrupt, // CRYPTO0_LOOP_STACK_OVERFLOW_INT
      unhandled_interrupt, // CRYPTO0_LOOP_STACK_UNDERFLOW_INT
      unhandled_interrupt, // CRYPTO0_MOD_OPERAND_OUT_OF_RANGE_INT
      unhandled_interrupt, // CRYPTO0_PC_STACK_OVERFLOW_INT
      unhandled_interrupt, // CRYPTO0_PGM_FAULT_INT
      unhandled_interrupt, // CRYPTO0_TRAP_INT
      unhandled_interrupt, // DMA0_INTR_COMPLETE_CHAN_INT
      unhandled_interrupt, // DMA0_INTR_ERROR_CHAN_INT
      unhandled_interrupt, // DMA0_INTR_PROG_CHAN_INT
      unhandled_interrupt, // DMA0_INTR_TIMEOUT_CHAN_INT
      unhandled_interrupt, // FLASH0_EDONEINT
      unhandled_interrupt, // FLASH0_PDONEINT
      unhandled_interrupt, // GLOBALSEC_CAMO0_BREACH_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_CRYPTO0_DMEM_PARITY_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_CRYPTO0_DRF_PARITY_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_CRYPTO0_IMEM_PARITY_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_CRYPTO0_PGM_FAULT_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_CPU0_D_IF_BUS_ERR_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_CPU0_D_IF_UPDATE_WATCHDOG_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_CPU0_I_IF_BUS_ERR_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_CPU0_I_IF_UPDATE_WATCHDOG_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_CPU0_S_IF_BUS_ERR_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_CPU0_S_IF_UPDATE_WATCHDOG_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_DDMA0_IF_BUS_ERR_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_DDMA0_IF_UPDATE_WATCHDOG_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_DSPS0_IF_BUS_ERR_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_DSPS0_IF_UPDATE_WATCHDOG_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_DUSB0_IF_BUS_ERR_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_DBCTRL_DUSB0_IF_UPDATE_WATCHDOG_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_FUSE0_FUSE_DEFAULTS_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_ALERT_GROUPA_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_ALERT_GROUPB_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_ALERT_GROUPC_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_DIFF_FAIL_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_FW0_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_FW1_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_FW2_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_FW3_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_HEARTBEAT_FAIL_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_PROC_OPCODE_HASH_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_GLOBALSEC_SRAM_PARITY_SCRUB_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_KEYMGR0_AES_EXEC_CTR_MAX_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_KEYMGR0_AES_HKEY_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_KEYMGR0_CERT_LOOKUP_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_KEYMGR0_FLASH_ENTRY_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_KEYMGR0_PW_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_KEYMGR0_SHA_EXEC_CTR_MAX_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_KEYMGR0_SHA_FAULT_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_KEYMGR0_SHA_HKEY_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_PMU_BATTERY_MON_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_PMU_PMU_WDOG_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_RTC0_RTC_DEAD_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_TEMP0_MAX_TEMP_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_TEMP0_MAX_TEMP_DIFF_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_TEMP0_MIN_TEMP_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_TRNG0_OUT_OF_SPEC_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_TRNG0_TIMEOUT_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_VOLT0_VOLT_ERR_ALERT_INT
      unhandled_interrupt, // GLOBALSEC_XO0_JITTERY_TRIM_DIS_ALERT_INT
      ::gpio::gpio0_0_handler, // GPIO0_GPIO0INT
      ::gpio::gpio0_1_handler, // GPIO0_GPIO1INT
      unhandled_interrupt, // GPIO0_GPIO2INT
      unhandled_interrupt, // GPIO0_GPIO3INT
      unhandled_interrupt, // GPIO0_GPIO4INT
      unhandled_interrupt, // GPIO0_GPIO5INT
      unhandled_interrupt, // GPIO0_GPIO6INT
      unhandled_interrupt, // GPIO0_GPIO7INT
      unhandled_interrupt, // GPIO0_GPIO8INT
      unhandled_interrupt, // GPIO0_GPIO9INT
      unhandled_interrupt, // GPIO0_GPIO10INT
      unhandled_interrupt, // GPIO0_GPIO11INT
      unhandled_interrupt, // GPIO0_GPIO12INT
      unhandled_interrupt, // GPIO0_GPIO13INT
      unhandled_interrupt, // GPIO0_GPIO14INT
      unhandled_interrupt, // GPIO0_GPIO15INT
      ::gpio::gpio0_combined_handler, // GPIO0_GPIOCOMBINT
      unhandled_interrupt, // GPIO1_GPIO0INT
      unhandled_interrupt, // GPIO1_GPIO1INT
      unhandled_interrupt, // GPIO1_GPIO2INT
      unhandled_interrupt, // GPIO1_GPIO3INT
      unhandled_interrupt, // GPIO1_GPIO4INT
      unhandled_interrupt, // GPIO1_GPIO5INT
      unhandled_interrupt, // GPIO1_GPIO6INT
      unhandled_interrupt, // GPIO1_GPIO7INT
      unhandled_interrupt, // GPIO1_GPIO8INT
      unhandled_interrupt, // GPIO1_GPIO9INT
      unhandled_interrupt, // GPIO1_GPIO10INT
      unhandled_interrupt, // GPIO1_GPIO11INT
      unhandled_interrupt, // GPIO1_GPIO12INT
      unhandled_interrupt, // GPIO1_GPIO13INT
      unhandled_interrupt, // GPIO1_GPIO14INT
      unhandled_interrupt, // GPIO1_GPIO15INT
      unhandled_interrupt, // GPIO1_GPIOCOMBINT
      unhandled_interrupt, // I2C0_I2CINT
      unhandled_interrupt, // I2C1_I2CINT
      unhandled_interrupt, // I2CS0_INTR_READ_BEGIN_INT
      unhandled_interrupt, // I2CS0_INTR_READ_COMPLETE_INT
      unhandled_interrupt, // I2CS0_INTR_WRITE_COMPLETE_INT
      unhandled_interrupt, // KEYMGR0_AES_DONE_CIPHER_INT
      unhandled_interrupt, // KEYMGR0_AES_DONE_KEYEXPANSION_INT
      unhandled_interrupt, // KEYMGR0_AES_DONE_WIPE_SECRETS_INT
      unhandled_interrupt, // KEYMGR0_AES_RFIFO_OVERFLOW_INT
      unhandled_interrupt, // KEYMGR0_AES_RFIFO_UNDERFLOW_INT
      unhandled_interrupt, // KEYMGR0_AES_WFIFO_OVERFLOW_INT
      unhandled_interrupt, // KEYMGR0_DSHA_INT
      unhandled_interrupt, // KEYMGR0_SHA_WFIFO_FULL_INT
      unhandled_interrupt, // PMU_INTR_WAKEUP_INT
      unhandled_interrupt, // RBOX0_INTR_AC_PRESENT_FED_INT
      unhandled_interrupt, // RBOX0_INTR_AC_PRESENT_RED_INT
      unhandled_interrupt, // RBOX0_INTR_BUTTON_COMBO0_RDY_INT
      unhandled_interrupt, // RBOX0_INTR_BUTTON_COMBO1_RDY_INT
      unhandled_interrupt, // RBOX0_INTR_BUTTON_COMBO2_RDY_INT
      unhandled_interrupt, // RBOX0_INTR_EC_RST_FED_INT
      unhandled_interrupt, // RBOX0_INTR_EC_RST_RED_INT
      unhandled_interrupt, // RBOX0_INTR_KEY0_IN_FED_INT
      unhandled_interrupt, // RBOX0_INTR_KEY0_IN_RED_INT
      unhandled_interrupt, // RBOX0_INTR_KEY1_IN_FED_INT
      unhandled_interrupt, // RBOX0_INTR_KEY1_IN_RED_INT
      unhandled_interrupt, // RBOX0_INTR_PWRB_IN_FED_INT
      unhandled_interrupt, // RBOX0_INTR_PWRB_IN_RED_INT
      unhandled_interrupt, // RDD0_INTR_DEBUG_STATE_DETECTED_INT
      unhandled_interrupt, // SPI0_SPITXINT
      unhandled_interrupt, // SPI1_SPITXINT
      unhandled_interrupt, // SPS0_CS_ASSERT_INTR
      unhandled_interrupt, // SPS0_CS_DEASSERT_INTR
      unhandled_interrupt, // SPS0_INTR_CMD_ADDR_FIFO_NOT_EMPTY_INT
      unhandled_interrupt, // SPS0_INTR_CMD_ADDR_FIFO_OVFL_INT
      unhandled_interrupt, // SPS0_INTR_CMD_MEM_OVFL_INT
      unhandled_interrupt, // SPS0_INTR_RAM_PAGE0_LVL_INT
      unhandled_interrupt, // SPS0_INTR_RAM_PAGE1_LVL_INT
      unhandled_interrupt, // SPS0_INTR_RAM_PAGE2_LVL_INT
      unhandled_interrupt, // SPS0_INTR_RAM_PAGE3_LVL_INT
      unhandled_interrupt, // SPS0_RXFIFO_LVL_INTR
      unhandled_interrupt, // SPS0_RXFIFO_OVERFLOW_INTR
      unhandled_interrupt, // SPS0_SPSCTRLINT0
      unhandled_interrupt, // SPS0_SPSCTRLINT1
      unhandled_interrupt, // SPS0_SPSCTRLINT2
      unhandled_interrupt, // SPS0_SPSCTRLINT3
      unhandled_interrupt, // SPS0_SPSCTRLINT4
      unhandled_interrupt, // SPS0_SPSCTRLINT5
      unhandled_interrupt, // SPS0_SPSCTRLINT6
      unhandled_interrupt, // SPS0_SPSCTRLINT7
      unhandled_interrupt, // SPS0_TXFIFO_EMPTY_INTR
      unhandled_interrupt, // SPS0_TXFIFO_FULL_INTR
      unhandled_interrupt, // SPS0_TXFIFO_LVL_INTR
      unhandled_interrupt, // TEMP0_ADC_ICLKDV_INT
      unhandled_interrupt, // TEMP0_COMP_OVERFLOW_INT
      unhandled_interrupt, // TIMEHS0_TIMINT1
      unhandled_interrupt, // TIMEHS0_TIMINT2
      unhandled_interrupt, // TIMEHS0_TIMINTC
      unhandled_interrupt, // TIMEHS1_TIMINT1
      unhandled_interrupt, // TIMEHS1_TIMINT2
      unhandled_interrupt, // TIMEHS1_TIMINTC
      unhandled_interrupt, // TIMELS0_TIMINT0
      unhandled_interrupt, // TIMELS0_TIMINT1
      unhandled_interrupt, // TIMEUS0_INTR_MAX_COUNT_HIT0_INT
      unhandled_interrupt, // TIMEUS0_INTR_MAX_COUNT_HIT1_INT
      unhandled_interrupt, // TIMEUS0_INTR_MAX_COUNT_HIT2_INT
      unhandled_interrupt, // TIMEUS0_INTR_MAX_COUNT_HIT3_INT
      unhandled_interrupt, // TIMEUS0_INTR_PROG_COUNT_HIT0_INT
      unhandled_interrupt, // TIMEUS0_INTR_PROG_COUNT_HIT1_INT
      unhandled_interrupt, // TIMEUS0_INTR_PROG_COUNT_HIT2_INT
      unhandled_interrupt, // TIMEUS0_INTR_PROG_COUNT_HIT3_INT
      unhandled_interrupt, // TRNG0_INTR_BUFFER_FULL_INT
      unhandled_interrupt, // TRNG0_INTR_ONE_SHOT_DONE_INT
      unhandled_interrupt, // TRNG0_INTR_READ_EMPTY_INT
      unhandled_interrupt, // UART0_RXBINT
      unhandled_interrupt, // UART0_RXFINT
      ::uart::uart0_rx_handler, // UART0_RXINT
      unhandled_interrupt, // UART0_RXOVINT
      unhandled_interrupt, // UART0_RXTOINT
      ::uart::uart0_tx_handler, // UART0_TXINT
      unhandled_interrupt, // UART0_TXOVINT
      unhandled_interrupt, // UART1_RXBINT
      ::uart::uart1_rx_handler, // UART1_RXINT
      unhandled_interrupt, // UART1_RXINT
      unhandled_interrupt, // UART1_RXOVINT
      unhandled_interrupt, // UART1_RXTOINT
      ::uart::uart1_tx_handler, // UART1_TXINT
      unhandled_interrupt, // UART1_TXOVINT
      unhandled_interrupt, // UART2_RXBINT
      unhandled_interrupt, // UART2_RXFINT
      ::uart::uart2_rx_handler, // UART2_RXINT
      unhandled_interrupt, // UART2_RXOVINT
      unhandled_interrupt, // UART2_RXTOINT
      ::uart::uart2_tx_handler, // UART2_TXINT
      unhandled_interrupt, // UART2_TXOVINT
      unhandled_interrupt, // USB0_USBINTR
      unhandled_interrupt, // WATCHDOG0_WDOGINT
      unhandled_interrupt, // XO0_CLK_JTR_NOP_SEEN_INT
      unhandled_interrupt, // XO0_CLK_JTR_SW_TRIM_DONE_INT
      unhandled_interrupt, // XO0_CLK_TIMER_NOP_SEEN_INT
      unhandled_interrupt, // XO0_CLK_TIMER_SW_TRIM_DONE_INT
      unhandled_interrupt, // XO0_FAST_CALIB_OVERFLOW_INT
      unhandled_interrupt, // XO0_FAST_CALIB_UNDERRUN_INT
      unhandled_interrupt, // XO0_SLOW_CALIB_OVERFLOW_INT
      unhandled_interrupt, // XO0_SLOW_CALIB_UNDERRUN_INT
    ];

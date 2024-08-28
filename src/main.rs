
//! Prints "Hello, world!" on the host console using semihosting
#![no_std]
#![no_main]

use imxrt_rt::entry;
use imxrt_ral as pac;
use panic_halt as _;
use rtt_target::rprintln;
use cortex_m as _;
use imxrt1060evk_fcb as _;

const LED_OFFSET: u32 = 1 << 11;

// Register addresses come from the reference manual.
const IOMUXC_MUX_CTL_PAD_GPIO_11: *mut u32 = 0x401F_8090 as _;
const GPIO1_GDIR: *mut u32 = (0x401B_8000 + 0x04) as _;
const GPIO1_DR_SET: *mut u32 = (0x401B_8000 + 0x84) as _;

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();
     unsafe {
        // Configure the pad named "GPIO_11" as a GPIO pin
        // (as opposed to a UART TX pin, for example).
        IOMUXC_MUX_CTL_PAD_GPIO_11.write_volatile(5);

        // Set the GPIO as an output with a RMW operation.
        let mut gpio1_gdir = GPIO1_GDIR.read_volatile();
        gpio1_gdir |= LED_OFFSET;
        GPIO1_GDIR.write_volatile(gpio1_gdir);

        // Turn on the LED.
        GPIO1_DR_SET.write_volatile(LED_OFFSET);
    }
    
    // Initialize FlexSPI

    loop {
        rprintln!("Hello, world!");
    }
}
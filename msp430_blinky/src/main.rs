#![no_main]
#![no_std]

use embedded_hal::digital::v2::*;
use msp430_rt::entry;
use msp430fr2x5x_hal::{gpio::Batch, pmm::Pmm, watchdog::Wdt};
// use panic_msp430 as _;
use msp430fr2355::Peripherals;
mod panic;

// Red onboard LED should blink at a steady period.
// Green onboard LED should go on when P2.3 button is pressed
#[entry]
unsafe fn main() -> ! {
    // let periph = Peripherals::take().unwrap();   // TODO: only possible with critical section feature, but there are build errors
    let periph = Peripherals::conjure();

    let _wdt = Wdt::constrain(periph.WDT_A);

    let pmm = Pmm::new(periph.PMM);
    let p1 = Batch::new(periph.P1).split(&pmm);
    let p2 = Batch::new(periph.P2)
        .config_pin3(|p| p.pullup())
        .split(&pmm);
    let p6 = Batch::new(periph.P6)
        .config_pin6(|p| p.to_output())
        .split(&pmm);

    let mut p1_0 = p1.pin0.to_output();
    let p2_3 = p2.pin3;
    let mut p6_6 = p6.pin6;

    loop {
        p1_0.toggle().ok();

        for _ in 0..5000 {
            if p2_3.is_high().unwrap() {
                p6_6.set_low().ok();
            } else {
                p6_6.set_high().ok();
            }
        }
    }
}

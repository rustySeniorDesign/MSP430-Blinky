
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Disable interrupts to prevent further damage.
    msp430::interrupt::disable();
    loop {
        // Prevent optimizations that can remove this loop.
        msp430::asm::barrier();
    }
}

use core::panic::PanicInfo;

use crate::sbi;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        crate::println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        crate::println!("Panicked: {}", info.message().unwrap());
    }
    sbi::shutdown()
}

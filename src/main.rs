// cargo run   -> ビルド、BOOTX64.EFI へのコピー、QEMU起動を一括実行
// cargo build -> x86_64-unknown-uefi ターゲット向けにビルド

#![no_std]
#![no_main]

#[no_mangle]
fn efi_main() {
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
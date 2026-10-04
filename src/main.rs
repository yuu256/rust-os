//cargo build --target x86_64-unknown-uefi
//cp target/x86_64-unknown-uefi/debug/rust-os.efi mnt/EFI/BOOT/BOOTX64.EFI
//qemu-system-x86_64 -bios third_party/ovmf/RELEASEX64_OVMF.fd -drive format=raw,file=fat:rw:mnt

#![no_std]
#![no_main]

#![no_mangle]
fn efi_main() {
    loop {}
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
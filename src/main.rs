fn main() {
    //cargo build --target x86_64-unknown-uefi
    //cp target/x86_64-unknown-uefi/debug/rust-os.efi mnt/EFI/BOOT/BOOTX64.EFI
    //qemu-system-x86_64 -bios third_party/ovmf/RELEASEX64_OVMF.fd -drive format=raw,file=fat:rw:mnt
    println!("Hello, world!");
    loop{}
}

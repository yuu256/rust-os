#!/usr/bin/env bash
set -euo pipefail

EFI_BIN="${1:-target/x86_64-unknown-uefi/debug/rust-os.efi}"
shift || true

mkdir -p mnt/EFI/BOOT
cp "${EFI_BIN}" mnt/EFI/BOOT/BOOTX64.EFI

qemu-system-x86_64 \
    -bios third_party/ovmf/RELEASEX64_OVMF.fd \
    -drive format=raw,file=fat:rw:mnt \
    "$@"

# Sano OS

This is simple operating system written in Rust based on [this project](https://os.phil-opp.com)

## Building bootable image
```
cargo bootimage
```
This creates a bootable binary image file in `./target/x86_64-sano_os/debug/bootimage-sano_os.bin`

## Running using QEMU
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-sano_os/debug/bootimage-sano_os.bin
```

## Building and running at once
```
  [target.'cfg(target_os = "none")']
  runner = "bootimage runner"
```

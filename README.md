# Sano OS

This is simple operating system written in Rust based on [this project](https://os.phil-opp.com)

## Build bootable image and run
```
  cargo run
```

## Manually build image and run
```
cargo bootimage
```
This creates a bootable binary image file in `./target/x86_64-sano_os/debug/bootimage-sano_os.bin`

Then, manually runs built bootable image using QEMU
```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-sano_os/debug/bootimage-sano_os.bin
```

# led-controller
Control color of led strip via smartphone

# How to run

### ESP-IDF tutorial
https://docs.esp-rs.org/std-training/01_intro.html

### How to run

``` Bash
cargo build --release
```

``` Bash
espflash flash target/xtensa-esp32s3-espidf/debug/led-controller --monitor
```
## openmv_h7_bsp

A board support package for experimenting with rust on 
the [OpenMV H7](https://openmv.io/products/openmv-cam-h7)
open hardware machine vision board.


## Embedded Examples
The examples are designed to be used with a debug probe that supports J-Link / RTT.
We provide a couple different ways to run these:
- With the Segger tools (this is the default configuration for examples)
- With [probe-run](https://crates.io/crates/probe-run) (This may not yet work.)

#### With probe-run installed
- Simply run the example (see below) with a JLink debug probe attached to your PX4FLOW
- If you have problems, edit [config](.cargo/config) to ensure that the probe-run runner is selected

#### With segger tools installed 
- Edit [config](.cargo/config) to select the `segger.gdb` runner
- In one shell run: `./start_gdb_server_jlink.sh`
- In another shell run: `JLinkRTTClient`
- Then run your choice of examples

### Running examples

```shell script
cargo run  --example blinky 
```

```shell script
cargo run  --example play --features  rttdebug,mt9v034
```

## Status

This is work-in-progress

- [x] Interoperates with the default Micropython bootloader. 
You can reinstall the micropython firmware easily via USB from the OpenMV IDE.
- [x] Debug build runs on board
- [x] Example with LED blinky
- [ ] Support for running examples with [probe-run](https://crates.io/crates/probe-run) (WIP)
- [ ] Camera reading example
- [ ] SDIO card support

## Clocks
- 12 MHz high speed external (HSE) clock crystal
- LSE TBD


## Notes on buses
###  I2C Buses
Format: `(SCL, SDA)`
- (PB8, PB9) I2C1 is used for configuring camera sensor
- Other i2c TBD

### SPI Buses
Format:  `(SCK, MISO, MOSI)` 
- TBD

### GPIO 
- TBD

### USB OTG
- TBD

### LEDs
- LEDs r,g,b = PC0, PC1, PC2 on OpenMV H7
- IR led on PE2

### UARTs / USARTs
Format: `(RX, TX)` , `(CTS, RTS)`
- TBD

### DCMI (Parallel Camera Data Interface)
- Control pins:  PA6 (DCMI_PIXCK), PA4 (DCMI_HSYNC), PB7 (DCMI_VSYNC)
- Data Pins D0-D7: PC6, PC7, PE0, PE1, PE4, PB6, PE5, PE6
- External clock provided to camera on PA8.  This is required to enable the camera. 

## External Accessories Support
- TBD

## Resources
- [OpenMV H7 base schematic](https://github.com/openmv/openmv-boards/raw/master/openmv4/base/base.pdf) 


## License

BSD-3-Clause, see `LICENSE` file. 

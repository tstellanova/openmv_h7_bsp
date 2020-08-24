## openmv_h7_bsp

A board support package for experimenting with rust on 
the [OpenMV H7](https://openmv.io/products/openmv-cam-h7)
open hardware machine vision board.


For installation and debugging use either 
- RTT / Segger J-Link
- openocd (built with stm32h743 support) or the 
[daily build of the Black Magic Probe firmware](https://github.com/blacksphere/blackmagic/wiki/Upgrading-Firmware)
(which also requires recently introduced stm32h743 support).


## Examples

The examples are currently designed to be used with J-Link / RTT.
In the future as tools such as probe-rs solidify, we may switch to that toolset


- In one shell run: `./start_gdb_server_jlink.sh`
- In another shell run: `JLinkRTTClient`
- Then run your choice of examples:

```shell script
cargo run  --example blinky 
```

```shell script
cargo run  --example play --features  rttdebug,mt9v034
```

## Status

This is  work-in-progress

- [x] Interoperates with the default Micropython bootloader
- [x] Debug build runs on board
- [x] Example with LED blinky
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


## External Accessories Support
- TBD

## Resources
- [OpenMV H7 base schematic](https://github.com/openmv/openmv-boards/raw/master/openmv4/base/base.pdf) 


## License

BSD-3-Clause, see `LICENSE` file. 

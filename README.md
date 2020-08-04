## openmv_h7_bsp

A board support package for experimenting with rust on 
the [OpenMV H7](https://openmv.io/products/openmv-cam-h7)
open hardware machine vision board.


For installation and debugging use either 
- RTT / Segger J-Link
- openocd (built with stm32h743 support) or the 
[daily build of the Black Magic Probe firmware](https://github.com/blacksphere/blackmagic/wiki/Upgrading-Firmware)
(which also requires recently introduced stm32h743 support).



## Status

This is  work-in-progress

- [ ] Does not overwrite the default Micropython bootloader
- [ ] Launched by the Micropython bootloader
- [ ] Debug build runs on board


## Clocks
- TBD


## Notes on buses
###  I2C Buses
Format: `(SCL, SDA)`
- TBD

### SPI Buses
Format:  `(SCK, MISO, MOSI)` 
- TBD

### GPIO 
- TBD

### USB OTG
- TBD


### LEDs
- TBD: 3 colors, plus separate IR


### UARTs / USARTs
Format: `(RX, TX)` , `(CTS, RTS)`

- TBD


## External Accessories Support
- TBD

## Resources
- [OpenMV H7 base schematic](https://github.com/openmv/openmv-boards/raw/master/openmv4/base/base.pdf) 


## License

BSD-3-Clause, see `LICENSE` file. 

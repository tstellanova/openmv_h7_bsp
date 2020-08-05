use p_hal::{prelude::*, stm32};
use stm32h7xx_hal as p_hal;
use p_hal::stm32 as pac;

use ehal::blocking::delay::{DelayMs, DelayUs};
use ehal::digital::v2::OutputPin;
use ehal::digital::v2::ToggleableOutputPin;
use embedded_hal as ehal;

// use crate::port_types::{DbgUartPortType, Gps1PortType, HalGpioError, HalI2cError, HalSpiError};

use p_hal::gpio::{Output, PushPull};
use p_hal::pwr::VoltageScale;
use p_hal::rcc::PllConfigStrategy;
use stm32h7xx_hal::gpio::Speed;
// use p_hal::serial::config::{Parity, StopBits, WordLength};

/// Main convenience function to initialize the system clock for highest performance,
/// provide peripherals preconfigured.
pub fn setup() -> (
    RgbLeds,
    InfraredLed,
    DelaySource,
    DcmiCtrlPins,
    DcmiDataPins,
    SdioCtrlPins,
    SdioDataPins,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    // --- Clock configuration
    // Set up the system clock
    // For OpenMV H7 we know we have:
    // - 12 MHz xtal HSE
    // - SYSCLK of 480 MHz (processor max)
    // - HCLK of SYSCLK/2 (240 MHz)
    // - (PCLK1, PCLK2, PCLK3, PCLK4) is HCLK/2 (120 MHz)
    // - PLL1P = PLL1_VCO/2  = 960 MHz / 2   = 480 MHz
    // - PLL1Q = PLL1_VCO/4  = 960 MHz / 4   = 240 MHz
    // - PLL1R = PLL1_VCO/8  = 960 MHz / 8   = 120 MHz
    const LE_SYSCLK: u32 = 480;
    const LE_HCLK: u32 = LE_SYSCLK / 2;
    const LE_PCLK: u32 = LE_HCLK / 2;
    let rcc = dp
        .RCC
        .constrain()
        .use_hse(12.mhz()) // OpenMV H7 has 12 MHz xtal HSE
        .sysclk(LE_SYSCLK.mhz())
        .hclk(LE_HCLK.mhz())
        .pll1_p_ck(480.mhz())
        .pll1_q_ck(240.mhz())
        .pll1_r_ck(120.mhz())
        .pll1_strategy(PllConfigStrategy::Iterative)
        .pclk1(LE_PCLK.mhz())
        .pclk2(LE_PCLK.mhz())
        .pclk3(LE_PCLK.mhz())
        .pclk4(LE_PCLK.mhz());

    let pwr = dp.PWR.constrain();
    let _vos = pwr.freeze();
    //vos defaults to Scale1 but needs to upgrade to Scale0 to boost to 480 MHz
    let vos = VoltageScale::Scale0; //may force higher? or just allow asserts to pass?

    //TODO need to write? : self.rb.d3cr.write(|w| unsafe { w.vos().bits(0b11) });
    // see "VOS0 activation/deactivation sequence" in RM0433

    let mut ccdr = rcc.freeze(vos, &dp.SYSCFG);
    let clocks = ccdr.clocks;
    // --- Clock configuration

    let delay_source = p_hal::delay::Delay::new(cp.SYST, clocks);

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let gpiof = dp.GPIOF.split(ccdr.peripheral.GPIOF);
    let gpiog = dp.GPIOG.split(ccdr.peripheral.GPIOG);
    let gpioh = dp.GPIOH.split(ccdr.peripheral.GPIOH);
    let gpioi = dp.GPIOI.split(ccdr.peripheral.GPIOI);

    let led_red = gpioc.pc0.into_push_pull_output();
    let led_green = gpioc.pc1.into_push_pull_output();
    let led_blue = gpioc.pc2.into_push_pull_output();
    let led_infrared = gpioe.pe2.into_push_pull_output();

    // DCMI control pins
    let dcmi_ctrl_pins = {
        let pixck = gpioa
            .pa6 // DCMI_PIXCK
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh); //s/b 100 MHz Pullup

        let hsync = gpioa
            .pa4 // DCMI_HSYNC
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh); // s/b 100 MHz Pullup

        let vsync = gpiob
            .pb7 // DCMI_VSYNC
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh); //s/b 100 MHz Pullup

        (pixck, hsync, vsync)
    };

    // DCMI digital camera interface pins (AF13)
    // this board supports 8 parallel lines D0-D7
    let dcmi_data_pins = (
        gpioc
            .pc6
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh), // DCMI_D0
        gpioc
            .pc7
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh), // DCMI_D1
        gpioe
            .pe0
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh), // DCMI_D2
        gpioe
            .pe1
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh), // DCMI_D3
        gpioe
            .pe4
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh), // DCMI_D4
        gpiob
            .pb6
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh), // DCMI_D5
        gpioe
            .pe5
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh), // DCMI_D6
        gpioe
            .pe6
            .into_pull_up_input()
            .into_alternate_af13()
            .internal_pull_up(true)
            .set_speed(Speed::VeryHigh), // DCMI_D7
    );

    // SDMCC1 pins
    let sdio_data_pins = (
        gpioc.pc8.into_alternate_af12(), // D0
        gpioc.pc9.into_alternate_af12(),
        gpioc.pc10.into_alternate_af12(),
        gpioc.pc11.into_alternate_af12(), // D3
        );

    let sdio_ctrl_pins = (
        gpiod.pd0.into_alternate_af12(), // SD_CD
        gpioc.pc12.into_alternate_af12(), // SDMMC1_CK
        gpiod.pd2.into_alternate_af12(), // SDMMC1_CMD
        );

    // enable SDIO peripheral
    dp.SDMMC1.dctrl.modify(|_r, w| w.sdioen().set_bit());

    // TODO set clock
    // dp.SDMMC1.clkcr.write(|w| unsafe { w.clkdiv().bits(118) });

    // power on sdmmc1
    dp.SDMMC1.power.write(|w| unsafe { w.pwrctrl().bits(0b11) });

    (
        (led_red, led_green, led_blue),
        led_infrared,
        delay_source,
        dcmi_ctrl_pins,
        dcmi_data_pins,
        sdio_ctrl_pins,
        sdio_data_pins
    )
}

pub type InfraredLed = p_hal::gpio::gpioe::PE2<Output<PushPull>>;
pub type RgbLeds = (
    p_hal::gpio::gpioc::PC0<Output<PushPull>>,
    p_hal::gpio::gpioc::PC1<Output<PushPull>>,
    p_hal::gpio::gpioc::PC2<Output<PushPull>>,
);
pub type DelaySource = p_hal::delay::Delay;

/// The DCMI (camera interface) has
/// - a parallel data interface from 8 to 14 data lines,
/// - a pixel clock line DCMI_PIXCLK (rising / falling edge configuration),
/// - horizontal synchronization line, DCMI_HSYNC,
/// - vertical synchronization line,  DCMI_VSYNC, with a programmable polarity.
pub type DcmiCtrlPins = (
    p_hal::gpio::gpioa::PA6<DcmiControlPin>, // DCMI_PIXCK
    p_hal::gpio::gpioa::PA4<DcmiControlPin>, // DCMI_HSYNC
    p_hal::gpio::gpiob::PB7<DcmiControlPin>, // DCMI_VSYNC
);
pub type DcmiControlPin = p_hal::gpio::Alternate<p_hal::gpio::AF13>;
pub type DcmiParallelDataPin = p_hal::gpio::Alternate<p_hal::gpio::AF13>;

/// Parallel image data lines for DCMI:
/// for the PX4FLOW, only 8 are connected to the image sensor
pub type DcmiDataPins = (
    p_hal::gpio::gpioc::PC6<DcmiParallelDataPin>, // D0
    p_hal::gpio::gpioc::PC7<DcmiParallelDataPin>, // D1
    p_hal::gpio::gpioe::PE0<DcmiParallelDataPin>, // D2
    p_hal::gpio::gpioe::PE1<DcmiParallelDataPin>, // D3
    p_hal::gpio::gpioe::PE4<DcmiParallelDataPin>, // D4
    p_hal::gpio::gpiob::PB6<DcmiParallelDataPin>, // D5
    p_hal::gpio::gpioe::PE5<DcmiParallelDataPin>, // D6
    p_hal::gpio::gpioe::PE6<DcmiParallelDataPin>, // D7
);


pub type SdioAfPin = p_hal::gpio::Alternate<p_hal::gpio::AF12>;

/// Pins for SDIO
pub type SdioDataPins = (
    p_hal::gpio::gpioc::PC8<SdioAfPin>, // PC8 // SDMMC1_D0 // SDIO_D0
    p_hal::gpio::gpioc::PC9<SdioAfPin>, // PC9 // SDIO_D1
    p_hal::gpio::gpioc::PC10<SdioAfPin>, // PC10 // SDIO_D2
    p_hal::gpio::gpioc::PC11<SdioAfPin>, // PC11 // SDIO_D3
);

pub type SdioCtrlPins =  (
    p_hal::gpio::gpiod::PD0<SdioAfPin>, // PD0 // SD_CD // TODO verify AF
    p_hal::gpio::gpioc::PC12<SdioAfPin>, // PC12 // SDMMC1_CK // SDIO_CLK
    p_hal::gpio::gpiod::PD2<SdioAfPin>, // PD2 // SDMMC1_CMD // SDIO_CMD
);
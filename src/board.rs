use crate::peripherals::*;
use embedded_hal::blocking::delay::DelayMs;

// use mt9v034_i2c::{Binning, Mt9v034, ParamContext};

use core::sync::atomic::{AtomicPtr, Ordering};
use cortex_m::singleton;

// use crate::dcmi::DcmiWrapper;
#[cfg(feature = "rttdebug")]
use panic_rtt_core::rprintln;

/// The main Board support type:
/// This contains both pre-initialized drivers for
/// onboard devices as well as bus ports for external ports peripherals.
pub struct Board<'a> {
    pub activity_led: LedOutputActivity,
    pub comms_led: LedOutputComm,
    pub error_led: LedOutputError,

    pub delay_source: DelaySource,
    pub external_i2c1: I2c1BusManager,
    pub camera_config: Option<CameraConfigType<'a>>,
    pub gyro: Option<GyroType>,
    pub eeprom: Option<EepromType<'a>>,
    pub dcmi_wrap: Option<DcmiWrapper>,
    pub usart2: Usart2Port,
    pub usart3: Usart3Port,
    pub uart4: Uart4Port,
}
/*
Copyright (c) 2020 Todd Stellanova
LICENSE: BSD3 (see LICENSE file)
*/

use crate::peripherals::*;
use embedded_hal::blocking::delay::DelayMs;

use core::sync::atomic::{AtomicPtr, Ordering};
use cortex_m::singleton;

use crate::dcmi::{DcmiWrapper, SQ_DIM_120};

#[cfg(feature = "rttdebug")]
use panic_rtt_core::rprintln;

#[cfg(feature = "mt9v034")]
use mt9v034_i2c::{BinningFactor, Mt9v034, ParamContext};

/// The main Board support type:
/// This contains both pre-initialized drivers for
/// onboard devices as well as bus ports for external ports peripherals.
pub struct Board<'a> {
    pub rgb_leds: RgbLeds,
    pub ir_led: InfraredLed,

    pub delay_source: DelaySource,

    pub dcmi_wrap: Option<DcmiWrapper<'a>>,

    #[cfg(feature = "mt9v034")]
    pub mt9v034_config: Option<Mt9v034Configurator<'a>>,
}

impl Default for Board<'_> {
    fn default() -> Self {
        #[cfg(feature = "rttdebug")]
        rprintln!("new board");

        let (
            rgb_leds,
            ir_led,
            mut delay_source,
            i2c1_port,
            dcmi_ctrl_pins,
            dcmi_data_pins,
            sdio_ctrl_pins,
            sdio_data_pins,
        ) = setup_peripherals();

        //We are safe to forget the DCMI pins after configuration
        core::mem::forget(dcmi_ctrl_pins);
        core::mem::forget(dcmi_data_pins);

        // Since any number of devices could sit on i2c1, we should treat it as a shared bus.
        let i2c1_bus_mgr: &'static mut I2c1BusManager =
            singleton!(:I2c1BusManager =
                shared_bus::CortexMBusManager::new(i2c1_port)
            )
            .unwrap();

        // // option A: select Context A with row and column bin 4 (188x120)
        // let mut dcmi_wrap = DcmiWrapper::default(dcmi);
        // // option B: select Context B with square-120 images
        // // let mut dcmi_wrap = DcmiWrapper::new(dcmi, SQ_DIM_120, SQ_DIM_120, 8);
        //
        // dcmi_wrap.setup(&dma2);

        #[cfg(feature = "mt9v034")]
        let mt9v034_config = {
            // Default i2c address is the same as for px4flow board
            let base_i2c_address = mt9v034_i2c::PX4FLOW_CAM_ADDRESS;

            let mut cam_config =
                Mt9v034::new(i2c1_bus_mgr.acquire(), base_i2c_address);

            // configure image sensor with two distinct contexts:
            // - Context A: 480x480 window, binning 4 -> 120x120 output images (square-120)
            // - Context B: 752x480 window, binning 4 -> 188x120 output images
            const BINNING_A: BinningFactor = BinningFactor::Four;
            const BINNING_B: BinningFactor = BinningFactor::Four;
            const WINDOW_W_A: u16 = 480;
            const WINDOW_H_A: u16 = 480;
            const WINDOW_W_B: u16 = 752;
            const WINDOW_H_B: u16 = 480;

            cam_config
                .setup_with_dimensions(
                    WINDOW_W_A,
                    WINDOW_H_A,
                    BINNING_A,
                    BINNING_A,
                    WINDOW_W_B,
                    WINDOW_H_B,
                    BINNING_B,
                    BINNING_B,
                    ParamContext::ContextB,
                )
                .expect("Could not configure MT9V034");

            Some(cam_config)
        };

        // Note that we do not call dcmi_wrap.enable_capture() here --
        // instead we allow the board user to do that if desired.

        Self {
            rgb_leds,
            delay_source,
            ir_led,

            #[cfg(feature = "mt9v034")]
            mt9v034_config,
            i2c1_bus: (),
        }
    }
}

pub type BusManager<Port> = shared_bus::proxy::BusManager<
    cortex_m::interrupt::Mutex<core::cell::RefCell<Port>>,
    Port,
>;

pub type BusProxy<'a, Port> = shared_bus::proxy::BusProxy<
    'a,
    cortex_m::interrupt::Mutex<core::cell::RefCell<Port>>,
    Port,
>;

pub type I2c1BusManager = BusManager<I2c1Port>;
pub type I2c1BusProxy<'a> = BusProxy<'a, I2c1Port>;

/// Concrete type for camera configuration driver
#[cfg(feature = "mt9v034")]
pub type Mt9v034Configurator<'a> = Mt9v034<I2c1BusProxy<'a>>;

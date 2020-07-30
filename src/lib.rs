//! This is a platform agnostic Rust driver for the Ltr559 ambient light
//! sensors using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read the measurement in lux. See: [`get_lux()`].
//! - Read the measurement in raw. See: [`get_als_raw_data()`]
//! - Read the conversion status. See: [`get_status()`].
//! - Read PS Data. See: [`get_ps_data()`].
//! - Get the manufacturer ID. See: [`get_manufacturer_id()`].
//! - Get the part ID. See: [`get_part_id()`].
//! - Set ALS Enable, Gain and SW Reset. See: [`set_als_contr()`].
//! - Set PS Mode and Saturation. See: [`set_ps_contr()`].
//! - Set PS LED Pulse, DutyCycle and PeakCurrent. See: [`set_ps_led()`].
//! - Set Interrupt Persist. See: [`set_interrupt_persist()`].
//! - Set ALS Meas Rate. See: [`set_als_meas_rate()`].
//! - Set ALS Low Limit. See: [`set_als_low_limit_raw()`].
//! - Set ALS High Limit. See: [`set_als_high_limit_raw()`].
//! - Set PS Low Limit. See: [`set_ps_low_limit_raw()`].
//! - Set PS High Limit. See: [`set_ps_high_limit_raw()`].
//! - Set PS Meas Rate. See: [`set_ps_meas_rate()`].
//! - Set PS Offset. See: [`set_ps_offset()`].
//! - Set PS N Pulses. See: [`set_ps_n_pulses()`].
//! - Set Interrupt Mode and Polarity. See: [`set_interrupt()`].
//!
//! [`get_lux()`]: struct.Ltr559.html#method.get_lux
//! [`get_als_raw_data()`]: struct.Ltr559.html#method.get_als_raw_data
//! [`get_status()`]: struct.Ltr559.html#method.get_status
//! [`get_manufacturer_id()`]: struct.Ltr559.html#method.get_manufacturer_id
//! [`get_part_id()`]: struct.Ltr559.html#method.get_part_id
//! [`get_ps_data()`]: struct.Ltr559.html#method.get_ps_data
//! [`set_als_contr()`]: struct.Ltr559.html#method.set_als_contr
//! [`set_ps_contr()`]: struct.Ltr559.html#method.set_ps_contr
//! [`set_ps_led()`]: struct.Ltr559.html#method.set_ps_led
//! [`set_interrupt_persist()`]: struct.Ltr559.html#method.set_interrupt_persist
//! [`set_als_meas_rate()`]: struct.Ltr559.html#method.set_als_meas_rate
//! [`set_als_low_limit_raw()`]: struct.Ltr559.html#method.set_als_low_limit_raw
//! [`set_als_high_limit_raw()`]: struct.Ltr559.html#method.set_als_high_limit_raw
//! [`set_ps_low_limit_raw()`]: struct.Ltr559.html#method.set_ps_low_limit_raw
//! [`set_ps_high_limit_raw()`]: struct.Ltr559.html#method.set_ps_high_limit_raw
//! [`set_ps_meas_rate()`]: struct.Ltr559.html#method.set_ps_meas_rate
//! [`set_ps_offset()`]: struct.Ltr559.html#method.set_ps_offset
//! [`set_ps_n_pulses()`]: struct.Ltr559.html#method.set_ps_n_pulses
//! [`set_interrupt()`]: struct.Ltr559.html#method.set_interrupt
//!
//!
//! ## The devices
//!
//! This driver is compatible with the device Ltr-559
//!
//!
//! Datasheets:
//! - [LTR-559](https://optoelectronics.liteon.com/upload/download/DS86-2013-0003/LTR-559ALS-01_DS_V1.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! In the following examples an instance of the device will be created.
//! `Ltr559::new_device(...)`.
//!
//! ### Create a driver instance for the Ltr559
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ltr_559;
//! use ltr_559::{Ltr559, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let sensor = Ltr559::new_device(dev, address);
//! # }
//! ```
//!
//! ### Read lux
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! #[macro_use]
//! extern crate nb;
//! extern crate ltr_559;
//! use ltr_559::{Ltr559, SlaveAddr, AlsGain, AlsIntTime, AlsMeasRate};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Ltr559::new_device(dev, address);
//! sensor
//!       .set_als_meas_rate(AlsIntTime::_50ms, AlsMeasRate::_50ms)
//!       .unwrap();
//!    sensor.set_als_contr(AlsGain::Gain4x, false, true).unwrap();
//!    loop {
//!      let status = sensor.get_status().unwrap();
//!         if status.als_data_valid {
//!             let (lux_raw_0, lux_raw_1) = sensor.get_als_raw_data().unwrap();
//!             let lux = sensor.get_lux().unwrap();
//!             println!(
//!                 "Raw Lux CH1: 0x{:04x}, CH0: 0x{:04x} Lux = {}, Status.als_data_valid = {}",
//!                 lux_raw_0, lux_raw_1, lux, status.als_data_valid
//!             );
//!         }
//!     }
//! # }
//! ```
//!
//!
//! ### Set the integration time and manual lux range
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate ltr_559;
//! use ltr_559::{AlsIntTime, AlsMeasRate, AlsGain, Ltr559, SlaveAddr};
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Ltr559::new_device(dev, address);
//! sensor.set_als_meas_rate(AlsIntTime::_400ms, AlsMeasRate::_1000ms).unwrap();
//! sensor.set_als_contr(AlsGain::Gain48x, false, true).unwrap();
//! # }
//! ```
//!
//! ### Configure interrupts
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! #[macro_use]
//! extern crate nb;
//! extern crate ltr_559;
//! use ltr_559::{
//!     InterruptMode, InterruptPinPolarity, Ltr559, SlaveAddr, AlsPersist, PsPersist,
//! };
//!
//! # fn main() {
//! let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Ltr559::new_device(dev, address);
//! sensor.set_interrupt(InterruptPinPolarity::High, InterruptMode::Both).unwrap();
//! sensor.set_interrupt_persist(AlsPersist::_4v, PsPersist::_5v).unwrap();
//! sensor.set_als_low_limit_raw(1000).unwrap();
//! sensor.set_als_high_limit_raw(15000).unwrap();
//! loop {
//!     let status = sensor.get_status().unwrap();
//!     println!("status {:?}", status);
//! }
//! # }
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]

pub mod types;
pub use crate::types::{
    AlsGain, AlsIntTime, AlsMeasRate, AlsPersist, InterruptMode, LedCurrent, LedDutyCycle,
    LedPulse, PsMeasRate, PsPersist,
};

use core::marker::PhantomData;
extern crate embedded_hal as hal;
extern crate nb;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus communication error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
}

/// Error type for mode changes.
///
/// This allows to retrieve the unchanged device in case of an error.
pub enum ModeChangeError<E, DEV> {
    /// I²C bus error while changing mode.
    ///
    /// `E` is the error that happened.
    /// `DEV` is the device with the mode unchanged.
    I2C(E, DEV),
}

/// IC markers
#[doc(hidden)]
pub mod ic {
    /// Used for Ltr559 devices
    pub struct Ltr559(());
}

/// markers
#[doc(hidden)]
pub mod marker {
    use super::private;
    pub trait WithDeviceId: private::Sealed {}
}

/// Ltr559 device driver
#[derive(Debug)]
pub struct Ltr559<I2C, IC> {
    i2c: I2C,
    address: u8,
    als_gain: AlsGain,
    als_int: AlsIntTime,
    _ic: PhantomData<IC>,
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A1 and A0
    Alternative(bool, bool),
}

/// Interrupt pin polarity (active state)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptPinPolarity {
    /// Active low (default)
    Low,
    /// Active high
    High,
}

impl InterruptPinPolarity {
    /// Return value InterruptMode
    pub fn value(&self) -> u8 {
        match *self {
            InterruptPinPolarity::Low => 0,
            InterruptPinPolarity::High => 1 << 2,
        }
    }
}

/// Conversion status
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Status {
    /// ALS Data Valid
    pub als_data_valid: bool,
    /// ALS Gain
    pub als_gain: u8,
    /// ALS Interrupt Status
    pub als_interrupt_status: bool,
    /// ALS Data Status
    pub als_data_status: bool,
    /// PS Interrupt Status
    pub ps_interrupt_status: bool,
    /// PS Data Status
    pub ps_data_status: bool,
}

mod device_impl;
mod slave_addr;

mod private {
    use super::ic;
    pub trait Sealed {}

    impl Sealed for ic::Ltr559 {}
}

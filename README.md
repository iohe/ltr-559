# Rust LTR-559 ALS and PS Driver

[![crates.io](https://img.shields.io/crates/v/ltr-559.svg)](https://crates.io/crates/ltr-559)
[![Build Status](https://travis-ci.com/iohe/ltr-559.svg?branch=master)](https://travis-ci.com/iohe/ltr-559)


This is a platform agnostic Rust driver for LTR-559 Ambient light sensor and
Proximity sensor using the [`embedded-hal`] traits.

This driver allows you to:
- Read the measurement in lux. See: `get_lux()`.
- Read the measurement in raw. See: `get_als_raw_data()`.
- Read the conversion status. See: `get_status()`.
- Read PS Data. See: `get_ps_data()`.
- Get the manufacturer ID. See: `get_manufacturer_id()`.
- Get the part ID. See: `get_part_id()`.
- Set ALS Enable, Gain and SW Reset. See: `set_als_contr()`.
- Set PS Mode and Saturation. See: `set_ps_contr()`.
- Set PS LED Pulse, DutyCycle and PeakCurrent. See: `set_ps_led()`.
- Set Interrupt Persist. See: `set_interrupt_persist()`.
- Set ALS Meas Rate. See: `set_als_meas_rate()`.
- Set ALS Low Limit. See: `set_als_low_limit_raw()`.
- Set ALS High Limit. See: `set_als_high_limit_raw()`.
- Set PS Low Limit. See: `set_ps_low_limit_raw()`.
- Set PS High Limit. See: `set_ps_high_limit_raw()`.
- Set PS Meas Rate. See: `set_ps_meas_rate()`.
- Set PS Offset. See: `set_ps_offset()`.
- Set PS N Pulses. See: `set_ps_n_pulses()`.
- Set Interrupt Mode and Polarity. See: `set_interrupt()`.

## The device

The LTR-559 is a an integrated low voltage I2C digital light sensor[ALS] and proximity sensor[PS]


Datasheet: [LTR-559](https://optoelectronics.liteon.com/upload/download/DS86-2013-0003/LTR-559ALS-01_DS_V1.pdf)


## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the device.

```rust
extern crate linux_embedded_hal as hal;

extern crate ltr_559;
use ltr_559::{Ltr559, SlaveAddr};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let sensor = Ltr559::new_device(dev, address);
    sensor
        .set_als_meas_rate(AlsIntTime::_50ms, AlsMeasRate::_50ms)
        .unwrap();
    sensor.set_als_contr(AlsGain::Gain4x, false, true).unwrap();
    loop {
        let status = sensor.get_status().unwrap();
        if status.als_data_valid {
            let (lux_raw_0, lux_raw_1) = sensor.get_als_raw_data().unwrap();
            let lux = sensor.get_lux().unwrap();
            println!(
                "Raw Lux CH1: 0x{:04x}, CH0: 0x{:04x} Lux = {}, Status.als_data_valid = {}",
                lux_raw_0, lux_raw_1, lux, status.als_data_valid
            );
        }
    }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/iohe/ltr-559/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
use crate::hal::blocking::i2c;
use crate::{
    ic, marker, AlsGain, AlsIntTime, AlsMeasRate, AlsPersist, Error, InterruptMode,
    InterruptPinPolarity, LedCurrent, LedDutyCycle, LedPulse, Ltr559, PhantomData, PsMeasRate,
    PsPersist, SlaveAddr, Status,
};

struct Register;
impl Register {
    const ALS_CONTR: u8 = 0x80;
    const PS_CONTR: u8 = 0x81;
    const PS_LED: u8 = 0x82;
    const PS_N_PULSES: u8 = 0x83;
    const PS_MEAS_RATE: u8 = 0x84;
    const ALS_MEAS_RATE: u8 = 0x85;
    const PART_ID: u8 = 0x86;
    const MANUFAC_ID: u8 = 0x87;
    const ALS_DATA_CH1_0: u8 = 0x88;
    const ALS_DATA_CH1_1: u8 = 0x89;
    const ALS_DATA_CH0_0: u8 = 0x8A;
    const ALS_DATA_CH0_1: u8 = 0x8B;
    const ALS_PS_STATUS: u8 = 0x8C;
    const PS_DATA_0: u8 = 0x8D;
    const PS_DATA_1: u8 = 0x8E;
    const INTERRUPT: u8 = 0x8F;
    const PS_THRES_UP_0: u8 = 0x90;
    const PS_THRES_UP_1: u8 = 0x91;
    const PS_THRES_LOW_0: u8 = 0x92;
    const PS_THRES_LOW_1: u8 = 0x93;
    const PS_OFFSET_0: u8 = 0x94;
    const PS_OFFSET_1: u8 = 0x95;
    const ALS_THRES_UP_0: u8 = 0x97;
    const ALS_THRES_UP_1: u8 = 0x98;
    const ALS_THRES_LOW_0: u8 = 0x99;
    const ALS_THRES_LOW_1: u8 = 0x9A;
    const INTERRUPT_PERSIST: u8 = 0x9E;
}

struct BitFlags;
impl BitFlags {
    const R8C_PS_DATA_STATUS: u8 = 1 << 0;
    const R8C_PS_INTERRUPT_STATUS: u8 = 1 << 1;
    const R8C_ALS_DATA_STATUS: u8 = 1 << 2;
    const R8C_ALS_INTERRUPT_STATUS: u8 = 1 << 3;
    const R8C_ALS_DATA_VALID: u8 = 1 << 7;
    const R8C_ALS_GAIN: u8 = 7 << 4;
    const R8E_PS_SATURATION: u8 = 1 << 7;
}

impl marker::WithDeviceId for ic::Ltr559 {}

macro_rules! create {
    ($ic:ident, $method:ident) => {
        impl<I2C> Ltr559<I2C, ic::$ic> {
            /// Create new instance of the device
            pub fn $method(i2c: I2C, address: SlaveAddr) -> Self {
                Ltr559 {
                    i2c,
                    address: address.addr(),
                    als_gain: AlsGain::default(),
                    als_int: AlsIntTime::default(),
                    _ic: PhantomData,
                }
            }
        }
    };
}
create!(Ltr559, new_device);

impl<I2C, IC> Ltr559<I2C, IC> {
    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C, E, IC> Ltr559<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Read the status of the conversion.
    ///
    /// Note that the conversion ready flag is cleared automatically
    /// after calling this method.
    pub fn get_status(&mut self) -> Result<Status, Error<E>> {
        let config = self.read_register(Register::ALS_PS_STATUS)?;
        Ok(Status {
            ps_data_status: (config & BitFlags::R8C_PS_DATA_STATUS) != 0,
            ps_interrupt_status: (config & BitFlags::R8C_PS_INTERRUPT_STATUS) != 0,
            als_data_status: (config & BitFlags::R8C_ALS_DATA_STATUS) != 0,
            als_interrupt_status: (config & BitFlags::R8C_ALS_INTERRUPT_STATUS) != 0,
            als_gain: (config & BitFlags::R8C_ALS_GAIN) >> 4,
            als_data_valid: (config & BitFlags::R8C_ALS_DATA_VALID) != BitFlags::R8C_ALS_DATA_VALID,
        })
    }
}

impl<I2C, E, IC> Ltr559<I2C, IC>
where
    I2C: i2c::Write<Error = E>,
{
    /// Set ALS_CONTR Register
    ///
    pub fn set_als_contr(
        &mut self,
        als_gain: AlsGain,
        sw_reset: bool,
        als_active: bool,
    ) -> Result<(), Error<E>> {
        let mut value: u8 = als_gain.value();
        if sw_reset {
            value += 2;
        }
        if als_active {
            value += 1;
        }

        self.write_register(Register::ALS_CONTR, value)?;
        self.als_gain = als_gain;
        Ok(())
    }

    /// Set PS_CONTR Register
    ///
    pub fn set_ps_contr(
        &mut self,
        ps_saturation_indicator_enable: bool,
        ps_active: bool,
    ) -> Result<(), Error<E>> {
        let mut value: u8 = 0;
        if ps_saturation_indicator_enable {
            value += 1 << 5;
        }
        if ps_active {
            value += 3;
        }

        self.write_register(Register::PS_CONTR, value)
    }

    /// Set PS LED controls
    ///
    pub fn set_ps_led(
        &mut self,
        led_pulse_freq: LedPulse,
        led_duty_cycle: LedDutyCycle,
        led_peak_current: LedCurrent,
    ) -> Result<(), Error<E>> {
        let mut value: u8;
        value = led_pulse_freq.value();
        value |= led_duty_cycle.value();
        value |= led_peak_current.value();
        self.write_register(Register::PS_LED, value)
    }

    /// Set the fault count for both ALS and PS
    ///
    pub fn set_interrupt_persist(
        &mut self,
        als_count: AlsPersist,
        ps_count: PsPersist,
    ) -> Result<(), Error<E>> {
        let value = ps_count.value() | als_count.value();
        self.write_register(Register::INTERRUPT_PERSIST, value)
    }

    /// Set the integration (conversion) time and measurement repeat timer
    pub fn set_als_meas_rate(
        &mut self,
        als_int: AlsIntTime,
        als_meas_rate: AlsMeasRate,
    ) -> Result<(), Error<E>> {
        let value = (als_int.value() << 3) | als_meas_rate.value();
        self.write_register(Register::ALS_MEAS_RATE, value)?;
        self.als_int = als_int;
        Ok(())
    }

    /// Set the lux low limit in raw format
    pub fn set_als_low_limit_raw(&mut self, value: u16) -> Result<(), Error<E>> {
        let low = (value & 0xff) as u8;
        let high = ((value >> 8) & 0xff) as u8;
        self.write_register(Register::ALS_THRES_LOW_0, low)?;
        self.write_register(Register::ALS_THRES_LOW_1, high)?;
        Ok(())
    }

    /// Set the lux low limit in raw format
    pub fn set_als_high_limit_raw(&mut self, value: u16) -> Result<(), Error<E>> {
        let low = (value & 0xff) as u8;
        let high = ((value >> 8) & 0xff) as u8;
        self.write_register(Register::ALS_THRES_UP_0, low)?;
        self.write_register(Register::ALS_THRES_UP_1, high)?;
        Ok(())
    }

    /// Set the ps low limit in raw format
    pub fn set_ps_low_limit_raw(&mut self, value: u16) -> Result<(), Error<E>> {
        let low = (value & 0xff) as u8;
        let high = ((value >> 8) & 0xff) as u8;
        self.write_register(Register::PS_THRES_LOW_0, low)?;
        self.write_register(Register::PS_THRES_LOW_1, high)?;
        Ok(())
    }

    /// Set the ps low limit in raw format
    pub fn set_ps_high_limit_raw(&mut self, value: u16) -> Result<(), Error<E>> {
        let low = (value & 0xff) as u8;
        let high = ((value >> 8) & 0xff) as u8;
        self.write_register(Register::PS_THRES_UP_0, low)?;
        self.write_register(Register::PS_THRES_UP_1, high)?;
        Ok(())
    }

    /// Set PS Meas Rate
    pub fn set_ps_meas_rate(&mut self, ps_meas_rate: PsMeasRate) -> Result<(), Error<E>> {
        self.write_register(Register::PS_MEAS_RATE, ps_meas_rate.value())
    }

    /// Set PS OFFSET.
    ///
    /// Values that exceed 1023 will cause an Err to be returned
    pub fn set_ps_offset(&mut self, value: u16) -> Result<(), Error<E>> {
        if value > 1023 {
            return Err(Error::InvalidInputData);
        }
        let ps_offset_0 = (value & 0xff) as u8;
        let ps_offset_1 = ((value >> 8) & 0xff) as u8;
        self.write_register(Register::PS_OFFSET_0, ps_offset_0)?;
        self.write_register(Register::PS_OFFSET_1, ps_offset_1)
    }

    /// Set PS N Pulses
    ///
    /// Accepted values are 1..16
    pub fn set_ps_n_pulses(&mut self, value: u8) -> Result<(), Error<E>> {
        if value > 0 && value < 16 {
            self.write_register(Register::PS_N_PULSES, value)
        } else {
            Err(Error::InvalidInputData)
        }
    }

    /// Set Interrupt Polarity and Enable
    pub fn set_interrupt(
        &mut self,
        polarity: InterruptPinPolarity,
        mode: InterruptMode,
    ) -> Result<(), Error<E>> {
        let value = mode.value() | polarity.value();
        self.write_register(Register::INTERRUPT, value)
    }
}

impl<I2C, E, IC> Ltr559<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E>,
    IC: marker::WithDeviceId,
{
    /// Read the manufacturer ID
    pub fn get_manufacturer_id(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::MANUFAC_ID)
    }

    /// Read the device part number and revision id
    pub fn get_part_id(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::PART_ID)
    }

    /// Get ALS Data in (als_ch0, als_ch1) format
    pub fn get_als_raw_data(&mut self) -> Result<(u16, u16), Error<E>> {
        let mut measurements = [0; 4];
        let regs = [
            Register::ALS_DATA_CH1_0,
            Register::ALS_DATA_CH1_1,
            Register::ALS_DATA_CH0_0,
            Register::ALS_DATA_CH0_1,
        ];
        for i in 0..4 {
            let value = self.read_register(regs[i])?;
            measurements[i] = value;
        }

        let ch1 = ((measurements[1] as u16) << 8) + (measurements[0] as u16);
        let ch0 = ((measurements[3] as u16) << 8) + (measurements[2] as u16);
        Ok((ch0, ch1))
    }

    /// Return calculated lux
    pub fn get_lux(&mut self) -> Result<f32, Error<E>> {
        let (als_data_ch0, als_data_ch1) = self.get_als_raw_data()?;
        let mut ret;
        let ratio;
        if als_data_ch1 + als_data_ch0 == 0 {
            ratio = 1000.0;
        } else {
            ratio = (als_data_ch1 as f32 * 1000.0) as f32 / (als_data_ch1 + als_data_ch0) as f32;
        }

        let ch0_c: [f32; 4] = [17743.0, 42785.0, 5926.0, 0.0];
        let ch1_c: [f32; 4] = [-11059.0, 19548.0, -1185.0, 0.0];
        let index_co;
        if ratio < 450.0 {
            index_co = 0;
        } else if ratio < 640.0 {
            index_co = 1;
        } else if ratio < 850.0 {
            index_co = 2;
        } else {
            index_co = 3;
        }

        ret = ((als_data_ch0 as f32) * ch0_c[index_co] - (als_data_ch1 as f32) * ch1_c[index_co])
            / 10000.0;

        ret /= self.als_int.lux_compute_value();
        ret /= self.als_gain.lux_compute_value();
        Ok(ret)
    }

    /// Return PS Data in format (value, saturated)
    pub fn get_ps_data(&mut self) -> Result<(u16, bool), Error<E>> {
        let ps0 = self.read_register(Register::PS_DATA_0)?;
        let ps1 = self.read_register(Register::PS_DATA_1)?;
        let value = (((ps1 & 7) as u16) << 8) + (ps0 as u16);
        let saturated = ps1 & BitFlags::R8E_PS_SATURATION;
        Ok((value, saturated != 0))
    }
}

impl<I2C, IC> Ltr559<I2C, IC> {
    /// Reset the internal state of this driver to the default values.
    ///
    /// *Note:* This does not alter the state or configuration of the device.
    ///
    /// This resets the cached configuration register value in this driver to
    /// the power-up (reset) configuration of the device.
    ///
    /// This needs to be called after performing a reset on the device, for
    /// example through an I2C general-call Reset command, which was not done
    /// through this driver to ensure that the configurations in the device
    /// and in the driver match.
    pub fn reset_internal_driver_state(&mut self) {
        self.als_gain = AlsGain::default();
        self.als_int = AlsIntTime::default();
    }
}

impl<I2C, E, IC> Ltr559<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E>,
{
    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }
}

impl<I2C, E, IC> Ltr559<I2C, IC>
where
    I2C: i2c::Write<Error = E>,
{
    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        let data = [register, value];
        self.i2c.write(self.address, &data).map_err(Error::I2C)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct I2cMock;
    impl i2c::Write for I2cMock {
        type Error = ();
        fn write(&mut self, _addr: u8, _bytes: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[test]
    fn can_reset_driver_state() {
        let mut device = Ltr559::new_device(I2cMock {}, SlaveAddr::default());
        device
            .set_interrupt_persist(AlsPersist::_3v, PsPersist::_2v)
            .unwrap();
        device
            .set_als_contr(AlsGain::Gain96x, false, false)
            .unwrap();
        assert_eq!(device.als_gain, AlsGain::Gain96x);
        device.reset_internal_driver_state();
        assert_eq!(device.als_gain, AlsGain::default());
    }

    #[test]
    fn ps_offset_outside() {
        let mut device = Ltr559::new_device(I2cMock {}, SlaveAddr::default());
        assert!(device.set_ps_offset(1024).is_err());
    }

    #[test]
    fn ps_offset_ok() {
        let mut device = Ltr559::new_device(I2cMock {}, SlaveAddr::default());
        assert!(device.set_ps_offset(1023).is_ok());
    }

    #[test]
    fn ps_n_pulses_outside() {
        let mut device = Ltr559::new_device(I2cMock {}, SlaveAddr::default());
        assert!(device.set_ps_n_pulses(0).is_err());
    }

    #[test]
    fn ps_n_pulses_ok() {
        let mut device = Ltr559::new_device(I2cMock {}, SlaveAddr::default());
        assert!(device.set_ps_n_pulses(15).is_ok());
    }
}

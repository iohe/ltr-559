//! Types used in LTR

/// ALS Gain
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlsGain {
    /// Gain 1x (1 lux to 64k lux default)
    Gain1x,
    /// Gain 2x (0.5 lux to 32k lux)
    Gain2x,
    /// Gain 4x (0.25 lux to 16k lux)
    Gain4x,
    /// Gain 8x (0.125 lux to 8k lux)
    Gain8x,
    /// Gain 48x (0.2 lux to 1.3k lux)
    Gain48x,
    /// Gain 96x (0.1 lux to 600 lux)
    Gain96x,
}

/// Default for AlsGain
impl Default for AlsGain {
    fn default() -> Self {
        AlsGain::Gain1x
    }
}

impl AlsGain {
    /// ALS Gain value
    pub fn value(&self) -> u8 {
        const BIT_OFFSET: u8 = 2;
        match *self {
            AlsGain::Gain1x => 0 << BIT_OFFSET,
            AlsGain::Gain2x => 1 << BIT_OFFSET,
            AlsGain::Gain4x => 2 << BIT_OFFSET,
            AlsGain::Gain8x => 3 << BIT_OFFSET,
            AlsGain::Gain48x => 6 << BIT_OFFSET,
            AlsGain::Gain96x => 7 << BIT_OFFSET,
        }
    }

    /// ALS_GAIN value, used in lux computation
    pub fn lux_compute_value(&self) -> f32 {
        match *self {
            AlsGain::Gain1x => 1.0,
            AlsGain::Gain2x => 2.0,
            AlsGain::Gain4x => 4.0,
            AlsGain::Gain8x => 8.0,
            AlsGain::Gain48x => 48.0,
            AlsGain::Gain96x => 96.0,
        }
    }
}

/// LED Pulse Modulation Frequency
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LedPulse {
    /// Pulse 30khz
    Pulse30,
    /// Pulse 40khz
    Pulse40,
    /// Pulse 50khz
    Pulse50,
    /// Pulse 60khz (default)
    Pulse60,
    /// Pulse 70khz
    Pulse70,
    /// Pulse 80khz
    Pulse80,
    /// Pulse 90khz
    Pulse90,
    /// Pulse 100khz
    Pulse100,
}

impl Default for LedPulse {
    fn default() -> Self {
        LedPulse::Pulse60
    }
}

/// Implement something
impl LedPulse {
    /// LED Pulse value
    pub fn value(&self) -> u8 {
        const BIT_OFFSET: u8 = 5;
        match *self {
            LedPulse::Pulse30 => 0 << BIT_OFFSET,
            LedPulse::Pulse40 => 1 << BIT_OFFSET,
            LedPulse::Pulse50 => 2 << BIT_OFFSET,
            LedPulse::Pulse60 => 3 << BIT_OFFSET,
            LedPulse::Pulse70 => 4 << BIT_OFFSET,
            LedPulse::Pulse80 => 5 << BIT_OFFSET,
            LedPulse::Pulse90 => 6 << BIT_OFFSET,
            LedPulse::Pulse100 => 7 << BIT_OFFSET,
        }
    }
}

/// LED Duty Cycle
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LedDutyCycle {
    /// 25% duty
    _25,
    /// 50% duty
    _50,
    /// 75% duty
    _75,
    /// 100% (default)
    _100,
}

impl Default for LedDutyCycle {
    fn default() -> Self {
        LedDutyCycle::_100
    }
}

impl LedDutyCycle {
    /// LED Duty Cycle bits value
    pub fn value(&self) -> u8 {
        const BIT_OFFSET: u8 = 3;
        match *self {
            LedDutyCycle::_25 => 0 << BIT_OFFSET,
            LedDutyCycle::_50 => 1 << BIT_OFFSET,
            LedDutyCycle::_75 => 2 << BIT_OFFSET,
            LedDutyCycle::_100 => 3 << BIT_OFFSET,
        }
    }
}

/// Operating mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LedCurrent {
    /// 5 mA
    _5mA,
    /// 10 mA
    _10mA,
    /// 20 mA
    _20mA,
    /// 50 mA
    _50mA,
    /// 100 mA (default)
    _100mA,
}

impl Default for LedCurrent {
    fn default() -> Self {
        LedCurrent::_100mA
    }
}

impl LedCurrent {
    /// LED Current bits value
    pub fn value(&self) -> u8 {
        match *self {
            LedCurrent::_5mA => 0,
            LedCurrent::_10mA => 1,
            LedCurrent::_20mA => 2,
            LedCurrent::_50mA => 3,
            LedCurrent::_100mA => 7,
        }
    }
}

/// Operating mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PsMeasRate {
    /// 50 ms
    _50ms,
    /// 70 ms
    _70ms,
    /// 100 ms (default)
    _100ms,
    /// 200 ms
    _200ms,
    /// 500 ms
    _500ms,
    /// 1000 ms
    _1000ms,
    /// 2000 ms
    _2000ms,
    /// 10ms
    _10ms,
}

impl Default for PsMeasRate {
    fn default() -> Self {
        PsMeasRate::_100ms
    }
}

impl PsMeasRate {
    /// PS Measure Rate value
    pub fn value(&self) -> u8 {
        match *self {
            PsMeasRate::_10ms => 8,
            PsMeasRate::_50ms => 0,
            PsMeasRate::_70ms => 1,
            PsMeasRate::_100ms => 2,
            PsMeasRate::_200ms => 3,
            PsMeasRate::_500ms => 4,
            PsMeasRate::_1000ms => 5,
            PsMeasRate::_2000ms => 6,
        }
    }
}

/// Operating mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlsMeasRate {
    /// 50 ms
    _50ms,
    /// 100 ms
    _100ms,
    /// 200 ms
    _200ms,
    /// 500 ms (default)
    _500ms,
    /// 1000 ms
    _1000ms,
    /// 2000 ms
    _2000ms,
}

impl Default for AlsMeasRate {
    fn default() -> Self {
        AlsMeasRate::_500ms
    }
}

impl AlsMeasRate {
    /// Return value for AlsMeasRate
    pub fn value(&self) -> u8 {
        match *self {
            AlsMeasRate::_50ms => 0,
            AlsMeasRate::_100ms => 1,
            AlsMeasRate::_200ms => 2,
            AlsMeasRate::_500ms => 3,
            AlsMeasRate::_1000ms => 4,
            AlsMeasRate::_2000ms => 7,
        }
    }
}

/// ALS Integration Time
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlsIntTime {
    /// 50 ms
    _50ms,
    /// 100 ms
    _100ms,
    /// 150 ms
    _150ms,
    /// 200 ms
    _200ms,
    /// 250 ms
    _250ms,
    /// 300 ms
    _300ms,
    /// 350 ms
    _350ms,
    /// 400 ms
    _400ms,
}

impl Default for AlsIntTime {
    fn default() -> Self {
        AlsIntTime::_100ms
    }
}

impl AlsIntTime {
    /// Return value for AlsIntegrationTime
    pub fn value(&self) -> u8 {
        match *self {
            AlsIntTime::_100ms => 0,
            AlsIntTime::_50ms => 1,
            AlsIntTime::_200ms => 2,
            AlsIntTime::_400ms => 3,
            AlsIntTime::_150ms => 4,
            AlsIntTime::_250ms => 5,
            AlsIntTime::_300ms => 6,
            AlsIntTime::_350ms => 7,
        }
    }

    /// ALS_INT value used for lux computation
    pub fn lux_compute_value(&self) -> f32 {
        match *self {
            AlsIntTime::_100ms => 1.0,
            AlsIntTime::_50ms => 0.5,
            AlsIntTime::_200ms => 2.0,
            AlsIntTime::_400ms => 4.0,
            AlsIntTime::_150ms => 1.5,
            AlsIntTime::_250ms => 2.5,
            AlsIntTime::_300ms => 3.0,
            AlsIntTime::_350ms => 3.5,
        }
    }
}

/// ALS Interrupt Persist
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlsPersist {
    /// every ALS value out of threshold range (default)
    EveryTime,
    /// 2 consecutive values outside threshold
    _2v,
    /// 3 consecutive values outside threshold
    _3v,
    /// 4 consecutive values outside threshold
    _4v,
    /// 5 consecutive values outside threshold
    _5v,
    /// 6 consecutive values outside threshold
    _6v,
    /// 7 consecutive values outside threshold
    _7v,
    /// 8 consecutive values outside threshold
    _8v,
    /// 9 consecutive values outside threshold
    _9v,
    /// 10 consecutive values outside threshold
    _10v,
    /// 11 consecutive values outside threshold
    _11v,
    /// 12 consecutive values outside threshold
    _12v,
    /// 13 consecutive values outside threshold
    _13v,
    /// 14 consecutive values outside threshold
    _14v,
    /// 15 consecutive values outside threshold
    _15v,
    /// 16 consecutive values outside threshold
    _16v,
}

impl Default for AlsPersist {
    fn default() -> Self {
        AlsPersist::EveryTime
    }
}

impl AlsPersist {
    /// Return value for ALS Persistent
    pub fn value(&self) -> u8 {
        match *self {
            AlsPersist::EveryTime => 0,
            AlsPersist::_2v => 1,
            AlsPersist::_3v => 2,
            AlsPersist::_4v => 3,
            AlsPersist::_5v => 4,
            AlsPersist::_6v => 5,
            AlsPersist::_7v => 6,
            AlsPersist::_8v => 7,
            AlsPersist::_9v => 8,
            AlsPersist::_10v => 9,
            AlsPersist::_11v => 10,
            AlsPersist::_12v => 11,
            AlsPersist::_13v => 12,
            AlsPersist::_14v => 13,
            AlsPersist::_15v => 14,
            AlsPersist::_16v => 15,
        }
    }
}

/// PS Interrupt Persist
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PsPersist {
    /// every PS value out of threshold range (default)
    EveryTime,
    /// 2 consecutive values outside threshold
    _2v,
    /// 3 consecutive values outside threshold
    _3v,
    /// 4 consecutive values outside threshold
    _4v,
    /// 5 consecutive values outside threshold
    _5v,
    /// 6 consecutive values outside threshold
    _6v,
    /// 7 consecutive values outside threshold
    _7v,
    /// 8 consecutive values outside threshold
    _8v,
    /// 9 consecutive values outside threshold
    _9v,
    /// 10 consecutive values outside threshold
    _10v,
    /// 11 consecutive values outside threshold
    _11v,
    /// 12 consecutive values outside threshold
    _12v,
    /// 13 consecutive values outside threshold
    _13v,
    /// 14 consecutive values outside threshold
    _14v,
    /// 15 consecutive values outside threshold
    _15v,
    /// 16 consecutive values outside threshold
    _16v,
}

impl Default for PsPersist {
    fn default() -> Self {
        PsPersist::EveryTime
    }
}

impl PsPersist {
    /// Return value for PS Persist
    pub fn value(&self) -> u8 {
        const BIT_OFFSET: u8 = 4;
        match *self {
            PsPersist::EveryTime => 0,
            PsPersist::_2v => 1 << BIT_OFFSET,
            PsPersist::_3v => 2 << BIT_OFFSET,
            PsPersist::_4v => 3 << BIT_OFFSET,
            PsPersist::_5v => 4 << BIT_OFFSET,
            PsPersist::_6v => 5 << BIT_OFFSET,
            PsPersist::_7v => 6 << BIT_OFFSET,
            PsPersist::_8v => 7 << BIT_OFFSET,
            PsPersist::_9v => 8 << BIT_OFFSET,
            PsPersist::_10v => 9 << BIT_OFFSET,
            PsPersist::_11v => 10 << BIT_OFFSET,
            PsPersist::_12v => 11 << BIT_OFFSET,
            PsPersist::_13v => 12 << BIT_OFFSET,
            PsPersist::_14v => 13 << BIT_OFFSET,
            PsPersist::_15v => 14 << BIT_OFFSET,
            PsPersist::_16v => 15 << BIT_OFFSET,
        }
    }
}

/// PS Interrupt Persist
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptMode {
    /// Interrupt mode is disabled
    Inactive,
    /// Interrupt for PS
    OnlyPS,
    /// Interrupt for ALS
    OnlyALS,
    /// Interrupt for both ALS and PS
    Both,
}

impl Default for InterruptMode {
    fn default() -> Self {
        InterruptMode::Inactive
    }
}

impl InterruptMode {
    /// Return value InterruptMode
    pub fn value(&self) -> u8 {
        match *self {
            InterruptMode::Inactive => 0,
            InterruptMode::OnlyPS => 1,
            InterruptMode::OnlyALS => 2,
            InterruptMode::Both => 3,
        }
    }
}

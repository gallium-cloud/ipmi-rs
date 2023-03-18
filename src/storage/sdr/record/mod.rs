mod full_sensor_record;
pub use full_sensor_record::FullSensorRecord;

use nonmax::NonMaxU8;

use crate::connection::LogicalUnit;

use super::{RecordId, Unit};

#[derive(Debug, Clone, Copy)]
pub enum SensorOwner {
    I2C(u8),
    System(u8),
}

impl From<u8> for SensorOwner {
    fn from(value: u8) -> Self {
        let id = (value & 0xFE) >> 1;

        if (value & 1) == 1 {
            Self::System(id)
        } else {
            Self::I2C(id)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EntityRelativeTo {
    System,
    Device,
}

#[derive(Debug, Clone, Copy)]

pub enum EntityInstance {
    Physical {
        relative: EntityRelativeTo,
        instance_number: u8,
    },
    LogicalContainer {
        relative: EntityRelativeTo,
        instance_number: u8,
    },
}

impl From<u8> for EntityInstance {
    fn from(value: u8) -> Self {
        let instance_number = value & 0x7F;
        let relative = match instance_number {
            0x00..=0x5F => EntityRelativeTo::System,
            0x60..=0x7F => EntityRelativeTo::Device,
            _ => unreachable!(),
        };

        if (value & 0x80) == 0x80 {
            Self::LogicalContainer {
                relative,
                instance_number,
            }
        } else {
            Self::Physical {
                relative,
                instance_number,
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SensorInitialization {
    pub settable: bool,
    pub scanning: bool,
    pub events: bool,
    pub thresholds: bool,
    pub hysteresis: bool,
    pub sensor_type: bool,
    pub event_generation_enabled_on_startup: bool,
    pub sensor_scanning_enabled_on_startup: bool,
}

impl From<u8> for SensorInitialization {
    fn from(value: u8) -> Self {
        bitflags::bitflags! {
            pub struct Flags: u8 {
                const SETTABLE = 1 << 7;
                const SCANNING = 1 << 6;
                const EVENTS = 1 << 5;
                const THRESHOLDS = 1 << 4;
                const HYSTERESIS = 1 << 3;
                const TYPE = 1 << 2;
                const EVENTGEN_ON_STARTUP = 1 << 1;
                const SCANNING_ON_STARTUP = 1 << 0;
            }
        }

        let flags = Flags::from_bits_truncate(value);

        Self {
            settable: flags.contains(Flags::SETTABLE),
            scanning: flags.contains(Flags::SCANNING),
            events: flags.contains(Flags::EVENTS),
            thresholds: flags.contains(Flags::THRESHOLDS),
            hysteresis: flags.contains(Flags::THRESHOLDS),
            sensor_type: flags.contains(Flags::TYPE),
            event_generation_enabled_on_startup: flags.contains(Flags::EVENTGEN_ON_STARTUP),
            sensor_scanning_enabled_on_startup: flags.contains(Flags::SCANNING_ON_STARTUP),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HysteresisCapability {
    NoneOrUnspecified,
    Readable,
    ReadableAndSettable,
    FixedAndUnreadable,
}

bitflags::bitflags! {
    pub struct ThresholdAssertEventMask: u16 {
        const UPPER_NON_RECOVERABLE_GOING_HIGH = 1 << 11;
        const UPPER_NON_RECOVERABLE_GOING_LOW = 1 << 10;
        const UPPER_CRITICAL_GOING_HIGH = 1 << 9;
        const UPPER_CRITICAL_GOING_LOW = 1 << 8;
        const UPPER_NON_CRITICAL_GOING_HIGH = 1 << 7;
        const UPPER_NON_CRITICAL_GOING_LOW = 1 << 6;
        const LOWER_NON_RECOVERABLE_GOING_HIGH = 1 << 5;
        const LOWER_NON_RECOVERABLE_GOING_LOW = 1 << 4;
        const LOWER_CRITICAL_GOING_HIGH = 1 << 3;
        const LOWER_CRITICAL_GOING_LOW = 1 << 2;
        const LOWER_NON_CRITICAL_GOING_HIGH = 1 << 1;
        const LOWER_NON_CRITICAL_GOING_LOW = 1 << 0;

    }
}

impl ThresholdAssertEventMask {
    pub fn for_kind(&self, kind: ThresholdKind) -> &[EventKind] {
        static BOTH: [EventKind; 2] = [EventKind::GoingHigh, EventKind::GoingLow];
        static HIGH: [EventKind; 1] = [EventKind::GoingHigh];
        static LOW: [EventKind; 1] = [EventKind::GoingLow];
        static NONE: [EventKind; 0] = [];

        let (low, high) = match kind {
            ThresholdKind::LowerNonCritical => (
                self.contains(Self::LOWER_NON_CRITICAL_GOING_LOW),
                self.contains(Self::LOWER_NON_CRITICAL_GOING_HIGH),
            ),
            ThresholdKind::LowerCritical => (
                self.contains(Self::LOWER_CRITICAL_GOING_LOW),
                self.contains(Self::LOWER_CRITICAL_GOING_HIGH),
            ),
            ThresholdKind::LowerNonRecoverable => (
                self.contains(Self::LOWER_NON_RECOVERABLE_GOING_LOW),
                self.contains(Self::LOWER_NON_RECOVERABLE_GOING_HIGH),
            ),
            ThresholdKind::UpperNonCritical => (
                self.contains(Self::UPPER_NON_CRITICAL_GOING_LOW),
                self.contains(Self::UPPER_NON_CRITICAL_GOING_HIGH),
            ),
            ThresholdKind::UpperCritical => (
                self.contains(Self::UPPER_CRITICAL_GOING_LOW),
                self.contains(Self::UPPER_CRITICAL_GOING_HIGH),
            ),
            ThresholdKind::UpperNonRecoverable => (
                self.contains(Self::UPPER_NON_RECOVERABLE_GOING_LOW),
                self.contains(Self::UPPER_NON_RECOVERABLE_GOING_HIGH),
            ),
        };

        if low && high {
            &BOTH
        } else if low {
            &LOW
        } else if high {
            &HIGH
        } else {
            &NONE
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventKind {
    GoingHigh,
    GoingLow,
}

#[derive(Debug, Clone, Copy)]

pub struct Thresholds {
    pub lower_non_recoverable: bool,
    pub lower_critical: bool,
    pub lower_non_critical: bool,
    pub upper_non_recoverable: bool,
    pub upper_critical: bool,
    pub upper_non_critical: bool,
}

impl Thresholds {
    pub fn for_kind(&self, kind: ThresholdKind) -> bool {
        match kind {
            ThresholdKind::LowerNonCritical => self.lower_non_critical,
            ThresholdKind::LowerCritical => self.lower_critical,
            ThresholdKind::LowerNonRecoverable => self.lower_non_recoverable,
            ThresholdKind::UpperNonCritical => self.upper_non_critical,
            ThresholdKind::UpperCritical => self.upper_critical,
            ThresholdKind::UpperNonRecoverable => self.upper_non_recoverable,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThresholdKind {
    LowerNonCritical,
    LowerCritical,
    LowerNonRecoverable,
    UpperNonCritical,
    UpperCritical,
    UpperNonRecoverable,
}

impl ThresholdKind {
    pub fn variants() -> impl Iterator<Item = Self> {
        [
            Self::LowerNonCritical,
            Self::LowerCritical,
            Self::LowerNonRecoverable,
            Self::UpperNonCritical,
            Self::UpperCritical,
            Self::UpperNonRecoverable,
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Threshold {
    pub kind: ThresholdKind,
    pub readable: bool,
    pub settable: bool,
    pub event_assert_going_high: bool,
    pub event_assert_going_low: bool,
    pub event_deassert_going_high: bool,
    pub event_deassert_going_low: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum ThresholdAccessCapability {
    None,
    Readable {
        readable: Thresholds,
        values: Thresholds,
    },
    ReadableAndSettable {
        readable: Thresholds,
        values: Thresholds,
        settable: Thresholds,
    },
    FixedAndUnreadable {
        supported: Thresholds,
    },
}

impl ThresholdAccessCapability {
    pub fn readable(&self, kind: ThresholdKind) -> bool {
        match self {
            ThresholdAccessCapability::Readable { readable, .. } => readable.for_kind(kind),
            ThresholdAccessCapability::ReadableAndSettable { readable, .. } => {
                readable.for_kind(kind)
            }
            _ => false,
        }
    }

    pub fn settable(&self, kind: ThresholdKind) -> bool {
        match self {
            ThresholdAccessCapability::ReadableAndSettable { settable, .. } => {
                settable.for_kind(kind)
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SensorCapabilities {
    pub ignore: bool,
    pub auto_rearm: bool,
    // TODO: make a type
    pub event_message_control: u8,
    pub hysteresis: HysteresisCapability,
    pub threshold_access: ThresholdAccessCapability,
    pub assertion_threshold_events: ThresholdAssertEventMask,
    pub deassertion_threshold_events: ThresholdAssertEventMask,
}

impl SensorCapabilities {
    pub fn new(
        caps: u8,
        assert_lower_thrsd: u16,
        deassert_upper_thrshd: u16,
        discrete_rd_thrsd_set_thrshd_read: u16,
    ) -> Self {
        let ignore = (caps & 0x80) == 0x80;
        let auto_rearm = (caps & 0x40) == 0x40;
        let hysteresis = match caps & 0x30 >> 4 {
            0b00 => HysteresisCapability::NoneOrUnspecified,
            0b01 => HysteresisCapability::Readable,
            0b10 => HysteresisCapability::ReadableAndSettable,
            0b11 => HysteresisCapability::FixedAndUnreadable,
            _ => unreachable!(),
        };
        let event_message_control = caps & 0b11;

        let assertion_event_mask = ThresholdAssertEventMask::from_bits_truncate(assert_lower_thrsd);
        let deassertion_event_mask =
            ThresholdAssertEventMask::from_bits_truncate(deassert_upper_thrshd);

        let threshold_read_value_mask = Thresholds {
            lower_non_recoverable: ((assert_lower_thrsd >> 14) & 0x1) == 1,
            lower_critical: ((assert_lower_thrsd >> 13) & 0x1) == 1,
            lower_non_critical: ((assert_lower_thrsd >> 12) & 0x1) == 1,
            upper_non_recoverable: ((deassert_upper_thrshd >> 14) & 0x1) == 1,
            upper_critical: ((deassert_upper_thrshd >> 14) & 0x1) == 1,
            upper_non_critical: ((deassert_upper_thrshd >> 14) & 0x1) == 1,
        };

        let threshold_set_mask = Thresholds {
            upper_non_recoverable: ((discrete_rd_thrsd_set_thrshd_read >> 13) & 0x1) == 1,
            upper_critical: ((discrete_rd_thrsd_set_thrshd_read >> 12) & 0x1) == 1,
            upper_non_critical: ((discrete_rd_thrsd_set_thrshd_read >> 11) & 0x1) == 1,
            lower_non_recoverable: ((discrete_rd_thrsd_set_thrshd_read >> 10) & 0x1) == 1,
            lower_critical: ((discrete_rd_thrsd_set_thrshd_read >> 9) & 0x1) == 1,
            lower_non_critical: ((discrete_rd_thrsd_set_thrshd_read >> 8) & 0x1) == 1,
        };

        let threshold_read_mask = Thresholds {
            upper_non_recoverable: ((discrete_rd_thrsd_set_thrshd_read >> 5) & 0x1) == 1,
            upper_critical: ((discrete_rd_thrsd_set_thrshd_read >> 4) & 0x1) == 1,
            upper_non_critical: ((discrete_rd_thrsd_set_thrshd_read >> 3) & 0x1) == 1,
            lower_non_recoverable: ((discrete_rd_thrsd_set_thrshd_read >> 2) & 0x1) == 1,
            lower_critical: ((discrete_rd_thrsd_set_thrshd_read >> 1) & 0x1) == 1,
            lower_non_critical: ((discrete_rd_thrsd_set_thrshd_read >> 0) & 0x1) == 1,
        };

        let threshold_access_support = match (caps & 0xC) >> 2 {
            0b00 => ThresholdAccessCapability::None,
            0b01 => ThresholdAccessCapability::Readable {
                readable: threshold_read_mask,
                values: threshold_read_value_mask,
            },
            0b10 => ThresholdAccessCapability::ReadableAndSettable {
                readable: threshold_read_mask,
                values: threshold_read_value_mask,
                settable: threshold_set_mask,
            },
            0b11 => ThresholdAccessCapability::FixedAndUnreadable {
                supported: threshold_read_mask,
            },
            _ => unreachable!(),
        };

        Self {
            ignore,
            auto_rearm,
            hysteresis,
            event_message_control,
            threshold_access: threshold_access_support,
            assertion_threshold_events: assertion_event_mask,
            deassertion_threshold_events: deassertion_event_mask,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataFormat {
    Unsigned,
    OnesComplement,
    TwosComplement,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RateUnit {
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModifierUnit {
    BasicUnitDivByModifierUnit,
    BasicUnitMulByModifierUnit,
}

#[derive(Debug, Clone, Copy)]
pub struct SensorUnits {
    pub analog_data_format: Option<DataFormat>,
    pub rate: Option<RateUnit>,
    pub modifier: Option<ModifierUnit>,
    pub is_percentage: bool,
}

impl From<u8> for SensorUnits {
    fn from(sensor_units_1: u8) -> Self {
        let analog_data_format = match (sensor_units_1 >> 6) & 0x03 {
            0b00 => Some(DataFormat::Unsigned),
            0b01 => Some(DataFormat::OnesComplement),
            0b10 => Some(DataFormat::TwosComplement),
            0b11 => None,
            _ => unreachable!(),
        };

        let rate = match (sensor_units_1 >> 3) & 0b111 {
            0b000 => None,
            0b001 => Some(RateUnit::Microsecond),
            0b010 => Some(RateUnit::Millisecond),
            0b011 => Some(RateUnit::Second),
            0b100 => Some(RateUnit::Minute),
            0b101 => Some(RateUnit::Hour),
            0b110 => Some(RateUnit::Day),
            0b111 => None,
            _ => unreachable!(),
        };

        let modifier = match (sensor_units_1 >> 1) & 0b11 {
            0b00 => None,
            0b01 => Some(ModifierUnit::BasicUnitDivByModifierUnit),
            0b10 => Some(ModifierUnit::BasicUnitMulByModifierUnit),
            0b11 => None,
            _ => unreachable!(),
        };

        let is_percentage = (sensor_units_1 & 0x1) == 0x1;

        Self {
            analog_data_format,
            rate,
            modifier,
            is_percentage,
        }
    }
}

#[derive(Debug, Clone, Copy)]

pub enum Linearization {
    Linear,
    Ln,
    Log10,
    Log2,
    E,
    Exp10,
    Exp2,
    OneOverX,
    Sqr,
    Cube,
    Sqrt,
    CubeRoot,
    Oem(u8),
    Unknown(u8),
}

impl From<u8> for Linearization {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Linear,
            1 => Self::Ln,
            2 => Self::Log10,
            3 => Self::Log2,
            4 => Self::E,
            5 => Self::Exp10,
            6 => Self::Exp2,
            7 => Self::OneOverX,
            8 => Self::Sqr,
            9 => Self::Sqrt,
            10 => Self::Cube,
            11 => Self::Sqrt,
            12 => Self::CubeRoot,
            0x71..=0x7F => Self::Oem(value),
            v => Self::Unknown(v),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    UnspecifiedNotApplicable,
    Input,
    Output,
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let dir = match value {
            0b00 => Self::UnspecifiedNotApplicable,
            0b01 => Self::Input,
            0b10 => Self::Output,
            _ => return Err(()),
        };
        Ok(dir)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeLengthRaw<'a>(u8, &'a [u8]);

impl<'a> TypeLengthRaw<'a> {
    pub fn new(value: u8, other_data: &'a [u8]) -> Self {
        Self(value, other_data)
    }
}

impl<'a> Into<SensorId> for TypeLengthRaw<'a> {
    fn into(self) -> SensorId {
        let Self(value, data) = self;
        let type_code = (value >> 6) & 0x3;

        let length = (value & 0xF) as usize;

        let data = &data[..length];

        let str = core::str::from_utf8(data).map(ToString::to_string);

        match type_code {
            0b00 => SensorId::Unicode(str.unwrap()),
            0b01 => SensorId::BCDPlus(data.to_vec()),
            0b10 => SensorId::Ascii6BPacked(data.to_vec()),
            0b11 => SensorId::Ascii8BAndLatin1(str.unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SensorId {
    Unicode(String),
    BCDPlus(Vec<u8>),
    Ascii6BPacked(Vec<u8>),
    Ascii8BAndLatin1(String),
}

impl SensorId {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            SensorId::Unicode(v) => Some(v.as_str()),
            SensorId::Ascii8BAndLatin1(v) => Some(v.as_str()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecordHeader {
    pub id: RecordId,
    pub sdr_version_major: u8,
    pub sdr_version_minor: u8,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub header: RecordHeader,
    pub contents: RecordContents,
}

#[derive(Debug, Clone)]
pub enum RecordContents {
    FullSensor(FullSensorRecord),
    Unknown { data: Vec<u8> },
}

impl Record {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 5 {
            return None;
        }

        let record_id = RecordId::new_raw(u16::from_le_bytes([data[0], data[1]]));
        let sdr_version_min = (data[2] & 0xF0) >> 4;
        let sdr_version_maj = data[2] & 0x0F;
        let record_type = data[3];
        let record_length = data[4];

        let record_data = &data[5..];
        if record_data.len() != record_length as usize {
            return None;
        }

        let contents = if record_type == 0x01 {
            RecordContents::FullSensor(FullSensorRecord::parse(record_data)?)
        } else {
            RecordContents::Unknown {
                data: record_data.to_vec(),
            }
        };

        Some(Self {
            header: RecordHeader {
                id: record_id,
                sdr_version_minor: sdr_version_min,
                sdr_version_major: sdr_version_maj,
            },
            contents,
        })
    }
}

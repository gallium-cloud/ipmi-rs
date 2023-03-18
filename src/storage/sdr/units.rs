use finte::IntEnum;

#[derive(Debug, Clone, Copy, PartialEq, finte::IntEnum)]
#[repr(u8)]
pub enum Unit {
    Unspecified = 0,
    DegreesCelcius = 1,
    DegreesFahrenheit = 2,
    DegreesKelvin = 3,
    Volt = 4,
    Amp = 5,
    Watt = 6,
    Joule = 7,
    Coulomb = 8,
    VoltAmpere = 9,
    Nit = 10,
    Lumen = 11,
    Lux = 12,
    Candela = 13,
    KiloPascal = 14,
    PoundsPerSquareInch = 15,
    Newton = 16,
    CubicFeetPerMinute = 17,
    RevolutionsPerMinute = 18,
    Hertz = 19,
    Microsecond = 20,
    Millisecond = 21,
    Second = 22,
    Minute = 23,
    Hour = 24,
    Day = 25,
    Week = 26,
    Mil = 27,
    Inch = 28,
    Foot = 29,
    CubicInch = 30,
    CubicFoot = 31,
    Millimeter = 32,
    Centimeter = 33,
    Meter = 34,
    CubicCentimeter = 35,
    CubicMeter = 36,
    Liter = 37,
    FluidOunce = 38,
    Radian = 39,
    Steradian = 40,
    Revolution = 41,
    Cycle = 42,
    Gravity = 43,
    Ounce = 44,
    Pound = 45,
    FootPound = 46,
    OunceInch = 47,
    Gauss = 48,
    Gilbert = 49,
    Henry = 50,
    Millihenry = 51,
    Farad = 52,
    Microfarad = 53,
    Ohm = 54,
    Siemen = 55,
    Mole = 56,
    Becquerel = 57,
    PartsPerMillion = 58,
    Decibel = 60,
    AWeightedDecibel = 61,
    CWeightedDecibel = 62,
    Gray = 63,
    Sievert = 64,
    ColorTemperatureDegreesKelvin = 65,
    Bit = 66,
    Kilobit = 67,
    Megabit = 68,
    Gigabit = 69,
    Byte = 70,
    Kilobyte = 71,
    Megabyte = 72,
    Gigabyte = 73,
    Word = 74,
    DoubleWord = 75,
    QuadWord = 76,
    CacheLine = 77,
    Hit = 78,
    Miss = 79,
    Retry = 80,
    Reset = 81,
    OverrunOrUnderflow = 82,
    Underrun = 83,
    Collision = 84,
    Packet = 85,
    Message = 86,
    Character = 87,
    Error = 88,
    CorrectableError = 89,
    UncorrectableError = 90,
    FatalError = 91,
    Gram = 92,
    Unknown = 0xFF,
}

impl TryFrom<u8> for Unit {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
        Self::try_from_int(value).map_err(|_| ())
    }
}

impl From<Unit> for u8 {
    fn from(value: Unit) -> Self {
        value.int_value()
    }
}

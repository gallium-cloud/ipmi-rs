use crate::{
    connection::{CompletionCode, IpmiCommand, Message, ParseResponseError},
    storage::record::SensorNumber,
};

pub struct GetSensorReading {
    sensor_number: SensorNumber,
}

impl GetSensorReading {
    pub fn new(value: SensorNumber) -> Self {
        Self {
            sensor_number: value,
        }
    }

    pub fn for_sensor(sensor: SensorNumber) -> Self {
        Self::new(sensor)
    }
}

pub struct SensorReading {
    pub reading: u8,
    pub all_event_messages_disabled: bool,
    pub scanning_disabled: bool,
    pub reading_or_state_unavailable: bool,
    pub offset_data_1: Option<u8>,
    pub offset_data_2: Option<u8>,
}

impl SensorReading {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 2 {
            return None;
        }

        let reading = data[0];

        // Bit indicates that all event messages are enabled => must negate result
        let all_event_messages_disabled = (data[1] & 0x80) != 0x80;

        // Bit indicates that sensor scanning is enabled => must negate result
        let scanning_disabled = (data[1] & 0x40) != 0x40;

        let reading_or_state_unavailable = (data[1] & 0x20) == 0x20;

        let offset_data_1 = data.get(2).map(Clone::clone);
        let offset_data_2 = data.get(3).map(Clone::clone);

        Some(Self {
            reading,
            all_event_messages_disabled,
            scanning_disabled,
            reading_or_state_unavailable,
            offset_data_1,
            offset_data_2,
        })
    }
}

impl From<GetSensorReading> for Message {
    fn from(value: GetSensorReading) -> Self {
        Message::new(
            crate::connection::NetFn::SensorEvent,
            0x2D,
            vec![value.sensor_number.get()],
        )
    }
}

impl IpmiCommand for GetSensorReading {
    type Output = SensorReading;

    type Error = ();

    fn parse_response(
        completion_code: CompletionCode,
        data: &[u8],
    ) -> Result<Self::Output, ParseResponseError<Self::Error>> {
        Self::check_cc_success(completion_code)?;

        SensorReading::parse(data).ok_or(ParseResponseError::NotEnoughData)
    }
}

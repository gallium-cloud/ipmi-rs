use crate::connection::{IpmiCommand, Message, NetFn};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrivilegeLevel {
    Callback,
    User,
    Operator,
    Administrator,
    OemProperietary,
}

impl TryFrom<u8> for PrivilegeLevel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value = value & 0x0F;
        let level = match value {
            1 => Self::Callback,
            2 => Self::User,
            3 => Self::Operator,
            4 => Self::Administrator,
            5 => Self::OemProperietary,
            _ => return Err(()),
        };
        Ok(level)
    }
}

impl From<PrivilegeLevel> for u8 {
    fn from(value: PrivilegeLevel) -> Self {
        match value {
            PrivilegeLevel::Callback => 1,
            PrivilegeLevel::User => 2,
            PrivilegeLevel::Operator => 3,
            PrivilegeLevel::Administrator => 4,
            PrivilegeLevel::OemProperietary => 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChannelAuthenticationCapabilities {
    pub channel_number: u8,
    pub oem_proprietary: bool,
    pub key: bool,
    pub md5: bool,
    pub md2: bool,
    pub none: bool,
    pub kg_status: bool,
    pub per_message_authentication_enabled: bool,
    pub user_level_authentication_enabled: bool,
    pub non_null_usernames_enabled: bool,
    pub null_usernames_enabled: bool,
    pub anonymous_login_enabled: bool,
    pub ipmi2_connections_supported: bool,
    pub ipmi15_connections_supported: bool,
    pub oem_id: [u8; 3],
    pub oem_auxiliary_data: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Channel {
    Current,
    Number(u8),
}

#[derive(Debug, Clone)]
pub struct GetChannelAuthenticationCapabilities {
    channel_number: u8,
    privilege_level: PrivilegeLevel,
}

impl GetChannelAuthenticationCapabilities {
    pub fn new(channel_number: Channel, privilege_level: PrivilegeLevel) -> Self {
        let channel_number = match channel_number {
            Channel::Current => 0xE,
            Channel::Number(n) => n & 0x0F,
        };

        Self {
            channel_number,
            privilege_level,
        }
    }
}

impl Into<Message> for GetChannelAuthenticationCapabilities {
    fn into(self) -> Message {
        Message::new_request(
            NetFn::App,
            0x38,
            vec![
                0x80 | (self.channel_number & 0x0F),
                self.privilege_level.into(),
            ],
        )
    }
}

impl IpmiCommand for GetChannelAuthenticationCapabilities {
    type Output = ChannelAuthenticationCapabilities;

    type Error = ();

    fn parse_response(
        completion_code: crate::connection::CompletionCode,
        data: &[u8],
    ) -> Result<Self::Output, crate::connection::ParseResponseError<Self::Error>> {
        Self::check_cc_success(completion_code)?;

        if data.len() < 7 {
            return Err(crate::connection::ParseResponseError::NotEnoughData);
        }

        let channel_number = data[0];
        let ipmi2_ext_cap = (data[1] & 0x80) == 0x80;

        let oem_proprietary = (data[1] & 0x20) == 0x20;
        let key = (data[1] & 0x10) == 0x10;
        let md5 = (data[1] & 0x04) == 0x04;
        let md2 = (data[1] & 0x02) == 0x02;
        let none = (data[1] & 0x01) == 0x01;

        let pma = (data[2] & 0x10) == 0x10;
        let ula = (data[2] & 0x08) == 0x08;
        let nnue = (data[2] & 0x04) == 0x04;
        let nue = (data[2] & 0x02) == 0x02;
        let ale = (data[2] & 0x01) == 0x01;

        let (kg, v2, v15, oem_id, oem_aux) = if ipmi2_ext_cap {
            if data.len() < 8 {
                return Err(crate::connection::ParseResponseError::NotEnoughData);
            }

            let kg = (data[2] & 0x20) == 0x20;

            let v2 = (data[3] & 0x02) == 0x02;
            let v15 = (data[3] & 0x01) == 0x01;

            let oem_id = [data[4], data[5], data[6]];
            let oem_aux = data[7];
            (kg, v2, v15, oem_id, oem_aux)
        } else {
            let oem_id = [data[3], data[4], data[5]];
            let oem_aux = data[6];
            (false, false, false, oem_id, oem_aux)
        };

        Ok(ChannelAuthenticationCapabilities {
            channel_number,
            oem_proprietary,
            key,
            md5,
            md2,
            none,
            kg_status: kg,
            per_message_authentication_enabled: !pma,
            user_level_authentication_enabled: !ula,
            non_null_usernames_enabled: nnue,
            null_usernames_enabled: nue,
            anonymous_login_enabled: ale,
            ipmi2_connections_supported: v2,
            ipmi15_connections_supported: v15,
            oem_id,
            oem_auxiliary_data: oem_aux,
        })
    }
}

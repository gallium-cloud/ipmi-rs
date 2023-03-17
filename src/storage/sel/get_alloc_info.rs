use crate::{
    connection::{IpmiCommand, Message, NetFn, ParseResponseError},
    LogOutput, Loggable,
};

pub struct GetAllocInfo;

impl IpmiCommand for GetAllocInfo {
    type Output = AllocInfo;

    type Error = ();

    fn parse_response(
        completion_code: crate::connection::CompletionCode,
        data: &[u8],
    ) -> Result<Self::Output, ParseResponseError<Self::Error>> {
        Self::check_cc_success(completion_code)?;

        AllocInfo::parse(data).ok_or(ParseResponseError::NotEnoughData)
    }
}

impl Into<Message> for GetAllocInfo {
    fn into(self) -> Message {
        Message::new(NetFn::Storage, 0x41, Vec::new())
    }
}

pub struct AllocInfo {
    inner: crate::storage::AllocInfo,
}

impl AllocInfo {
    pub fn parse(data: &[u8]) -> Option<Self> {
        Some(Self {
            inner: crate::storage::AllocInfo::parse(data)?,
        })
    }
}

impl Loggable for AllocInfo {
    fn log(&self, output: LogOutput) {
        crate::log!(output, "SEL Allocation Information:");
        self.inner.log(output);
    }
}

impl core::ops::Deref for AllocInfo {
    type Target = crate::storage::AllocInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AllocInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponseUnavailableReason {
    Unknown,
    SDRInUpdate,
    DeviceInFwUpdate,
    BMCInitializing,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompletionCode {
    Success,
    NodeBusy,
    InvalidCommand,
    InvalidCommandForLun,
    ProcessingTimeout,
    OutOfSpace,
    ReservationCancelledOrInvalidId,
    RequestDataTruncated,
    RequestDataLenInvalid,
    RequestDataLengthLimitExceeded,
    ParameterOutOfRange,
    CannotReturnNumOfRequestedBytes,
    RequestedDatapointNotPresent,
    InvalidDataFieldInRequest,
    CommandIllegalForSensorOrRecord,
    ResponseUnavailable { reason: ResponseUnavailableReason },
    CannotExecuteDuplicateRequest,
    DestinationUnavailable,
    InsufficientPrivilege,
    CannotExecuteCommand,
    SubFunctionDisabled,
    Unspecified,
    Oem(u8),
    CommandSpecific(u8),
    Reserved(u8),
}

impl From<u8> for CompletionCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Success,
            0xC0 => Self::NodeBusy,
            0xC1 => Self::InvalidCommand,
            0xC2 => Self::InvalidCommandForLun,
            0xC3 => Self::ProcessingTimeout,
            0xC4 => Self::OutOfSpace,
            0xC5 => Self::ReservationCancelledOrInvalidId,
            0xC6 => Self::RequestDataTruncated,
            0xC7 => Self::RequestDataLenInvalid,
            0xC8 => Self::RequestDataLengthLimitExceeded,
            0xC9 => Self::ParameterOutOfRange,
            0xCA => Self::CannotReturnNumOfRequestedBytes,
            0xCB => Self::RequestedDatapointNotPresent,
            0xCC => Self::InvalidDataFieldInRequest,
            0xCD => Self::CommandIllegalForSensorOrRecord,
            0xCE => Self::ResponseUnavailable {
                reason: ResponseUnavailableReason::Unknown,
            },
            0xCF => Self::CannotExecuteDuplicateRequest,
            0xD0 => Self::ResponseUnavailable {
                reason: ResponseUnavailableReason::SDRInUpdate,
            },
            0xD1 => Self::ResponseUnavailable {
                reason: ResponseUnavailableReason::DeviceInFwUpdate,
            },
            0xD2 => Self::ResponseUnavailable {
                reason: ResponseUnavailableReason::BMCInitializing,
            },
            0xD3 => Self::DestinationUnavailable,
            0xD4 => Self::InsufficientPrivilege,
            0xD5 => Self::CannotExecuteCommand,
            0xD6 => Self::SubFunctionDisabled,
            0xFF => Self::Unspecified,
            0x01..=0x7E => Self::Oem(value),
            0x80..=0xBE => Self::CommandSpecific(value),
            v => Self::Reserved(v),
        }
    }
}

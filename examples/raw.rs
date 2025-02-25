use clap::Parser;
use common::CommonOpts;
use ipmi_rs::connection::RequestTargetAddress;
use ipmi_rs::{
    connection::Message,
    connection::{IpmiConnection, LogicalUnit, Request},
};

mod common;

#[derive(Parser)]
pub struct Command {
    #[clap(flatten)]
    common: CommonOpts,

    #[clap(required = true)]
    message: Vec<String>,
}

fn try_parse_message(input: &[u8]) -> std::io::Result<Message> {
    if input.len() < 2 {
        let err = std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Need at least 2 bytes of input".to_string(),
        );
        return Err(err);
    }

    let cmd = input[1];

    let data: Vec<u8> = input[2..].iter().map(|v| *v).collect();

    Ok(Message::new_raw(input[0], cmd, data))
}

fn main() -> std::io::Result<()> {
    pretty_env_logger::formatted_builder()
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or("info".to_string()))
        .init();

    let command = Command::parse();

    let mut data = Vec::new();
    for arg in command.message {
        let u8_value = u8::from_str_radix(&arg, 16).map_err(|_| {
            let err = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Could not parse '{arg}' as hex integer"),
            );
            err
        })?;
        data.push(u8_value);
    }

    let message = try_parse_message(&data)?;

    let mut request: Request = Request::new(message, RequestTargetAddress::Bmc(LogicalUnit::Zero));

    let ipmi = command.common.get_connection()?;

    let result = match ipmi {
        common::IpmiConnectionEnum::Rmcp(mut r) => r.inner_mut().send_recv(&mut request)?,
        common::IpmiConnectionEnum::File(mut f) => f.inner_mut().send_recv(&mut request)?,
    };

    println!("Response:");
    println!("Completion code: 0x{:02X}", result.cc());
    println!("NetFN: 0x{:02X} ({:?})", result.netfn_raw(), result.netfn());
    println!("Cmd: 0x{:02X}", result.cmd());
    println!("Data: {:02X?}", result.data());
    Ok(())
}

use sonettobuf::{CmdId, prost};
use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Tokio IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Packet error: {0}")]
    Packet(#[from] PacketError),

    #[error("Command error: {0}")]
    Cmd(#[from] CmdError),
}
// #[error("Custom error: {0}")]
// Custom(&'static str),
// }

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Packet length less than header (expected: {0}, actual: {1})")]
    LengthLessThanHeader(usize, usize),

    #[error("Packet length mismatch (expected: {0}, actual: {1})")]
    LengthMismatch(usize, usize),

    #[error("Client packet data decode failed: {0}")]
    ClientPacketDataDecodeFail(#[from] prost::DecodeError),
}

#[derive(Debug, Error)]
pub enum CmdError {
    #[error("Unregistered Cmd: {0}")]
    UnregisteredCmd(i16),

    #[error("Unhandled Cmd: {0:?}")]
    UnhandledCmd(CmdId),
}

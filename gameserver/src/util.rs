use crate::DynError;
use crate::packet::ServerPacket;
use sonettobuf::CmdId;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[allow(dead_code)]
pub struct ByteWriting;

#[allow(dead_code)]
impl ByteWriting {
    const VAR_0_1: usize = 256;

    pub fn read_string(arg_12_0: &[u8], arg_12_1: usize) -> Option<(usize, String)> {
        if arg_12_1 + 1 >= arg_12_0.len() {
            return None;
        }

        let var_12_0 = arg_12_0[arg_12_1] as usize;
        let var_12_1 = arg_12_0[arg_12_1 + 1] as usize;
        let var_12_2 = (var_12_0 * Self::VAR_0_1 + var_12_1) as usize;

        if arg_12_1 + 2 + var_12_2 > arg_12_0.len() {
            return None;
        }

        match String::from_utf8(arg_12_0[(arg_12_1 + 2)..(arg_12_1 + 2 + var_12_2)].to_vec()) {
            Ok(var_12_3) => Some((arg_12_1 + 2 + var_12_2, var_12_3)),
            Err(_) => return None,
        }
    }

    pub fn write_string(arg_15_0: &mut Vec<u8>, arg_15_1: &str, arg_15_2: usize) {
        let var_15_0 = arg_15_1.as_bytes();
        arg_15_0[arg_15_2] = (var_15_0.len() / Self::VAR_0_1) as u8;
        arg_15_0[arg_15_2 + 1] = (var_15_0.len() % Self::VAR_0_1) as u8;
        arg_15_0[(arg_15_2 + 2)..(arg_15_2 + 2 + var_15_0.len())].copy_from_slice(var_15_0)
    }
}

pub async fn send_raw_buffer(
    socket: &mut TcpStream,
    cmd_id: CmdId,
    data: Vec<u8>,
    result_code: i16,
) -> Result<(), DynError> {
    let cmd_id = cmd_id as i16;
    let server_packet = ServerPacket {
        cmd_id,
        result_code,
        data,
        down_tag: 0,
        up_tag: 0,
    };
    socket.write_all(&server_packet.encode()).await?;
    Ok(())
}

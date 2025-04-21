use byteorder::{BE, ByteOrder};

#[derive(Debug)]
pub struct ServerPacket {
    pub cmd_id: i16,
    pub result_code: u16,
    pub up_tag: u8,
    pub down_tag: u8,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct ClientPacket {
    pub sequence: u32,
    pub cmd_id: i16,
    pub up_tag: u8,
    pub data: Vec<u8>,
}

impl ServerPacket {
    const PACKET_HEADER: usize = 10;

    pub fn encode(&self) -> Vec<u8> {
        let total_len = Self::PACKET_HEADER + self.data.len();
        let mut buffer = vec![0u8; total_len];

        BE::write_u32(&mut buffer[0..4], (total_len - 4) as u32);
        BE::write_i16(&mut buffer[4..6], self.cmd_id);
        BE::write_u16(&mut buffer[6..8], self.result_code);
        buffer[8] = self.up_tag;
        buffer[9] = self.down_tag;
        (&mut buffer[Self::PACKET_HEADER..]).copy_from_slice(&self.data);

        buffer
    }
}

impl ClientPacket {
    const PACKET_HEADER: usize = 11;

    pub fn decode(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < Self::PACKET_HEADER {
            return None;
        }

        let packet_size = BE::read_u32(&buffer[0..4]) as usize;

        if buffer.len() != packet_size + 4 {
            return None;
        }

        let sequence = BE::read_u32(&buffer[4..8]);
        let cmd_id = BE::read_i16(&buffer[8..10]);
        let up_tag = buffer[10];
        let data = buffer[Self::PACKET_HEADER..].to_vec();

        Some(Self {
            sequence,
            cmd_id,
            up_tag,
            data,
        })
    }
}

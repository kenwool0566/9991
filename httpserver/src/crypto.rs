use flate2::{read::GzEncoder, Compression};
use std::io::Read;

// this is just a 1:1 from pseudocode i suppose
#[allow(unused_assignments)]
fn swap_each_two_bytes(bytes: &mut Vec<u8>) {
    let length = bytes.len();
    let hlength = length / 2;
    let mut v5 = length >> 31;
    let mut v7 = 0;
    let mut v4 = 0;

    while v4 < hlength {
        if v7 >= length {
            break;
        }
        bytes[v7] = bytes[v7] ^ 237;
        v5 = v7 + 1;
        if v7 + 1 > length {
            break;
        }
        bytes[v5] = bytes[v5] ^ 237;
        if v7 >= length || v5 >= length {
            break;
        }
        bytes[v7] = bytes[v7] ^ bytes[v5];
        bytes[v5] = bytes[v5] ^ bytes[v7];
        bytes[v7] = bytes[v7] ^ bytes[v5];
        v7 = v7 + 2;
        v4 = v4 + 1;
    }
}

pub fn encrypt(json: serde_json::Value) -> Vec<u8> {
    let mut bytes = serde_json::to_vec(&json).unwrap();
    swap_each_two_bytes(&mut bytes);
    let mut gz = GzEncoder::new(&*bytes, Compression::default()); // default = level 6
    let mut result = Vec::new();
    gz.read_to_end(&mut result).unwrap();
    result
}

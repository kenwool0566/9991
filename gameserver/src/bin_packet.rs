use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../packets/"]
struct BinPacket;

pub fn get_packet_by_id(id: i16) -> Option<Vec<u8>> {
    let prefix = format!("packet_{}_", id);
    for filename in BinPacket::iter() {
        if filename.starts_with(&prefix) {
            if let Some(content) = BinPacket::get(&filename) {
                return Some(content.data.to_vec());
            }
        }
    }
    None
}

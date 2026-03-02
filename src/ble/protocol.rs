use crate::constants::PACKET_SIZE;

// Packet layout: [cmd (1 B)][payload up to 14 B, zero-padded][checksum (1 B)]
// Checksum = low byte of the sum of bytes 0..14.
pub fn build_cmd(key: u8, payload: &[u8]) -> [u8; PACKET_SIZE] {
  let mut buf = [0u8; PACKET_SIZE];
  buf[0] = key;
  let len = payload.len().min(PACKET_SIZE - 2);
  buf[1..1 + len].copy_from_slice(&payload[..len]);
  let sum: u16 = buf[..PACKET_SIZE - 1].iter().map(|&b| b as u16).sum();
  buf[PACKET_SIZE - 1] = (sum & 0xFF) as u8;
  buf
}

fn checksum_ok(data: &[u8]) -> bool {
  if data.len() != PACKET_SIZE {
    return false;
  }
  let sum: u16 = data[..PACKET_SIZE - 1].iter().map(|&b| b as u16).sum();
  data[PACKET_SIZE - 1] == (sum & 0xFF) as u8
}

pub fn parse_cmd(data: &[u8]) -> Option<(u8, &[u8])> {
  if !checksum_ok(data) {
    return None;
  }
  // Mask off bit 7 the ring sets it as a response flag, CMD_* constants don't have it.
  Some((data[0] & 0x7F, &data[1..PACKET_SIZE - 1]))
}

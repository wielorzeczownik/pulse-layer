// Packet format: [cmd (1 B)][payload (14 B, zero-padded)][checksum (1 B)]
const PACKET_SIZE: usize = 16;

// The ring sets bit 7 of the cmd byte as a response flag; CMD_* constants don't carry it
const RESPONSE_FLAG: u8 = 0x80;

// Packet layout: [cmd (1 B)][payload up to 14 B, zero-padded][checksum (1 B)]
// Checksum = low byte of the sum of bytes 0..14.
pub fn build_cmd(key: u8, payload: &[u8]) -> [u8; PACKET_SIZE] {
  let mut buf = [0u8; PACKET_SIZE];
  buf[0] = key;
  let len = payload.len().min(PACKET_SIZE - 2);
  buf[1..=len].copy_from_slice(&payload[..len]);
  let sum: u16 = buf[..PACKET_SIZE - 1]
    .iter()
    .map(|&byte| u16::from(byte))
    .sum();
  buf[PACKET_SIZE - 1] = (sum & 0xFF) as u8;
  buf
}

fn checksum_ok(data: &[u8]) -> bool {
  if data.len() != PACKET_SIZE {
    return false;
  }
  let sum: u16 = data[..PACKET_SIZE - 1]
    .iter()
    .map(|&byte| u16::from(byte))
    .sum();
  data[PACKET_SIZE - 1] == (sum & 0xFF) as u8
}

pub fn parse_cmd(data: &[u8]) -> Option<(u8, &[u8])> {
  if !checksum_ok(data) {
    return None;
  }
  Some((data[0] & !RESPONSE_FLAG, &data[1..PACKET_SIZE - 1]))
}

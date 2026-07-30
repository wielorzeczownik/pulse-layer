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

#[cfg(test)]
mod tests {
  use super::{PACKET_SIZE, RESPONSE_FLAG, build_cmd, parse_cmd};

  #[test]
  fn built_packet_carries_the_checksum_of_its_own_bytes() {
    let packet = build_cmd(0x69, &[1, 2, 3]);
    let expected: u16 = packet[..PACKET_SIZE - 1]
      .iter()
      .map(|&byte| u16::from(byte))
      .sum();
    assert_eq!(u16::from(packet[PACKET_SIZE - 1]), expected & 0xFF);
  }

  #[test]
  fn payload_is_zero_padded_to_a_fixed_length() {
    let packet = build_cmd(0x01, &[0xAB]);
    assert_eq!(packet.len(), PACKET_SIZE);
    assert_eq!(packet[1], 0xAB);
    assert!(packet[2..PACKET_SIZE - 1].iter().all(|&byte| byte == 0));
  }

  #[test]
  fn oversized_payload_is_truncated_rather_than_overflowing() {
    let packet = build_cmd(0x01, &[0xFF; 64]);
    assert_eq!(packet.len(), PACKET_SIZE);
    assert!(parse_cmd(&packet).is_some());
  }

  #[test]
  fn parse_round_trips_a_built_packet() {
    let packet = build_cmd(0x69, &[7, 8]);
    let (key, payload) = parse_cmd(&packet).expect("valid packet");
    assert_eq!(key, 0x69);
    assert_eq!(&payload[..2], &[7, 8]);
  }

  #[test]
  fn parse_strips_the_response_flag_from_the_key() {
    let packet = build_cmd(0x69 | RESPONSE_FLAG, &[]);
    let (key, _) = parse_cmd(&packet).expect("valid packet");
    assert_eq!(key, 0x69);
  }

  #[test]
  fn parse_rejects_a_corrupted_or_missized_packet() {
    let mut packet = build_cmd(0x69, &[1, 2, 3]);
    packet[2] ^= 0xFF;
    assert!(parse_cmd(&packet).is_none());
    assert!(parse_cmd(&packet[..PACKET_SIZE - 1]).is_none());
    assert!(parse_cmd(&[]).is_none());
  }
}

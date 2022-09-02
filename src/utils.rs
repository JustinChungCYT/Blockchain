pub fn u32_to_u8_slice(target: &u32) -> Vec<u8> {
     vec![
          (target >> 8 * 0) as u8,
          (target >> 8 * 1) as u8,
          (target >> 8 * 2) as u8,
          (target >> 8 * 3) as u8
     ]
}

pub fn u64_to_u8_slice(target: &u64) -> Vec<u8> {
     vec![
          (target >> 8 * 0) as u8,
          (target >> 8 * 1) as u8,
          (target >> 8 * 2) as u8,
          (target >> 8 * 3) as u8,
          (target >> 8 * 4) as u8,
          (target >> 8 * 5) as u8,
          (target >> 8 * 6) as u8,
          (target >> 8 * 7) as u8,
     ]
}

pub fn u128_to_u8_slice(target: &u128) -> Vec<u8> {
     vec![
          (target >> 8 * 0) as u8,
          (target >> 8 * 1) as u8,
          (target >> 8 * 2) as u8,
          (target >> 8 * 3) as u8,
          (target >> 8 * 4) as u8,
          (target >> 8 * 5) as u8,
          (target >> 8 * 6) as u8,
          (target >> 8 * 7) as u8,
          (target >> 8 * 8) as u8,
          (target >> 8 * 9) as u8,
          (target >> 8 * 10) as u8,
          (target >> 8 * 11) as u8,
          (target >> 8 * 12) as u8,
          (target >> 8 * 13) as u8,
          (target >> 8 * 14) as u8,
          (target >> 8 * 15) as u8,
     ]
}

pub fn hash_to_difficulty(hash: &Vec<u8>) -> u128 {
     ((hash[31] as u128) << 0xf * 8) |
     ((hash[30] as u128) << 0xe * 8) |
     ((hash[29] as u128) << 0xd * 8) |
     ((hash[28] as u128) << 0xc * 8) |
     ((hash[27] as u128) << 0xb * 8) |
     ((hash[26] as u128) << 0xa * 8) |
     ((hash[25] as u128) << 0x9 * 8) |
     ((hash[24] as u128) << 0x8 * 8) |
     ((hash[23] as u128) << 0x7 * 8) |
     ((hash[22] as u128) << 0x6 * 8) |
     ((hash[21] as u128) << 0x5 * 8) |
     ((hash[20] as u128) << 0x4 * 8) |
     ((hash[19] as u128) << 0x3 * 8) |
     ((hash[18] as u128) << 0x2 * 8) |
     ((hash[17] as u128) << 0x1 * 8) |
     ((hash[16] as u128) << 0x0 * 8)
}
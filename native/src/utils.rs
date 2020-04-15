pub fn convert_to_binary_from_hex(hex: &str) -> String {
  hex.chars().map(to_binary).collect()
}

pub fn to_binary(c: char) -> &'static str {
  match c {
    '0' => "0000",
    '1' => "0001",
    '2' => "0010",
    '3' => "0011",
    '4' => "0100",
    '5' => "0101",
    '6' => "0110",
    '7' => "0111",
    '8' => "1000",
    '9' => "1001",
    'A' => "1010",
    'B' => "1011",
    'C' => "1100",
    'D' => "1101",
    'E' => "1110",
    'F' => "1111",
    _ => "",
  }
}

pub fn get_hamming_distance_by_hex_hash(hash_1: &str, hash_2: &str) -> u32 {
  let mut binary_hash_1 = convert_to_binary_from_hex(hash_1);
  let mut binary_hash_2 = convert_to_binary_from_hex(hash_2);
  while binary_hash_1.len() < 64 {
    binary_hash_1 = String::from("0") + &binary_hash_1;
  }
  while binary_hash_2.len() < 64 {
    binary_hash_2 = String::from("0") + &binary_hash_2;
  }

  let mut hamming_distance = 0_u32;
  for index in 0..64 {
    if binary_hash_1.as_bytes()[index] != binary_hash_2.as_bytes()[index] {
      hamming_distance += 1;
    }
  }
  return hamming_distance;
}

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

trait FromVarInt {
    fn from_varint(&mut self) -> Result<i32, &'static str>;
}

impl FromVarInt for Vec<u8> {
    fn from_varint(&mut self) -> Result<i32, &'static str> {
        let mut value: i32 = 0;
        let mut position = 0;
        let mut current_byte_position = 0;
        let mut current_byte;

        loop {
            current_byte = self[current_byte_position];
            current_byte_position += 1;
            value |= (current_byte as i32 & SEGMENT_BITS as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err("VarInt is too big");
            }
        }

        self.drain(0..current_byte_position);

        return Ok(value);
    }
}

trait ToVarInt {
    fn to_varint(self) -> Vec<u8>;
}

impl ToVarInt for i32 {
    fn to_varint(self) -> Vec<u8> {
        let mut value = self as u32;
        let mut result = vec![];

        loop {
            if (value & !(SEGMENT_BITS as u32)) == 0 {
                result.push(value as u8);

                break;
            }

            result.push((value as u8 & SEGMENT_BITS) | CONTINUE_BIT);
            value >>= 7;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_varint() {
        let test_data = [
            (vec![0x00], 0),
            (vec![0x01], 1),
            (vec![0x02], 2),
            (vec![0x7f], 127),
            (vec![0x80, 0x01], 128),
            (vec![0xff, 0x01], 255),
            (vec![0xdd, 0xc7, 0x01], 25565),
            (vec![0xff, 0xff, 0x7f], 2097151),
            (vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647),
            (vec![0xff, 0xff, 0xff, 0xff, 0x0f], -1),
            (vec![0x80, 0x80, 0x80, 0x80, 0x08], -2147483648),
        ];

        for (mut input, output) in test_data {
            assert_eq!(input.from_varint(), Ok(output));
            assert_eq!(input.len(), 0);
        }
    }

    #[test]
    fn to_varint() {
        let test_data = [
            (0, vec![0x00]),
            (1, vec![0x01]),
            (2, vec![0x02]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (255, vec![0xff, 0x01]),
            (25565, vec![0xdd, 0xc7, 0x01]),
            (2097151, vec![0xff, 0xff, 0x7f]),
            (2147483647, vec![0xff, 0xff, 0xff, 0xff, 0x07]),
            (-1, vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
            (-2147483648, vec![0x80, 0x80, 0x80, 0x80, 0x08]),
        ];

        for (input, output) in test_data {
            assert_eq!(input.to_varint(), output);
        }
    }
}

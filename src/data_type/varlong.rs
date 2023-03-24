const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub trait FromVarLong {
    fn from_varlong(&mut self) -> Result<i64, String>;
}

impl FromVarLong for Vec<u8> {
    fn from_varlong(&mut self) -> Result<i64, String> {
        let mut value: i64 = 0;
        let mut position = 0;
        let mut current_byte_position = 0;
        let mut current_byte;

        if self.len() == 0 {
            return Err("No data to parse VarLong".to_string());
        }

        loop {
            if self.len() == current_byte_position {
                return Err("Not enough data to parse VarLong".to_string());
            }

            current_byte = self[current_byte_position];
            current_byte_position += 1;
            value |= (current_byte as i64 & SEGMENT_BITS as i64) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 64 {
                return Err("VarLong is too big".to_string());
            }
        }

        self.drain(0..current_byte_position);

        return Ok(value);
    }
}

pub trait ToVarLong {
    fn to_varlong(self) -> Vec<u8>;
}

impl ToVarLong for i64 {
    fn to_varlong(self) -> Vec<u8> {
        let mut value = self as u64;
        let mut result = vec![];

        loop {
            if (value & !(SEGMENT_BITS as u64)) == 0 {
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
    fn from_varlong() {
        let test_data = [
            (vec![0x00], 0),
            (vec![0x01], 1),
            (vec![0x02], 2),
            (vec![0x7f], 127),
            (vec![0x80, 0x01], 128),
            (vec![0xff, 0x01], 255),
            (vec![0xff, 0xff, 0xff, 0xff, 0x07], 2147483647),
            (
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
                9223372036854775807,
            ),
            (
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
                -1,
            ),
            (
                vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
                -2147483648,
            ),
            (
                vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
                -9223372036854775808,
            ),
        ];

        assert_eq!(
            vec![].from_varlong(),
            Err("No data to parse VarLong".to_string())
        );
        assert_eq!(
            vec![0x80, 0x80].from_varlong(),
            Err("Not enough data to parse VarLong".to_string())
        );

        for (mut input, output) in test_data {
            assert_eq!(input.from_varlong(), Ok(output));
            assert_eq!(input.len(), 0);
        }
    }

    #[test]
    fn to_varlong() {
        let test_data = [
            (0, vec![0x00]),
            (1, vec![0x01]),
            (2, vec![0x02]),
            (127, vec![0x7f]),
            (128, vec![0x80, 0x01]),
            (255, vec![0xff, 0x01]),
            (2147483647, vec![0xff, 0xff, 0xff, 0xff, 0x07]),
            (
                9223372036854775807,
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
            ),
            (
                -1,
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
            ),
            (
                -2147483648,
                vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
            ),
            (
                -9223372036854775808,
                vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            ),
        ];

        for (input, output) in test_data {
            assert_eq!(input.to_varlong(), output);
        }
    }
}

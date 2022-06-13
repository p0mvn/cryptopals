
fn decode_digit(c: &u8) -> Result<u8, String> {
    if c >= &b'0' && c <= &b'9' {
        return Ok(c - 48);
    } else if c >= &b'a' && c <= &b'f' {
        return Ok(c - 97 + 10);
    }
    return Err(format!("{} was not a hex character", c));
}

// decode if value is a slice containing hex characters, decodes
// the hex representation to regular bytes. Oterwise, returns error.
pub fn decode(value: &[u8]) -> Result<Vec<u8>, String> {
    let mut result = vec![];

    let mut current: u8 = 0;
    let value_length = value.len();
    for i in 0..value_length {
        if i % 2 == 0 {
            current = decode_digit(&value[i])? << 4;

            if i == value_length - 1 {
                result.push(current);
            }
        } else {
            current = current | decode_digit(&value[i])?;
            result.push(current);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn decode_digit_all_no_error() {
        let input: &[u8] = b"0123456789abcdef";

        for c in input {
            decode_digit(&c).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn decode_digit_invalid_input_error() {
        let input: &[u8] = b"0123456789abcdef$";

        for c in input {
            decode_digit(&c).unwrap();
        }
    }

    #[test]
    fn decode_basic() {
        let expected_output: Vec<u8> = vec![74, 39];
        let input: &[u8] = b"4a27";

        let actual_output = decode(input).unwrap();
        assert_eq!(&expected_output as &[u8], &actual_output as &[u8]);
    }

    #[test]
    fn decode_invalid_char() {
        let input: &[u8] = b"4/27";

        let err = decode(input).unwrap_err();

        assert_eq!(err, format!("{} was not a hex character", b'/'))
    }

    #[test]
    fn decode_cryptopals_hex() {
        let input: &[u8] =  b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_output: Vec<u8> = hex::decode(input).unwrap();

        let actual_output = decode(input).unwrap();
        assert_eq!(&expected_output as &[u8], &actual_output as &[u8]);
    }
}

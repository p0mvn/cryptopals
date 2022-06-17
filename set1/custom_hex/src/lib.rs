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

pub fn encode_byte(b: u8) -> Result<u8, String> {
    if b <= 9 {
        return Ok(b + 48);
    } else if b >= 10 && b <= 15 {
        return Ok(97 + b - 10);
    }
    return Err(format!("{} does not translate to hex", b));
}

// encode encodes raw bytes to base16 according to RFC-4648.
pub fn encode(value: &[u8]) -> Result<Vec<u8>, String> {
    let mut result: Vec<u8> = Vec::with_capacity(value.len() * 2);

    for b in value {
        result.push(encode_byte(b >> 4 & 0b1111)?);
        result.push(encode_byte(b & 0b1111)?);
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

    #[test]
    fn encode_byte_zero() {
        const EXPECTED_OUTPUT: u8 = b'0';
        let input: u8 = 0b0;

        let actual_output = encode_byte(input).unwrap();
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    #[test]
    fn encode_byte_eleven() {
        const EXPECTED_OUTPUT: u8 = b'a';
        let input: u8 = 0xa;

        let actual_output = encode_byte(input).unwrap();
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    #[test]
    fn encode_byte_sixteen_invalid() {
        let input: u8 = 0b10000;

        let err = encode_byte(input).unwrap_err();
        assert_eq!(err, format!("{} does not translate to hex", input))
    }

    #[test]
    fn encode_empty() {
        const EXPECTED_OUTPUT: &[u8] = b"";
        let input: &[u8] = b"";

        let actual_output = encode(input).unwrap();
        assert_eq!(EXPECTED_OUTPUT, &actual_output);
    }

    #[test]
    fn encode_f() {
        const EXPECTED_OUTPUT: &[u8] = b"66";
        let input: &[u8] = b"f";

        let actual_output = encode(input).unwrap();
        assert_eq!(EXPECTED_OUTPUT, &actual_output);
    }

    #[test]
    fn encode_fo() {
        const EXPECTED_OUTPUT: &[u8] = b"666f";
        let input: &[u8] = b"fo";

        let actual_output = encode(input).unwrap();
        assert_eq!(EXPECTED_OUTPUT, &actual_output);
    }

    #[test]
    fn encode_foo() {
        const EXPECTED_OUTPUT: &[u8] = b"666f6f";
        let input: &[u8] = b"foo";

        let actual_output = encode(input).unwrap();
        assert_eq!(EXPECTED_OUTPUT, &actual_output);
    }

    #[test]
    fn encode_foob() {
        const EXPECTED_OUTPUT: &[u8] = b"666f6f62";
        let input: &[u8] = b"foob";

        let actual_output = encode(input).unwrap();
        assert_eq!(EXPECTED_OUTPUT, &actual_output);
    }

    #[test]
    fn encode_fooba() {
        const EXPECTED_OUTPUT: &[u8] = b"666f6f6261";
        let input: &[u8] = b"fooba";

        let actual_output = encode(input).unwrap();
        assert_eq!(EXPECTED_OUTPUT, &actual_output);
    }

    #[test]
    fn encode_foobar() {
        const EXPECTED_OUTPUT: &[u8] = b"666f6f626172";
        let input: &[u8] = b"foobar";

        let actual_output = encode(input).unwrap();
        assert_eq!(EXPECTED_OUTPUT, &actual_output);
    }
}

use std::str;

const ALPHABET: [u8; 65] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'-', b'_',
    b'=',
];

fn get_first_char(first_octet: u8) -> u8 {
    return ALPHABET[first_octet as usize >> 2 & 0b111111];
}

fn get_second_char(first_octet: u8, second_octet: u8) -> u8 {
    return ALPHABET
        [(((first_octet << 4) & 0b110000) | ((second_octet >> 4) & 0b1111)) as usize & 0b111111];
}

fn get_third_char(second_octet: u8, third_octet: u8) -> u8 {
    return ALPHABET
        [(((second_octet << 2) & 0b111100) | ((third_octet >> 6) & 0b11)) as usize & 0b111111];
}

fn get_fourth_char(third_octet: u8) -> u8 {
    return ALPHABET[third_octet as usize & 0b111111];
}

// +--first octet--+-second octet--+--third octet--+
// |7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|7 6 5 4 3 2 1 0|
// +-----------+---+-------+-------+---+-----------+
// |5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|5 4 3 2 1 0|
// +--1.index--+--2.index--+--3.index--+--4.index--+
fn process_full_group(octets: &[u8], result: &mut Vec<u8>) {
    result.push(get_first_char(octets[0]));
    result.push(get_second_char(octets[0], octets[1]));
    result.push(get_third_char(octets[1], octets[2]));
    result.push(get_fourth_char(octets[2]));
}

// https://datatracker.ietf.org/doc/html/rfc4648
// encode returns a base64 encoding of the given str
// according to RFC-4648
pub fn encode_from_raw_bytes(bytes: &[u8]) -> String {
    let length = bytes.len();
    let mut result: Vec<u8> = Vec::with_capacity(length);

    let num_full_groups = length / 3;
    let mut cur_iteration = 0;
    while cur_iteration < num_full_groups {
        process_full_group(
            &bytes[3 * cur_iteration..3 * cur_iteration + 3],
            &mut result,
        );
        cur_iteration += 1;
    }

    let remaining_octets = length % 3;

    if remaining_octets == 1 {
        result.push(get_first_char(bytes[num_full_groups * 3]));
        result.push(get_second_char(bytes[num_full_groups * 3], 0b0));
        result.push(ALPHABET[64]);
        result.push(ALPHABET[64]);
    }

    if remaining_octets == 2 {
        result.push(get_first_char(bytes[num_full_groups * 3]));
        result.push(get_second_char(
            bytes[num_full_groups * 3],
            bytes[num_full_groups * 3 + 1],
        ));
        result.push(get_third_char(bytes[num_full_groups * 3 + 1], 0b0));
        result.push(ALPHABET[64]);
    }

    let result_str = match str::from_utf8(&result) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
    .to_owned();

    return result_str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use hex;

    #[test]
    fn get_first_char_basic() {
        const EXPECTED_OUTPUT: u8 = b'Z';

        let first_octet: u8 = b'f';

        let actual = get_first_char(first_octet);

        assert_eq!(EXPECTED_OUTPUT, actual);
    }

    #[test]
    fn get_second_char_basic() {
        const EXPECTED_OUTPUT: u8 = b'm';

        let first_octet: u8 = b'f';
        let second_octet: u8 = b'o';

        let actual = get_second_char(first_octet, second_octet);

        assert_eq!(EXPECTED_OUTPUT, actual);
    }

    #[test]
    fn get_third_char_basic() {
        const EXPECTED_OUTPUT: u8 = b'9';

        let second_octet: u8 = b'o';
        let third_octet: u8 = b'o';

        let actual = get_third_char(second_octet, third_octet);

        assert_eq!(EXPECTED_OUTPUT, actual);
    }

    #[test]
    fn get_fourth_char_basic() {
        const EXPECTED_OUTPUT: u8 = b'v';

        let third_octet: u8 = b'o';

        let actual = get_fourth_char(third_octet);
        assert_eq!(EXPECTED_OUTPUT, actual);
    }

    #[test]
    fn process_group_foo() {
        const EXPECTED_OUTPUT: &str = "Zm9v";
        let input: &str = "foo";
        process_group_helper(input, EXPECTED_OUTPUT);
    }

    #[test]
    fn process_group_bar() {
        const EXPECTED_OUTPUT: &str = "YmFy";
        let input: &str = "bar";
        process_group_helper(input, EXPECTED_OUTPUT);
    }

    #[test]
    fn process_encode_empty() {
        const EXPECTED_OUTPUT: &str = "";
        let input: &[u8] = b"";

        let actual_output = encode_from_raw_bytes(input);
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    #[test]
    fn process_encode_f() {
        const EXPECTED_OUTPUT: &str = "Zg==";
        let input: &[u8] = b"f";

        let actual_output = encode_from_raw_bytes(input);
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    #[test]
    fn process_encode_fo() {
        const EXPECTED_OUTPUT: &str = "Zm8=";
        let input: &[u8] = b"fo";

        let actual_output = encode_from_raw_bytes(input);
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    #[test]
    fn process_encode_foo() {
        const EXPECTED_OUTPUT: &str = "Zm9v";
        let input: &[u8] = b"foo";

        let actual_output = encode_from_raw_bytes(input);
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    #[test]
    fn process_encode_foob() {
        const EXPECTED_OUTPUT: &str = "Zm9vYg==";
        let input: &[u8] = b"foob";

        let actual_output = encode_from_raw_bytes(input);
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    #[test]
    fn process_encode_fooba() {
        const EXPECTED_OUTPUT: &str = "Zm9vYmE=";
        let input: &[u8] = b"fooba";

        let actual_output = encode_from_raw_bytes(input);
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    #[test]
    fn process_encode_bar() {
        const EXPECTED_OUTPUT: &str = "YmFy";
        let input: &[u8] = b"bar";

        let actual_output = encode_from_raw_bytes(input);
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    #[test]
    fn process_encode_foobar() {
        const EXPECTED_OUTPUT: &str = "Zm9vYmFy";
        let input: &[u8] = b"foobar";

        let actual_output = encode_from_raw_bytes(input);
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    #[test]
    fn process_encode_cryptopals() {
        const EXPECTED_OUTPUT: &str =
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let input: &[u8] = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

        let binary_input = hex::decode(input).unwrap();

        let actual_output = encode_from_raw_bytes(&binary_input);
        assert_eq!(EXPECTED_OUTPUT, actual_output);
    }

    fn process_group_helper(input: &str, expected_output: &str) {
        let mut result = Vec::with_capacity(4);

        process_full_group(input.as_bytes(), &mut result);

        assert_eq!(4, result.len());
        let result_str = match str::from_utf8(&result) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
        .to_owned();

        assert_eq!(expected_output, result_str)
    }
}

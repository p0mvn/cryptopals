use custom_hex;

// xor performs a binary xor of byte_a and byte_b
// xor is defined as follows:
//  a xor b === (a or b) and !(a and b)
//          === (a or b) and (!a or !b) by De Morgan's Law.
fn xor(byte_a: u8, byte_b: u8) -> u8 {
    return (byte_a | byte_b) & (!byte_a | !byte_b);
}

pub fn compute(hex_a: &[u8], hex_b: &[u8]) -> Result<Vec<u8>, String> {
    if hex_a.len() != hex_b.len() {
        return Err(format!(
            "length of hex_a {} was not equal to length of hex_b {}",
            hex_a.len(),
            hex_b.len()
        ));
    }

    let bytes_a = custom_hex::decode(hex_a)?;
    let bytes_b = custom_hex::decode(hex_b)?;

    let mut result: Vec<u8> = Vec::with_capacity(bytes_a.len());
    for i in 0..bytes_a.len() {
        let new_byte = xor(bytes_a[i], bytes_b[i]);
        result.push(custom_hex::encode_byte(new_byte >> 4 & 0b1111)?);
        result.push(custom_hex::encode_byte(new_byte & 0b1111)?);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_works_cryptopals() {
        const EXPECTED_OUTPUT: &[u8] = b"746865206b696420646f6e277420706c6179";
        let hex_a: &[u8] = b"1c0111001f010100061a024b53535009181c";
        let hex_b: &[u8] = b"686974207468652062756c6c277320657965";

        let actual = compute(hex_a, hex_b).unwrap();

        assert_eq!(EXPECTED_OUTPUT, &actual);
    }

    #[test]
    fn compute_unequal_length_error() {
        let hex_a: &[u8] = b"1c0111001f010100061a024b53535009181";
        let hex_b: &[u8] = b"686974207468652062756c6c277320657965";

        let expected_output = format!(
            "length of hex_a {} was not equal to length of hex_b {}",
            hex_a.len(),
            hex_b.len()
        );

        let err = compute(hex_a, hex_b).unwrap_err();

        assert_eq!(expected_output, err);
    }

    #[test]
    fn xor_works() {
        const EXPECTED_OUTPUT: u8 = 0b01100011;
        let byte_a: u8 = 0b11001110;
        let byte_b: u8 = 0b10101101;

        let actual = xor(byte_a, byte_b);

        assert_eq!(EXPECTED_OUTPUT, actual);
    }
}

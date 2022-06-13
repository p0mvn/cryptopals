pub mod base64;
pub mod hex;

pub fn convert(value: &[u8]) -> String {
    let bytes: &[u8] = &hex::decode(value).unwrap()[..];
    return base64::encode_from_raw_bytes(bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_works_cryptopals() {
        let expected_output: String = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        let input: &[u8] = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

        let actual = convert(input);

        assert_eq!(expected_output, actual);
    }
}

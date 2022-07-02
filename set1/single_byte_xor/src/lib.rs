use fixed_xor;

const MAX_BYTE_VALUE: u8 = !0;

fn fixed_xor_char(result: &mut Vec<u8>, decoded_text: &Vec<u8>, char: u8) {
    for i in 0..decoded_text.len() {
        result[i] = fixed_xor::xor(decoded_text[i], char as u8);
    }
}

pub fn find_key_byte_xor_cipher(encoded: &[u8]) -> Result<(u8, Vec<u8>), String> {
    let mut result_byte: u8 = 0;

    // max_frequency_so_far represents the max number
    // of word characters that a decoded trial has
    // observed so far.
    //
    // The byte that has the largest frequency
    // of word characters when XORed with encoded
    // is likely the decoded text we are looking for.
    let mut max_frequency_so_far: u32 = 0;



    let decoded = custom_hex::decode(encoded)?;

    // Reuse this vec in for loop to avoid reallocation.
    let mut result: Vec<u8> = vec![0; decoded.len()];

    for cur_byte in 0..MAX_BYTE_VALUE {
        // xor
        fixed_xor_char(&mut result, &decoded, cur_byte);

        // calculate frequency of word characters occuring
        let mut cur_frequency = 0;
        for byte_char in &result {
            if byte_char >= &b'a' && byte_char <= &b'z' || byte_char >= &b'A' && byte_char <= &b'Z' {
                cur_frequency +=1
            }
        }

        // if frequency > max_frequency_so_far -> set new result byte
        if cur_frequency > max_frequency_so_far {
            result_byte = cur_byte as u8;
            max_frequency_so_far = cur_frequency;
        }
    }

    // run fixed_xor_char again to update the result with correct value.
    fixed_xor_char(&mut result, &decoded, result_byte);

    Ok((result_byte, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_key_byte_xor_cipher_works_cryptopals() {
        const EXPECTED_BYTE_OUTPUT: u8 = 88;
        const EXPECTED_DECRYPTED_OUTPUT: &str = "Cooking MC's like a pound of bacon";
        let encoded: &[u8] =
            b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let (actual_byte, actual_decoded) = find_key_byte_xor_cipher(encoded).unwrap();

        let s = String::from_utf8_lossy(&actual_decoded[..]);

        assert_eq!(EXPECTED_BYTE_OUTPUT, actual_byte);
        assert_eq!(String::from(EXPECTED_DECRYPTED_OUTPUT),s);
    }
}

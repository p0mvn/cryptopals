use fixed_xor;

const MAX_BYTE_VALUE: u8 = !0;

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

    let mut result_so_far: Vec<u8> = vec![];

    // We reuse this vec in for loop to avoid reallocation.
    let mut key: Vec<u8> = vec![0; encoded.len()];

    for cur_byte in 0..MAX_BYTE_VALUE {
        // create xor pair from cur_byte to try
        // reuse vec to avoid redunant allocations.
        for i in 0..key.len() {
            key[i] = cur_byte;
        }

        // fixed xor
        result_so_far = fixed_xor::compute(encoded, &key)?;

        // calculate frequency of word characters occuring
        let mut cur_frequency = 0;
        for byte_char in &result_so_far {
            if byte_char >= &b'a' && byte_char <= &b'z' || byte_char >= &b'A' && byte_char <= &b'Z' {
                cur_frequency +=1
            }
        }

        // if frequency > max_frequency_so_far -> set new result byte
        if cur_frequency > max_frequency_so_far {
            result_byte = cur_byte;
            max_frequency_so_far = cur_frequency;
        }
    }

    Ok((result_byte, result_so_far))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_key_byte_xor_cipher_works_cryptopals() {
        const EXPECTED_BYTE_OUTPUT: u8 = b'7';
        const EXPECTED_DECRYPTED_OUTPUT: Vec<u8> = vec![];
        let encoded: &[u8] =
            b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let (actual_byte, actual_decoded) = find_key_byte_xor_cipher(encoded).unwrap();

        assert_eq!(EXPECTED_BYTE_OUTPUT, actual_byte);
        assert_eq!(EXPECTED_DECRYPTED_OUTPUT, actual_decoded);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Aes128<'a> {
    key: &'a [u8; 16],
}

impl<'a> Aes128<'a> {
    const SBOX: [u8; 256] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
        0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
        0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
        0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
        0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
        0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
        0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
        0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
        0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
        0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
        0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
        0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ];

    const INV_SBOX: [u8; 256] = [
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7,
        0xfb, 0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde,
        0xe9, 0xcb, 0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42,
        0xfa, 0xc3, 0x4e, 0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49,
        0x6d, 0x8b, 0xd1, 0x25, 0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c,
        0xcc, 0x5d, 0x65, 0xb6, 0x92, 0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15,
        0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84, 0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7,
        0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06, 0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02,
        0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b, 0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc,
        0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73, 0x96, 0xac, 0x74, 0x22, 0xe7, 0xad,
        0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e, 0x47, 0xf1, 0x1a, 0x71, 0x1d,
        0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b, 0xfc, 0x56, 0x3e, 0x4b,
        0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4, 0x1f, 0xdd, 0xa8,
        0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f, 0x60, 0x51,
        0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef, 0xa0,
        0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c,
        0x7d,
    ];

    const R_CONSTANTS: [u8; 11] = [
        0x00, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36,
    ];

    pub fn new(key: &'a [u8; 16]) -> Self {
        Self { key }
    }

    // Encrypts a single block of 16 bytes using AES-128.
    fn aes_encrypt_block(self, input: &[u8; 16], output: &mut [u8; 16]) {
        let mut state = *input;
        let mut expanded_key = [0u8; 176];
        key_expansion(self.key, &mut expanded_key);

        add_blocks(&mut state, &expanded_key[0..16]);

        for round in 1..10 {
            sub_bytes(&mut state);

            shift_rows(&mut state);

            mix_columns(&mut state);

            // Add round key
            add_blocks(&mut state, &expanded_key[round * 16..(round + 1) * 16]);
        }

        sub_bytes(&mut state);
        shift_rows(&mut state);
        add_blocks(&mut state, &expanded_key[160..176]);

        output.copy_from_slice(&state);
    }
    // DECRYPT

    fn aes_decrypt_block(self, input: &[u8; 16], output: &mut [u8; 16]) {
        let mut state = *input;
        let mut expanded_key = [0u8; 176];
        key_expansion(self.key, &mut expanded_key);

        add_blocks(&mut state, &expanded_key[160..176]);

        for round in (1..10).rev() {
            inv_shift_rows(&mut state);
            inv_sub_bytes(&mut state);
            add_blocks(&mut state, &expanded_key[round * 16..(round + 1) * 16]);
            inv_mix_columns(&mut state);
        }

        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_blocks(&mut state, &expanded_key[0..16]);

        output.copy_from_slice(&state);
    }

    // CBC mode
    pub fn aes_encrypt_blocks(
        self,
        plaintext: &Vec<u8>,
        iv: &[u8; 16],
    ) -> Result<Vec<u8>, &'static str> {
        if plaintext.len() % 16 != 0 {
            return Err("The plaintext isn't a multiple of 16 bits");
        }

        let mut ciphertext: Vec<u8> = Vec::new();
        let mut last_block = *iv;
        let num_blocks = plaintext.len() / 16;

        for block_index in 0..num_blocks {
            let mut block: [u8; 16] = [0; 16];

            for a in 0..16 {
                block[a] = plaintext[(16 * block_index) + a]
            }

            // xor blocks together
            add_blocks(&mut block, &last_block);
            // encrypt block
            self.aes_encrypt_block(&block.clone(), &mut block);
            // last_block = encrypted block
            last_block = block;

            for b in block {
                ciphertext.push(b);
            }
        }

        Ok(ciphertext)
    }

    pub fn aes_decrypt_blocks(
        self,
        ciphertext: &Vec<u8>,
        iv: &[u8; 16],
    ) -> Result<Vec<u8>, &'static str> {
        if ciphertext.len() % 16 != 0 {
            return Err("The plaintext isn't a multiple of 16 bits");
        }

        let num_blocks = ciphertext.len() / 16;
        let mut last = iv.clone();
        let mut plaintext = vec![];

        for block_index in 0..num_blocks {
            let mut block: [u8; 16] = [0; 16];

            for a in 0..16 {
                block[a] = ciphertext[(16 * block_index) + a]
            }

            let xor = last;
            last = block;

            // decrypt block
            self.aes_decrypt_block(&block.clone(), &mut block);
            // xor blocks together
            add_blocks(&mut block, &xor);

            for b in block {
                plaintext.push(b);
            }
        }

        Ok(plaintext)
    }

    pub fn aes_ctr_mode(
        self,
        data: &Vec<u8>,
        iv: &[u8; 16],
    ) -> Result<Vec<u8>, &'static str> {
        let mut ciphertext: Vec<u8> = Vec::new();
        let mut counter = *iv;

        for chunk  in data.chunks(16) {
            // Encrypt the counter
            let mut block = counter;
            self.aes_encrypt_block(&block.clone(), &mut block);

            // XOR the encrypted counter with the chunk
            for (i, byte) in chunk.iter().enumerate() {
                ciphertext.push(byte ^ block[i]);
            }

            // Increment the counter (big-endian increment)
            for i in (0..16).rev() {
                if counter[i] == 0xFF {
                    counter[i] = 0x00;
                } else {
                    counter[i] += 1;
                    break;
                }
            }
        }

        Ok(ciphertext)
    }
}

fn sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = Aes128::SBOX[*byte as usize];
    }
}

fn shift_rows(state: &mut [u8; 16]) {
    let mut temp = [0u8; 16];
    temp.copy_from_slice(state);

    // column 0
    state[0] = temp[0];
    state[1] = temp[5];
    state[2] = temp[10];
    state[3] = temp[15];

    // column 1
    state[4] = temp[4];
    state[5] = temp[9];
    state[6] = temp[14];
    state[7] = temp[3];

    // column 2
    state[8] = temp[8];
    state[9] = temp[13];
    state[10] = temp[2];
    state[11] = temp[7];

    // column 3
    state[12] = temp[12];
    state[13] = temp[1];
    state[14] = temp[6];
    state[15] = temp[11];
}

fn mix_columns(state: &mut [u8; 16]) {
    let temp = *state;

    // column 0
    state[0] = galois_mult(temp[0], 0x02) ^ galois_mult(temp[1], 0x03) ^ temp[2] ^ temp[3];
    state[1] = temp[0] ^ galois_mult(temp[1], 0x02) ^ galois_mult(temp[2], 0x03) ^ temp[3];
    state[2] = temp[0] ^ temp[1] ^ galois_mult(temp[2], 0x02) ^ galois_mult(temp[3], 0x03);
    state[3] = galois_mult(temp[0], 0x03) ^ temp[1] ^ temp[2] ^ galois_mult(temp[3], 0x02);

    // column 1
    state[4] = galois_mult(temp[4], 0x02) ^ galois_mult(temp[5], 0x03) ^ temp[6] ^ temp[7];
    state[5] = temp[4] ^ galois_mult(temp[5], 0x02) ^ galois_mult(temp[6], 0x03) ^ temp[7];
    state[6] = temp[4] ^ temp[5] ^ galois_mult(temp[6], 0x02) ^ galois_mult(temp[7], 0x03);
    state[7] = galois_mult(temp[4], 0x03) ^ temp[5] ^ temp[6] ^ galois_mult(temp[7], 0x02);

    // column 2
    state[8] = galois_mult(temp[8], 0x02) ^ galois_mult(temp[9], 0x03) ^ temp[10] ^ temp[11];
    state[9] = temp[8] ^ galois_mult(temp[9], 0x02) ^ galois_mult(temp[10], 0x03) ^ temp[11];
    state[10] = temp[8] ^ temp[9] ^ galois_mult(temp[10], 0x02) ^ galois_mult(temp[11], 0x03);
    state[11] = galois_mult(temp[8], 0x03) ^ temp[9] ^ temp[10] ^ galois_mult(temp[11], 0x02);

    // column 3
    state[12] = galois_mult(temp[12], 0x02) ^ galois_mult(temp[13], 0x03) ^ temp[14] ^ temp[15];
    state[13] = temp[12] ^ galois_mult(temp[13], 0x02) ^ galois_mult(temp[14], 0x03) ^ temp[15];
    state[14] = temp[12] ^ temp[13] ^ galois_mult(temp[14], 0x02) ^ galois_mult(temp[15], 0x03);
    state[15] = galois_mult(temp[12], 0x03) ^ temp[13] ^ temp[14] ^ galois_mult(temp[15], 0x02);
}

// Expands the key into multiple round keys.
// Nk = 4 as key = 128
// 10 passes * 16 bytes + 16 bytes = 176
fn key_expansion(key: &[u8; 16], expanded_key: &mut [u8; 176]) {
    // first 16 bits are the original key
    expanded_key[0..16].copy_from_slice(key);

    let mut i = 16;
    let mut temp = [0u8; 4];

    while i < 176 {
        temp.copy_from_slice(&expanded_key[i - 4..i]);

        if i % 16 == 0 {
            // Rotate left
            temp.rotate_left(1);
            // Substitute bytes using S-box
            for j in 0..4 {
                temp[j] = Aes128::SBOX[temp[j] as usize];
            }
            // XOR with round constant
            temp[0] ^= Aes128::R_CONSTANTS[i / 16];
        }

        for j in 0..4 {
            expanded_key[i] = expanded_key[i - 16] ^ temp[j];
            i += 1;
        }
    }
}

fn inv_sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = Aes128::INV_SBOX[*byte as usize];
    }
}

fn inv_shift_rows(state: &mut [u8; 16]) {
    let mut temp = [0u8; 16];
    temp.copy_from_slice(state);

    state[0] = temp[0];
    state[4] = temp[4];
    state[8] = temp[8];
    state[12] = temp[12];

    state[1] = temp[13];
    state[2] = temp[10];
    state[3] = temp[7];
    state[5] = temp[1];
    state[6] = temp[14];
    state[7] = temp[11];

    state[9] = temp[5];
    state[10] = temp[2];
    state[11] = temp[15];

    state[13] = temp[9];
    state[14] = temp[6];
    state[15] = temp[3];
}

fn inv_mix_columns(state: &mut [u8; 16]) {
    let temp = *state;

    state[0] = galois_mult(temp[0], 0x0e)
        ^ galois_mult(temp[1], 0x0b)
        ^ galois_mult(temp[2], 0x0d)
        ^ galois_mult(temp[3], 0x09);
    state[1] = galois_mult(temp[0], 0x09)
        ^ galois_mult(temp[1], 0x0e)
        ^ galois_mult(temp[2], 0x0b)
        ^ galois_mult(temp[3], 0x0d);
    state[2] = galois_mult(temp[0], 0x0d)
        ^ galois_mult(temp[1], 0x09)
        ^ galois_mult(temp[2], 0x0e)
        ^ galois_mult(temp[3], 0x0b);
    state[3] = galois_mult(temp[0], 0x0b)
        ^ galois_mult(temp[1], 0x0d)
        ^ galois_mult(temp[2], 0x09)
        ^ galois_mult(temp[3], 0x0e);

    state[4] = galois_mult(temp[4], 0x0e)
        ^ galois_mult(temp[5], 0x0b)
        ^ galois_mult(temp[6], 0x0d)
        ^ galois_mult(temp[7], 0x09);
    state[5] = galois_mult(temp[4], 0x09)
        ^ galois_mult(temp[5], 0x0e)
        ^ galois_mult(temp[6], 0x0b)
        ^ galois_mult(temp[7], 0x0d);
    state[6] = galois_mult(temp[4], 0x0d)
        ^ galois_mult(temp[5], 0x09)
        ^ galois_mult(temp[6], 0x0e)
        ^ galois_mult(temp[7], 0x0b);
    state[7] = galois_mult(temp[4], 0x0b)
        ^ galois_mult(temp[5], 0x0d)
        ^ galois_mult(temp[6], 0x09)
        ^ galois_mult(temp[7], 0x0e);

    state[8] = galois_mult(temp[8], 0x0e)
        ^ galois_mult(temp[9], 0x0b)
        ^ galois_mult(temp[10], 0x0d)
        ^ galois_mult(temp[11], 0x09);
    state[9] = galois_mult(temp[8], 0x09)
        ^ galois_mult(temp[9], 0x0e)
        ^ galois_mult(temp[10], 0x0b)
        ^ galois_mult(temp[11], 0x0d);
    state[10] = galois_mult(temp[8], 0x0d)
        ^ galois_mult(temp[9], 0x09)
        ^ galois_mult(temp[10], 0x0e)
        ^ galois_mult(temp[11], 0x0b);
    state[11] = galois_mult(temp[8], 0x0b)
        ^ galois_mult(temp[9], 0x0d)
        ^ galois_mult(temp[10], 0x09)
        ^ galois_mult(temp[11], 0x0e);

    state[12] = galois_mult(temp[12], 0x0e)
        ^ galois_mult(temp[13], 0x0b)
        ^ galois_mult(temp[14], 0x0d)
        ^ galois_mult(temp[15], 0x09);
    state[13] = galois_mult(temp[12], 0x09)
        ^ galois_mult(temp[13], 0x0e)
        ^ galois_mult(temp[14], 0x0b)
        ^ galois_mult(temp[15], 0x0d);
    state[14] = galois_mult(temp[12], 0x0d)
        ^ galois_mult(temp[13], 0x09)
        ^ galois_mult(temp[14], 0x0e)
        ^ galois_mult(temp[15], 0x0b);
    state[15] = galois_mult(temp[12], 0x0b)
        ^ galois_mult(temp[13], 0x0d)
        ^ galois_mult(temp[14], 0x09)
        ^ galois_mult(temp[15], 0x0e);
}

// UTILITY FUNCTIONS
pub fn add_blocks(state: &mut [u8; 16], b: &[u8]) {
    for i in 0..16 {
        state[i] ^= b[i];
    }
}

pub fn galois_mult(a: u8, b: u8) -> u8 {
    let mut result: u8 = 0;
    let mut a = a;
    let mut b = b; 

    // Irreducible polynomial for GF(2^8)
    const IRREDUCIBLE_POLY: u8 = 0x1b; // (x^8) + x^4 + x^3 + x + 1

    // Process each bit of the second operand
    while b != 0 {
        // If the least significant bit of b is 1, add the current a to the result
        if (b & 1) != 0 {
            result ^= a; // XOR is used instead of addition in GF(2^8)
        }

        // Shift a to the left, which corresponds to multiplying by x in GF(2^8)
        let high_bit_set = (a & 0x80) != 0; // Check if the high bit (x^7) is set
        a <<= 1; // Multiply a by x

        // If the high bit was set before shifting, reduce a modulo the irreducible polynomial
        if high_bit_set {
            a ^= IRREDUCIBLE_POLY; // Perform the reduction
        }

        // Shift b to the right, moving to the next bit
        b >>= 1;
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cbc() {
        let key: [u8; 16] = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let iv: [u8; 16] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F,
        ];
        let expected_plaintext: Vec<u8> = vec![
            0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93,
            0x17, 0x2a, 0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11,
            0x73, 0x93, 0x17, 0x2a, 0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d,
            0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a, 0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96,
            0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a, 0x6b, 0xc1, 0xbe, 0xe2, 0x2e, 0x40,
            0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93, 0x17, 0x2a,
        ];
        let expected_ciphertext: Vec<u8> = vec![
            0x76, 0x49, 0xab, 0xac, 0x81, 0x19, 0xb2, 0x46, 0xce, 0xe9, 0x8e, 0x9b, 0x12, 0xe9,
            0x19, 0x7d, 0x4c, 0xbb, 0xc8, 0x58, 0x75, 0x6b, 0x35, 0x81, 0x25, 0x52, 0x9e, 0x96,
            0x98, 0xa3, 0x8f, 0x44, 0x9f, 0x6f, 0x07, 0x96, 0xee, 0x3e, 0x47, 0xb0, 0xd8, 0x7c,
            0x76, 0x1b, 0x20, 0x52, 0x7f, 0x78, 0x07, 0x01, 0x34, 0x08, 0x5f, 0x02, 0x75, 0x17,
            0x55, 0xef, 0xca, 0x3b, 0x4c, 0xdc, 0x7d, 0x62, 0x1d, 0x93, 0x10, 0xca, 0xac, 0x69,
            0xe1, 0xff, 0xee, 0xe0, 0x71, 0x20, 0x25, 0x02, 0xfa, 0x70,
        ];

        let cipher = Aes128::new(&key);

        let ciphertext = cipher.aes_encrypt_blocks(&expected_plaintext, &iv).unwrap();
        let plaintext = cipher.aes_decrypt_blocks(&ciphertext, &iv).unwrap();

        assert_eq!(ciphertext, expected_ciphertext);
        assert_eq!(plaintext, expected_plaintext);
    }

    #[test]
    fn test_ctr() {
        let key = [54, 241, 131, 87, 190, 77, 189, 119, 240, 80, 81, 92, 115, 252, 249, 242];
        let expected_ciphertext = [228, 98, 24, 192, 165, 60, 190, 202, 105, 90, 228, 95, 170, 137, 82, 170, 14, 49, 27, 222, 157, 78, 1, 114, 109, 49, 132, 195, 68, 81];
        let expected_plaintext = [65, 108, 119, 97, 121, 115, 32, 97, 118, 111, 105, 100, 32, 116, 104, 101, 32, 116, 119, 111, 32, 116, 105, 109, 101, 32, 112, 97, 100, 33];
        let iv = [119, 11, 128, 37, 158, 195, 59, 235, 37, 97, 53, 138, 159, 45, 198, 23];

        let cipher = Aes128::new(&key);

        let plaintext = cipher.aes_ctr_mode(&expected_ciphertext.to_vec(), &iv).unwrap();
        let ciphertext = cipher.aes_ctr_mode(&plaintext.to_vec(), &iv).unwrap();

        assert_eq!(plaintext, expected_plaintext);
        assert_eq!(ciphertext, expected_ciphertext);
    }
}
// Cryptography module, contains functions for dealing with cryptography.

// Create a function to read in a string of hex and convert it to a byte array.
pub fn hex_string_to_bytes(hex: &str) -> Vec<u8> {
    // We need to literally read the string in as a byte array.
    // So for example if the string is: FF, we need to put that into a byte array.

    // Create a new vector to store the bytes in.
    let mut bytes: Vec<u8> = Vec::new();

    // If the string has an odd number of characters, add a 0 to the front.
    let mut hex = hex.to_string();
    if hex.len() % 2 != 0 {
        hex = format!("0{}", hex);
    }

    // Loop through the string, two characters at a time.
    for i in (0..hex.len()).step_by(2) {
        // Get the two characters.
        let hex_byte = &hex[i..i + 2];

        // Convert the two characters to a byte.
        let byte = u8::from_str_radix(hex_byte, 16).unwrap();

        // Add the byte to the vector.
        bytes.push(byte);
    }

    // Return the vector.
    bytes
}

// Function to print a byte array as a hex string.
pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    // Create a new string to store the hex in.
    let mut hex = String::new();

    // Loop through the bytes.
    for byte in bytes {
        // Convert the byte to a hex string.
        let hex_byte = format!("{:02x}", byte);

        // Add the hex string to the string.
        hex.push_str(&hex_byte);
    }

    // Return the string.
    hex
}

// XOR two byte arrays together.
pub fn xor_bytes(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    // Create a new vector to store the bytes in.
    let mut bytes: Vec<u8> = Vec::new();

    // Loop through the bytes.
    for i in 0..bytes1.len() {
        // XOR the bytes together.
        let byte = bytes1[i] ^ bytes2[i];

        // Add the byte to the vector.
        bytes.push(byte);
    }

    // Return the vector.
    bytes
}

// Function that will take a byte array and encode it into a base64 string.
pub fn bytes_to_base64(bytes: &Vec<u8>) -> String {
    // Create a new string to store the base64 string in.
    let mut base64 = String::new();
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // Loop through the bytes, three at a time.
    for i in (0..bytes.len()).step_by(3) {
        // Get the three bytes.
        let byte1 = bytes[i];
        let byte2 = bytes[i + 1];
        let byte3 = bytes[i + 2];

        // Get the first 6 bits of the first byte.
        let first_6_bits = byte1 >> 2;

        // Get the last 2 bits of the first byte and the first 4 bits of the second byte.
        let last_2_bits_first_byte = byte1 & 0b00000011;
        let first_4_bits_second_byte = byte2 >> 4;

        // Get the last 4 bits of the second byte and the first 2 bits of the third byte.
        let last_4_bits_second_byte = byte2 & 0b00001111;
        let first_2_bits_third_byte = byte3 >> 6;

        // Get the last 6 bits of the third byte.
        let last_6_bits_third_byte = byte3 & 0b00111111;

        // Add the characters to the base64 string.
        base64.push(base64_chars.chars().nth(first_6_bits as usize).unwrap());
        base64.push(
            base64_chars
                .chars()
                .nth(((last_2_bits_first_byte << 4) | first_4_bits_second_byte) as usize)
                .unwrap(),
        );

        base64.push(
            base64_chars
                .chars()
                .nth(((last_4_bits_second_byte << 2) | first_2_bits_third_byte) as usize)
                .unwrap(),
        );

        base64.push(
            base64_chars
                .chars()
                .nth(last_6_bits_third_byte as usize)
                .unwrap(),
        );
    }

    // Return the base64 string.
    base64
}

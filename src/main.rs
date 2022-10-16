// Main file for the project, run various tests/challenges and return the results.

// Import the modules
mod cryptography;
mod utility;

// Import the functions
use cryptography::*;
use utility::*;

// Main function.
fn main() {
    // Run the tests.
    run_tests();
}

// Run the tests.
fn run_tests() {
    // Run stuff in Set 1.
    run_set_1();
}

// Run stuff in Set 1.
fn run_set_1() {
    // Challenge 1.
    println!(
        "Set 1, Challenge 1: {}", 
        bytes_to_base64(&string_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"))
    );

    // Challenge 2.
    println!(
        "Set 1, Challenge 2: {}",
        bytes_to_hex_string(&xor(
            &hex_string_to_bytes("1c0111001f010100061a024b53535009181c"),
            &hex_string_to_bytes("686974207468652062756c6c277320657965")
        ))
    );

    // Challenge 3.
    // Need to find the key that produces the most readable string, single byte XOR cipher.
    let mut best_score: f32 = 0.0;
    let mut best_string = String::new();

    // Loop through all the possible keys.
    for key in 0..255 {
        // Get the bytes from the hex string.
        let bytes = hex_string_to_bytes(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        );

        // Use xor function to decode the bytes.
        let decoded_bytes = xor(&bytes, &[key]);

        // Convert the bytes to a string.
        let decoded_string = bytes_to_string(&decoded_bytes);

        // Get the score for the string.
        let score = score_english_text(&decoded_string);

        // Check if the score is better than the best score.
        if score > best_score {
            // Update the best score and key.
            best_score = score;
            best_string = decoded_string;
        }
    }

    // Print the key and string.
    println!("Set 1, Challenge 3: {}", best_string);

    // Challenge 5.
    println!(
        "Set 1, Challenge 5: {}",
        bytes_to_hex_string(&xor(
            &string_to_bytes(
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
            ),
            &string_to_bytes("ICE")
        ))
    );
}

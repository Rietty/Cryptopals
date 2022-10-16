// Main file for the project, run various tests/challenges and return the results.

// Import the modules
mod cryptography;

// Import the functions
use cryptography::*;

// Main function.
fn main() {
    // Run the tests.
    run_tests();
}

// Run the tests.
fn run_tests() {
    println!(
        "Set 1, Challenge 1: {}", 
        bytes_to_base64(&hex_string_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"))
    );

    println!(
        "Set 1, Challenge 2: {}",
        bytes_to_hex_string(&xor_bytes(
            &hex_string_to_bytes("1c0111001f010100061a024b53535009181c"),
            &hex_string_to_bytes("686974207468652062756c6c277320657965")
        ))
    );
}

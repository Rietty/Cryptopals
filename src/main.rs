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
    // Set 1, Challenge 1.
    println!("Set 1, Challenge 1: {}", bytes_to_base64(&hex_string_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")));
}

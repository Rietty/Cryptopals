// Utility module, contains functions that are not strictly cryptography related, instead they provide other functionality.

// This function will take a character and return the frequency of that character in english text.
// Based off wikipedia: https://en.wikipedia.org/wiki/Letter_frequency
fn get_character_frequency(character: char) -> f32 {
    // Create a new f32 variable to store the frequency in.
    #[allow(unused_assignments)]
    let mut frequency = 0.0;

    // Check the character.
    match character {
        'a' | 'A' => frequency = 0.08167,
        'b' | 'B' => frequency = 0.01492,
        'c' | 'C' => frequency = 0.02782,
        'd' | 'D' => frequency = 0.04253,
        'e' | 'E' => frequency = 0.12702,
        'f' | 'F' => frequency = 0.02228,
        'g' | 'G' => frequency = 0.02015,
        'h' | 'H' => frequency = 0.06094,
        'i' | 'I' => frequency = 0.06966,
        'j' | 'J' => frequency = 0.00153,
        'k' | 'K' => frequency = 0.00772,
        'l' | 'L' => frequency = 0.04025,
        'm' | 'M' => frequency = 0.02406,
        'n' | 'N' => frequency = 0.06749,
        'o' | 'O' => frequency = 0.07507,
        'p' | 'P' => frequency = 0.01929,
        'q' | 'Q' => frequency = 0.00095,
        'r' | 'R' => frequency = 0.05987,
        's' | 'S' => frequency = 0.06327,
        't' | 'T' => frequency = 0.09056,
        'u' | 'U' => frequency = 0.02758,
        'v' | 'V' => frequency = 0.00978,
        'w' | 'W' => frequency = 0.02360,
        'x' | 'X' => frequency = 0.00150,
        'y' | 'Y' => frequency = 0.01974,
        'z' | 'Z' => frequency = 0.00074,
        ' ' => frequency = 0.13000,
        _ => frequency = 0.0,
    }

    // Return the frequency.
    frequency
}

// This function will take a string of english text and return a score based on character frequency.
pub fn score_english_text(text: &str) -> f32 {
    // Create a new score variable.
    let mut score = 0.0;

    // Loop through the string.
    for character in text.chars() {
        // Get the character frequency.
        let frequency = get_character_frequency(character);

        // Add the frequency to the score.
        score += frequency;
    }

    // Return the score.
    score
}

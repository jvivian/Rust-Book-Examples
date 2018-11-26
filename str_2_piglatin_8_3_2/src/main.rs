/// Convert strings to pig latin. The first consonant of each word is moved to the end of
/// the word and “ay” is added, so “first” becomes “irst-fay.”
/// Words that start with a vowel have “hay” added to the end instead
/// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
    // Get input
    println!("Enter a string to convert to pig-latin");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to get input");

    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for (i, c) in input.chars().enumerate() {
        if vowels.contains(&c) {
            let pre = &input[i..].trim();
            let post = &input[..i];
            if post.len() == 0 {
                println!("{}-hay", pre);
                break
            } else {
                println!("{}-{}ay", pre, post);
                break
            }
        }
    }
}

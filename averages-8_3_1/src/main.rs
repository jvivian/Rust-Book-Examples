/// https://doc.rust-lang.org/book/2018-edition/ch08-03-hash-maps.html
/// Given a list of integers, use a vector and return the mean (the average value),
/// median (when sorted, the value in the middle position), and mode
/// (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::io;

fn main() {
    // Read input from user
    println!("Enter a series of numbers separated by a space");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    // Convert input to series of integers
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|w| w.parse().ok()).collect();

    println!("Input vector is {:?}", nums);

    // Calculate mean of numbers
    let sum: i32 = nums.iter().sum();
    let mean = sum / nums.len() as i32;

    // Calculate median
    let med_index = nums.len() / 2;
    nums.sort();
    let median: i32 = nums[med_index];

    // Calculate mode
    let mut mode_counts = HashMap::new();
    let mut max_val = 0;
    for n in &nums {
        let count = mode_counts.entry(n).or_insert(0);
        *count += 1;
        if *count > max_val {
            max_val = *count;
        }
    }
    // Find key that matches mode value
    let mut mode: i32 = 0;
    for (key, val) in mode_counts {
        if val == max_val {
            mode = *key;
        }
    }

    let output = vec![mean, median, mode];
    println!("Mean/Median/Mode is {:?}", output)
}

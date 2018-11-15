use std::io;

fn main() {
    loop {
        // Acquire value from user via stdin
        println!("Please enter a farenheit value: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Parse string to float, continuing loop if not parseable
        let input: f64 = match input.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Enter a valid number, not '{}'\n", input.trim());
                continue;
            }
        };

        // Compute and print value in celsius
        let c = ((input - 32.0) * 5.0) / 9.0;
        println!("Your value in celsius is: {}", c);
        break;
    }
}

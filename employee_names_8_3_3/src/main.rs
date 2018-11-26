/// Using a hash map and vectors, create a text interface to allow a user to add employee names
/// to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
/// Then let the user retrieve a list of all people in a department or all people in the company
/// by department, sorted alphabetically.

use std::io;
use std::collections::HashMap;
use std::vec::Vec;

fn main() {
    //  Get user input
    let mut map = HashMap::new();
    println!("Add users to database via: Add <name> to <dept>. Type exit to move to next section");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        // Collect database information
        if input.trim() == "exit" {
            break;
        } else {
            let split: Vec<&str> = input
                .split_whitespace()
                .collect();

            // Store in Hashmap
            if split.len() != 4 {
                println!("Must follow database scheme: Add <name> to <dept>");
                continue
            }
            let name = split[1].to_string();
            let dept = split[3].to_string();
            let dept_list = map.entry(dept).or_insert(Vec::new());
            dept_list.push(name);
        }

        println!("{:?}", map);
    }

    // Print out department information
    println!("Type a department or else will print all people in company alphabetically");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");

    choice = choice.trim().to_string();

    if map.contains_key(&choice) {
        let mut people = map[&choice].clone();
        people.sort();

        println!("All people in department {}, sorted alphabetically", choice);
        println!("{}", people.join(", "));
    } else {
        for (key, val) in map.iter() {
            let mut people = val.clone();
            people.sort();
            for p in people {
                println!("Dept: {}\tPerson: {}", key, p);
            }
        }
    }
}

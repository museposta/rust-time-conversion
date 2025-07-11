use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeConversion(s: &str) -> String {
    // Split the string into parts
    let period = &s[s.len()-2..]; // AM or PM
    let time_part = &s[..s.len()-2];
    let mut time_parts: Vec<String> = time_part.split(':').map(|s| s.to_string()).collect();
    
    // Parse hours
    let mut hours: u8 = time_parts[0].parse().unwrap();
    
    // Convert based on AM/PM
    match period {
        "AM" => {
            if hours == 12 {
                hours = 0;
            }
        },
        "PM" => {
            if hours != 12 {
                hours += 12;
            }
        },
        _ => panic!("Invalid period"),
    }
    
    // Update hours part (now we can store the String directly)
    time_parts[0] = format!("{:02}", hours);
    
    // Rejoin all parts
    time_parts.join(":")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
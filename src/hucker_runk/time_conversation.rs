use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn time_conversion(s: &str) -> String {
    let part_of_day = &s[8..=9];
    let hour_str = &s[..2];
    let mut hour: i32 = hour_str.parse().expect("Failed to parse hour");

    if part_of_day == "AM" {
        if hour == 12 {
            hour = 0;
        }
    } else {
        if hour != 12 {
            hour += 12;
        }
    }
    format!("{:02}{}", hour, &s[2..8])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
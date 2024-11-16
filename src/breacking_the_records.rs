use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'breakingRecords' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut count_records = [0, 0];
    let mut count_max = &scores[0];
    let mut count_min = &scores[0];

    for score in scores {
        if score > count_max {
            count_records[0] += 1;
            count_max = score;
        }

        if score < count_min {
            count_records[1] += 1;
            count_min = score;
        }
    }

    count_records.to_vec()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
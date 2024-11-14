use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn min_max_sum(arr: &[i32]) {
    let mut new_arr = arr.to_vec();
    new_arr.sort();
    let mut min_sum: i64 = 0;
    let mut max_sum: i64 = 0;
    for i in 0..new_arr.len() - 1 {
        min_sum += new_arr[i] as i64;
    }
    for i in 1..new_arr.len() {
        max_sum += new_arr[i] as i64;
    }

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    min_max_sum(&arr);
}

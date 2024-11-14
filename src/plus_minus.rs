use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let mut positive = 0;
    let mut negative = 0;
    let mut zeros = 0;

    for index in 0..arr.len() {
        if arr[index] > 0 {
            positive += 1;
        } else if arr[index] == 0 {
            zeros += 1;
        } else {
            negative += 1;
        }
    }

    let pos_prop: f32 = (positive as f32 / arr.len() as f32 * 1000000.).round() / 1000000.;
    let neg_prop: f32 = (negative as f32 / arr.len() as f32 * 1000000.).round() / 1000000.;
    let zero_prop: f32 = (zeros as f32 / arr.len() as f32 * 1000000.).round() / 1000000.;

    println!("{}", pos_prop);
    println!("{}", neg_prop);
    println!("{}", zero_prop)

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}
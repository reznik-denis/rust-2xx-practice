use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::convert::TryInto;

/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */

fn sock_merchant(n: usize, ar: &[i32]) -> i32 {
    let mut used_values = Vec::new();
    let mut count = 0;

    for i in 0..n {
        if used_values.contains(&ar[i]) {
            continue;
        } else {
            let filtered = ar.iter().filter(|&&x| x == ar[i]).count();
            count += filtered / 2;
            used_values.push(ar[i]);
        }
    }

    count as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(n.try_into().unwrap(), &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
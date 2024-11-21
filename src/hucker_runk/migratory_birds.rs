use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut type_of_birds = 0;
    let mut max_count = 0;

    for bird in 1..=5 {
        let count_some_type = arr.iter().filter(|&&x| x == bird).count();
        if count_some_type > max_count || (count_some_type == max_count && bird < type_of_birds) {
            type_of_birds = bird;
            max_count = count_some_type;
        }
    }

    type_of_birds
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratory_birds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

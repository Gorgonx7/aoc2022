use std::fs;
fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Unable to open");
    let split = contents.split("\n");
    let mut current_max = 0;
    let mut current_second = 0;
    let mut current_third = 0;
    let mut total = 0;
    for s in split {
        if s == "" {
            if current_max < total {
                current_third = current_second;
                current_second = current_max;
                current_max = total;
            } else if current_second < total {
                current_third = current_second;
                current_second = total;
            } else if current_third < total {
                current_third = total;
            }
            total = 0;
            continue;
        }
        total += s.parse::<i32>().unwrap();
    }
    let total_total = current_max + current_second + current_third;
    println!("Total: {}", total_total);
}

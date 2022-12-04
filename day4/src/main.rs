use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");
    let total = get_total( input);
    println!("{}", total)
}

fn get_total(contents: String) -> i32 {
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut total = 0;
    for line in lines {
        let pairs: Vec<&str> = line.split(",").collect();
        let first_pair: Vec<&str> = pairs[0].split("-").collect();
        let second_pair: Vec<&str> = pairs[1].split("-").collect();
        let start_1: i32 = first_pair[0].to_string().trim().parse().unwrap();
        let end_1: i32 = first_pair[1].to_string().trim().parse().unwrap();
        let start_2: i32 = second_pair[0].to_string().trim().parse().unwrap();
        let end_2: i32 = second_pair[1].to_string().trim().parse().unwrap();
        if contains(start_1, end_1, start_2, end_2) || 
            contains(start_2, end_2, start_1, end_1){
        println!("start 1: {}, end 1: {}, start 2: {}, end 2: {}", first_pair[0], first_pair[1], second_pair[0], second_pair[1]);

            total += 1;
        } 
    }
    total
}

fn contains(start1: i32, end1: i32, start2: i32, end2: i32) -> bool{
    for i in start1..end1 + 1{
        for j in start2..end2 + 1 {
            if i == j {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::{contains, get_total};
    #[test]
    fn test_contains(){
        let start_1 = 1;
        let start_2: i32 = 3;
        let end_1 = 3;
        let end_2: i32 = 3;
        let want = true;
        let mut got = contains(start_1, end_1, start_2, end_2);
        assert_eq!(want, got);
        got = contains(start_2, end_2, start_1, end_1);
        assert_eq!(true, got);
    }
    #[test]
    fn test_overlaps(){
        let start_1 = 1;
        let end_1: i32 = 3;
        let start_2: i32 = 3;
        let end_2: i32 = 6;
        let want = true;
        let mut got = contains(start_1, end_1, start_2, end_2);
        assert_eq!(want, got);
        got = contains(start_2, end_2, start_1, end_1);
        assert_eq!(true, got);
    }
    #[test]
    fn test_get_total(){
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
        let want = 4;
        let got = get_total(input.to_string());
        assert_eq!(want, got);
    }
}
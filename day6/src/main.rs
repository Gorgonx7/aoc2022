use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Unable to open");
    let char_vec: Vec<char> = contents.chars().collect();
    let mut i = 13;

    while i < char_vec.len() {
        i += 1;
        let mut dictionary: HashMap<char, bool> = HashMap::new();
        dictionary.insert(char_vec[i - 14], true);
        dictionary.insert(char_vec[i - 13], true);
        dictionary.insert(char_vec[i - 12], true);
        dictionary.insert(char_vec[i - 11], true);
        dictionary.insert(char_vec[i - 10], true);
        dictionary.insert(char_vec[i - 9], true);
        dictionary.insert(char_vec[i - 8], true);
        dictionary.insert(char_vec[i - 7], true);
        dictionary.insert(char_vec[i - 6], true);
        dictionary.insert(char_vec[i - 5], true);
        dictionary.insert(char_vec[i - 4], true);
        dictionary.insert(char_vec[i - 3], true);
        dictionary.insert(char_vec[i - 2], true);
        dictionary.insert(char_vec[i - 1], true);
        if dictionary.len() == 14 {
            break;
        }
    }
    println!("{}", i);
}

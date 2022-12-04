use std::{fs, collections::HashMap};
fn main() {
    let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");
    //println!("{}", contents);
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut total = 0;
    let mut x = 0;
    while x < lines.len(){
        let result = get_same_character(lines[x], lines[x+1], lines[x+2]);
        total += get_character_value(result);
        x += 3;
    }
    println!("Total: {}",total)
}


fn get_same_character(first_slot: &str, second_slot: &str,third_slot: &str) -> char{
    let mut first_dictionary = HashMap::new();
    let mut second_dictionary = HashMap::new();


    for first_char in first_slot.chars() {
        first_dictionary.insert(first_char, true);   
    }
    for second_char in second_slot.chars() {
       if first_dictionary.contains_key(&second_char){
            second_dictionary.insert(second_char, true);
       }
    }
    for third_char in third_slot.chars() {
        if second_dictionary.contains_key(&third_char){
            return third_char
        }
    }
    'a'
}

fn get_character_value(char_in: char) -> i32 {
    if char_in.is_ascii_lowercase() {
        return char_in as i32 - 96
    }
    return char_in as i32 - 38
}

#[cfg(test)]
mod tests {
    use crate::{get_same_character, get_character_value};
    #[test]
    fn test_get_same_character(){
        let first_input = "ABcDE";
        let second_input = "eFGHc";
        let third_input = "IJcKLMNO";
        let want = 'c';
        let got = get_same_character(first_input, second_input, third_input);
        assert_eq!(want, got);
    }
    #[test]
    fn test_get_char_value_lower_case(){
        let input = 'a';
        let want = 1;
        let got = get_character_value(input);
        assert_eq!(want, got);
    }
    #[test]
    fn test_get_char_value_upper_case(){
        let input = 'A';
        let want = 27;
        let got = get_character_value(input);
        assert_eq!(want, got);
    }
}
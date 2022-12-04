use std::fs;
fn main() {
    let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");
    //println!("{}", contents);
    let lines= contents.split("\n");
    let mut total = 0;
    for s in lines {
        let (first, second) = split_string_in_half(s);
        let result = get_same_character(first, second);
        total += get_character_value(result)
        
    }
    println!("Total: {}",total)
}

fn split_string_in_half(to_split: &str) -> (&str, &str) {
    let first = &to_split[..to_split.chars().count()/2];
    let second = &to_split[to_split.chars().count()/2..];
    return (first, second)
}


fn get_same_character(first_slot: &str, second_slot: &str) -> char{
    for first_char in first_slot.chars() {
        for second_char in second_slot.chars() {
            if first_char == second_char{
                return first_char
            }
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

    use crate::{get_same_character, get_character_value, split_string_in_half};
    #[test]
    fn test_split_string_in_half(){
        let input = "ABCD";
        let want1 = "AB";
        let want2= "CD";
        let (got1, got2) = split_string_in_half(input);
        assert_eq!(want1, got1);
        assert_eq!(want2, got2);
    }
    #[test]
    fn test_get_same_character(){
        let first_input = "ABcDE";
        let second_input = "eFGHc";
        
        let want = 'c';
        let got = get_same_character(first_input, second_input);
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
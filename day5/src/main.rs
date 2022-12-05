use std::{collections::HashMap, fs};
fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Unable to open");
    let split: Vec<&str> = contents.split("\n\n").collect();
    let string_layout = split[0];
    let _ = split[1];
    let _ = parse_layout(string_layout);
}

fn parse_layout(layout: &str) -> HashMap<i32, Vec<&str>> {
    let mut output: HashMap<i32, Vec<&str>> = HashMap::new();
    let lines: Vec<&str> = layout.split("\n").collect();
    let last_line = lines[lines.len() - 1];
    let column_nums = last_line.split("   ");

    for column in column_nums {
        let column_number: i32 = column.trim().parse().unwrap();
        output.insert(column_number, Vec::new());
    }
    // Pattern is 3 then space or newline

    for line in lines {
        if line == last_line {
            break;
        }
        let mut i = 0;
        let mut column = 0;
        while i < line.len() {
            i += 4;
            column += 1;
            let line_chars: Vec<char> = line.chars().collect();
            if line_chars[i - 3] != ' ' {
                output.get(column)
                column_vec.push(line_chars[i - 3]);

            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parse_layout;

    #[test]
    fn test_parse_layout() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";
        let mut want: HashMap<i32, Vec<&str>> = HashMap::new();
        want.insert(1, ["N", "Z"].to_vec());
        want.insert(2, ["D", "C", "M"].to_vec());
        want.insert(3, ["P"].to_vec());
        let got = parse_layout(input);
        assert_eq!(want, got);
    }
}

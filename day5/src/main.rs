use std::{collections::HashMap, fs};
fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Unable to open");
    let output: String = run(contents).into_iter().collect();
    println!("{}", output);
}
fn run(content: String) -> Vec<char> {
    let split: Vec<&str> = content.split("\n\r\n").collect();
    let string_layout = split[0];
    let instructions = split[1];
    let mut layout: HashMap<i32, Vec<char>> = HashMap::new();
    parse_layout(string_layout, &mut layout);
    let instruction_list: Vec<&str> = instructions.split("\n").collect();
    for instruction_str in instruction_list {
        let instruction = parse_instruction(instruction_str);
        process_instruction(instruction, &mut layout);
    }
    let mut output: Vec<char> = Vec::new();
    for i in 1..layout.keys().len() + 1 {
        let index = i as i32;
        output.push(layout.get(&index).unwrap()[0])
    }
    output
}
fn process_instruction(
    instrunction: Vec<i32>,
    state: &mut HashMap<i32, Vec<char>>,
) -> &mut HashMap<i32, Vec<char>> {
    let mv = instrunction[0] as usize;
    let from = instrunction[1];
    let to = instrunction[2];
    let from_vec = state.get_mut(&from).unwrap();
    // let to_print: String = from_vec.into_iter().collect();
    // println!("{}", to_print);

    let mut to_move = from_vec[0..mv].to_vec();
    // let to_print: String = to_move.into_iter().collect();
    // let to_move = from_vec[0..mv as usize].to_vec();
    for _ in 0..mv {
        from_vec.remove(0);
    }
    let to_vec = state.get_mut(&to).unwrap();
    to_move.append(to_vec);
    state.insert(to, to_move);
    state
}
fn parse_layout(layout: &str, output: &mut HashMap<i32, Vec<char>>) {
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
                output.get_mut(&column).unwrap().push(line_chars[i - 3]);
            }
        }
    }
}

fn parse_instruction(line: &str) -> Vec<i32> {
    let split_output: Vec<&str> = line.split(" ").collect();
    let mv: i32 = split_output[1].parse().unwrap();
    let from: i32 = split_output[3].parse().unwrap();
    let to: i32 = split_output[5].trim_end().parse().unwrap();
    [mv, from, to].to_vec()
}

#[cfg(test)]
mod tests {
    use crate::{parse_instruction, parse_layout, process_instruction, run};
    use std::collections::HashMap;
    use std::fs;
    #[test]
    fn test_example() {
        let input = fs::read_to_string("./test_input.txt").expect("Unable to open");
        let want: Vec<char> = ['M', 'C', 'D'].to_vec();
        let got = run(input);
        assert_eq!(want, got)
    }
    #[test]
    fn test_process_instruction() {
        {
            let state: &mut HashMap<i32, Vec<char>> = &mut HashMap::new();
            state.insert(1, ['A'].to_vec());
            state.insert(2, ['B'].to_vec());

            let input = [1, 1, 2].to_vec();
            let want: &mut HashMap<i32, Vec<char>> = &mut HashMap::new();
            want.insert(1, [].to_vec());
            want.insert(2, ['A', 'B'].to_vec());
            let got = process_instruction(input, state);
            assert_eq!(want, got)
        }
        {
            let state: &mut HashMap<i32, Vec<char>> = &mut HashMap::new();
            state.insert(1, ['A', 'C'].to_vec());
            state.insert(2, ['B'].to_vec());

            let input = [2, 1, 2].to_vec();
            let want: &mut HashMap<i32, Vec<char>> = &mut HashMap::new();
            want.insert(1, [].to_vec());
            want.insert(2, ['A', 'C', 'B'].to_vec());
            let got = process_instruction(input, state);
            assert_eq!(want, got)
        }
    }
    #[test]
    fn test_parse_layout() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";
        let want: &mut HashMap<i32, Vec<char>> = &mut HashMap::new();
        want.insert(1, ['N', 'Z'].to_vec());
        want.insert(2, ['D', 'C', 'M'].to_vec());
        want.insert(3, ['P'].to_vec());
        let got: &mut HashMap<i32, Vec<char>> = &mut HashMap::new();
        parse_layout(input, got);
        assert_eq!(want, got);
    }
    #[test]
    fn test_parse_instruction() {
        let input = "move 7 from 6 to 8";
        let want = [7, 6, 8].to_vec();
        let got = parse_instruction(input);
        assert_eq!(want, got);
    }
}

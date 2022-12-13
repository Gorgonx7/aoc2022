use std::{fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Unable to open");
    let got = run(contents);
    println!("{}", got)
}

fn run(contents: String) -> i32{
    let split: Vec<&str> = contents.split("\n").collect();
    
    let result = parse(itr);
    let mut total = 0;
    for number in result {
        if number > 100000 {
            total += number;
        }
    }
    total
}

fn parse(split: Vec<&str>) -> Vec<i32> {
    let iterator = split.iter();
    let mut total: Vec<i32> = Vec::new();
    let mut size = 0; 
    for (i, ref_line) in iterator.enumerate() {
        let line = *ref_line;
        match line[0..3].to_string().as_str() {
            "$ c" => {
                let dir: Vec<&str> = line.split(" ").collect();
                if dir[1] == ".." {
                    total.push(size);
                    return total;
                } else {
                    let mut sub_dir_size = parse(split[i..].to_vec());
                    total.append(&mut sub_dir_size);
                }
            },
            _ => println!("{}", line),
        }
    }
    total
}


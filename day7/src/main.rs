use std::{fs, ptr::null_mut};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Unable to open");
    let got = run(contents);
    println!("{}", got)
}
fn run(contents: String) -> i32{
    let split = contents.split("\n");
    let root = &mut DirectoryNode::new("".to_string(),null_mut());
    #[allow(unused_mut)]
    let mut current_location: *mut DirectoryNode =  root;
    for s in split {
        unsafe{
        parse_line(s, current_location);
    }

    }
    0
}
fn walk(node: DirectoryNode) -> i32{
    0
}

unsafe fn parse_line(line: &str, mut current_location: *mut DirectoryNode){
    match line[0..3].to_string().as_str() {
            
        "$ c" => current_location = (*current_location).parent_directory,
       
        "$ l" => return,
        "dir" => {
            let split: Vec<&str> = line.split(" ").collect();
        let new_node = DirectoryNode::new(split[1].to_string(), current_location);
        (*current_location).sub_directories.push(new_node);
        },

        _ => {
            let split: Vec<&str> = line.split(" ").collect();
            let new_size: i32 = split[0].parse().unwrap();
            (*current_location).size += new_size;
        },

    }
}
struct DirectoryNode{
    parent_directory: *mut DirectoryNode,
    name: String,
    sub_directories: Vec<DirectoryNode>,
    size: i32,
}

impl DirectoryNode {
    fn new( name: String, parent: *mut DirectoryNode) -> DirectoryNode{
        DirectoryNode {
            sub_directories: Vec::new(),
            size: 0,
            parent_directory: parent,
            name: name,
        }
    }
    fn get_size(&self) -> i32 {
        let mut total = self.size;
        for dir in &self.sub_directories {
            total += dir.get_size();
        }
        total
    }
}

#[cfg(test)]
mod test {
    use std::{ptr::null_mut};

    use crate::{DirectoryNode, run, parse_line};
    #[test]
    fn test_directory_node_get_size(){
        let mut input = DirectoryNode::new("/".to_string(), null_mut());
        let mut nested = DirectoryNode::new("abcd".to_string(), null_mut());
        nested.size = 32;
        input.sub_directories.push(nested);
        input.size = 32;
        let want = 64;
        let got = input.get_size();
        assert_eq!(want, got);
    }
    #[test]
    fn test_parse_line(){
        unsafe{
        let input = "$ ls";
        let root = &mut DirectoryNode::new("".to_string(),null_mut());
        #[allow(unused_mut)]
        let mut current_location: *mut DirectoryNode =  root;
        parse_line(input, current_location);
        assert_eq!(current_location, root);
        }

    }
    #[test]
    fn test_example(){
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        let want = 95437;
        let got = run(input.to_string());
        assert_eq!(got, want);
    }
  
}
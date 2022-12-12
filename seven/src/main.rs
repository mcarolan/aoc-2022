use std::collections::HashMap;
use std::fmt::Debug;
use std::io::BufRead;
use std::{fs::File, io};

#[derive(Debug, PartialEq)]
enum NodeValue {
    File(String, usize),
    Dir(String),
}

#[derive(Debug, PartialEq)]
enum Instruction {
    ChangeDirectory(String),
    List,
}

#[derive(Debug, PartialEq)]
enum ResolvedInstruction {
    ChangeDirectory(Vec<String>),
    List(Vec<ListOutputItem>),
}


#[derive(Debug, PartialEq, Clone)]
enum ListOutputItem {
    File(String, usize),
    Dir(String),
}

struct Node {
    name: String,
    size: usize,
    children: Vec<Node>
}

fn parse_instruction(line: String) -> Option<Instruction> {
    let parts = Vec::from_iter(line.split(' '));

    if parts.first() == Some(&"$") {
        match parts.get(1) {
            Some(&"cd") => parts
                .get(2)
                .map(|s| Instruction::ChangeDirectory(s.to_string())),
            Some(&"ls") => Some(Instruction::List),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_list_output(line: &String) -> Option<ListOutputItem> {
    let parts = Vec::from_iter(line.split(' '));

    if parts.first() != Some(&"$") {
        match (parts.get(0), parts.get(1)) {
            (Some(&"dir"), Some(name)) => Some(ListOutputItem::Dir(name.to_string())),

            (Some(size), Some(name)) => {
                let size_parsed = usize::from_str_radix(size, 10).ok();
                size_parsed.map(|size| ListOutputItem::File(name.to_string(), size))
            }

            _ => None,
        }
    } else {
        None
    }
}

fn resolve_instructions(lines: &mut dyn Iterator<Item = String>) -> Vec<ResolvedInstruction> {
    let mut current_dir: Vec<String> = Vec::new();

    let mut result: Vec<ResolvedInstruction> = Vec::new();

    let mut i = lines.peekable();

    while let Some(line) = i.next() {
        let instruction = parse_instruction(line).unwrap();

        match instruction {
            Instruction::ChangeDirectory(to) => {
                if to == "..".to_string() {
                    current_dir.pop().unwrap();
                    result.push(ResolvedInstruction::ChangeDirectory(current_dir.clone()));
                }
                else {
                    current_dir.push(to);
                    result.push(ResolvedInstruction::ChangeDirectory(current_dir.clone()));
                }
            },
            Instruction::List => {
                let mut outputs: Vec<ListOutputItem> = Vec::new();
                while let Some(output) = i.peek().and_then(parse_list_output) {
                    i.next();
                    outputs.push(output);
                }
                result.push(ResolvedInstruction::List(outputs));
            }
        }
    }

    result
}

fn main() {
    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let strings = lines.flat_map(|l| l.ok());

    let iterator: &mut dyn Iterator<Item = String> = &mut strings.into_iter();
    let resolved_instructions = resolve_instructions(iterator);

    let mut children: HashMap<Vec<String>, Vec<NodeValue>> = HashMap::new();

    let mut current_directory: Vec<String> = Vec::new();

    for instruction in resolved_instructions {
        match instruction {
            ResolvedInstruction::ChangeDirectory(directory) => current_directory = directory,
            ResolvedInstruction::List(contents) => {
                let node_values = contents.iter().map(|i| {
                    match i {
                        ListOutputItem::Dir(name) => NodeValue::Dir(name.clone()),
                        ListOutputItem::File(name, size) => NodeValue::File(name.clone(), *size)
                    }
                });
                children.insert(current_directory.clone(), Vec::from_iter(node_values));
            }
        }
    }
    
    // children.iter().for_each(|i|println!("{:?}", i));

    let mut directory_sizes: HashMap<Vec<String>, usize> = HashMap::new();
    let mut main: Vec<Vec<String>> = vec![vec!["/".to_string()]];
    let mut traverse: Vec<Vec<String>> = Vec::new();
    
    while let Some(path) = main.pop() {
        traverse.push(path.clone());

        for child_opt in children.get(&path.clone()) {
            for child in child_opt {
                match child {
                    NodeValue::Dir(dir_path) => {
                        let mut full_path = path.clone();
                        full_path.push(dir_path.clone());
                        main.push(full_path);
                    },
                    NodeValue::File(_name, _size) => ()
                }
            }
        }
    }

    while let Some(path) = traverse.pop() {
        let mut total: usize = 0;

        for child_opt in children.get(&path.clone()) {
            for child in child_opt {
                match child {
                    NodeValue::Dir(dir_path) => {
                        let mut sub_dir = path.clone();
                        sub_dir.push(dir_path.clone());

                        let sub_dir_size = directory_sizes.get(&sub_dir).unwrap();
                        total += sub_dir_size;
                    },
                    NodeValue::File(_name, _size) => {
                        total += _size
                    }
                }
            }
        }

        directory_sizes.insert(path.clone(), total);
    }

    directory_sizes.iter().for_each(|kv| println!("{:?}", kv));

    let answer: usize = 
        directory_sizes.iter().map(|(_path, size)| size).filter(|size| **size <= 100000).sum();

    println!("the answer is {}", answer); 

}

#[cfg(test)]
mod tests {
    use crate::{parse_instruction, parse_list_output, Instruction, ListOutputItem, ResolvedInstruction, resolve_instructions};

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            parse_instruction("$ cd /".to_string()),
            Some(Instruction::ChangeDirectory("/".to_string()))
        );
        assert_eq!(
            parse_instruction("$ ls".to_string()),
            Some(Instruction::List)
        );
        assert_eq!(parse_instruction("104564 dnbmm.bgc".to_string()), None);
    }

    #[test]
    fn test_parse_list_output() {
        assert_eq!(parse_list_output(&&"$ ls".to_string()), None);
        assert_eq!(
            parse_list_output(&&"dir gdj".to_string()),
            Some(ListOutputItem::Dir("gdj".to_string()))
        );
        assert_eq!(
            parse_list_output(&&"167697 pcgjgc.wgl".to_string()),
            Some(ListOutputItem::File("pcgjgc.wgl".to_string(), 167697))
        );
    }

    #[test]
    fn test_resolve_instructions() {
        let instructions = vec![
        "$ cd /".to_string(),
        "$ ls".to_string(),
        "dir dcvzbqf".to_string(),
        "23804 gsdpmrq.bsz".to_string(),
        "$ cd dcvzbqf".to_string(),
        "$ cd ..".to_string()];

        let resolved = vec![
            ResolvedInstruction::ChangeDirectory(vec!["/".to_string()]),
            ResolvedInstruction::List(vec![ListOutputItem::Dir("dcvzbqf".to_string()), ListOutputItem::File("gsdpmrq.bsz".to_string(), 23804)]),
            ResolvedInstruction::ChangeDirectory(vec!["/".to_string(), "dcvzbqf".to_string()]),
            ResolvedInstruction::ChangeDirectory(vec!["/".to_string()]),
        ];

        let iterator: &mut dyn Iterator<Item = String> = &mut instructions.into_iter();
        assert_eq!(resolve_instructions(iterator), resolved)
    }
}

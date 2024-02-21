use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments specified");
    } else if args.len() == 2 {
        let path = &args[1];
        let fc = read_file(&path);
        
        let lines = get_lines(&fc);
        let words = get_words(&fc);
        let bytes = get_bytes(&fc);
        
        println!("{}, {}, {}, {}", lines, words, bytes, path);
    } else if args.len() == 3 {
        let flag = &args[1];
        let path = &args[2];
        let fc = read_file(&path);
        
        if flag == "-c" {
            println!("{}, {}", get_bytes(&fc), path);
        } else if flag == "-w" {
            println!("{}, {}", get_words(&fc), path);
        } else {
            println!("{}, {}", get_lines(&fc), path);
        }
    }
    
}

fn read_file(path: &String) -> String {
    let file_contents = fs::read_to_string(path)
    .expect("Failed to read the file");

    file_contents
}

fn get_bytes(fc: &String) -> usize {
    fc.len()
}

fn get_words(fc: &String) -> usize {
    let words: Vec<&str> = fc.split_whitespace().collect();
    words.len()
}

fn get_lines(fc: &String) -> usize {
    let lines: Vec<&str> = fc.split("\n").collect();
    lines.len() - 1
}
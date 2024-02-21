use std::env;
use std::fs;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let mut input = String::new();

        let res = io::stdin().read_line(&mut input);
        if res.is_ok() {
            let res = get_all_data(input);
            println!("{}, {}, {}", res[0], res[1], res[2]);
        } else {
            println!("No argument provided");
        }

    } else if args.len() == 2 {
        let path = &args[1];
        let fc = read_file(&path);
        
        let res = get_all_data(fc);
        println!("{}, {}, {}, {}", res[0], res[1], res[2], path);
        
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

fn get_all_data(fc: String) -> Vec<usize> {
    let lines = get_lines(&fc);
    let words = get_words(&fc);
    let bytes = get_bytes(&fc);
    
    let res = vec![lines, words, bytes];
    res
}
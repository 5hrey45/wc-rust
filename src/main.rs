use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    
    let fc = read_file(String::from("../text.txt"));
    
    println!("length of the file is {}", get_bytes(&fc));
    println!("words in the file is {}", get_words(&fc));
}

fn read_file(path: String) -> String {
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
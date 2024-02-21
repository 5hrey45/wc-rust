use std::fs;

fn main() {
    let fc = read_file(String::from("../text.txt"));
    
    println!("length of the file is {}", get_bytes(fc));
}

fn read_file(path: String) -> String {
    let file_contents = fs::read_to_string(path)
    .expect("Failed to read the file");

    file_contents
}

fn get_bytes(fc: String) -> usize {
    fc.len()
}
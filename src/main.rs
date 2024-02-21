use std::fs;

fn main() {
    let file_contents = read_file(String::from("../text.txt"));
    
    println!("{}", file_contents);
}

fn read_file(path: String) -> String {
    let file_contents = fs::read_to_string(path)
    .expect("Failed to read the file");

    file_contents
}
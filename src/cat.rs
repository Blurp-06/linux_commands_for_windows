use std::env;
use std::fs;

fn main(){
    for i in 1..env::args().count(){
        let file_name = env::args().nth(i).expect("File not found!");
        println!("----\n{}\n----", file_name);
        let file_content = fs::read_to_string(file_name).expect("Couldn't read the file!");
        println!("{}", file_content);
    }
}
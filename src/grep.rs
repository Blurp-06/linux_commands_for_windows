use std::env;
use std::fs;

fn main(){
    let word: String = env::args().nth(1).expect("Didn't get a word to search!");
    let file_name: String = env::args().nth(2).expect("Didn't get a file_name!");

    let content = fs::read_to_string(file_name).expect("Couldn't read the file.");
    let sentences = content.split("\r\n");
    for sentence in sentences{
        if sentence.contains(&word[..]){
            println!("{}", sentence);
        }
    }
}
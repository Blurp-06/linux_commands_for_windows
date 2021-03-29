use std::fs;
use std::env;

fn main(){
    for i in 1..env::args().count(){
        let file_name = env::args().nth(i).unwrap();
        fs::File::create(file_name);
    }
}
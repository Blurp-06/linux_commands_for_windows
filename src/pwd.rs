use std::env;

fn main(){
    let pwd = env::current_dir().expect("Something went wrong somehow...");
    println!("{}", pwd.display());
}
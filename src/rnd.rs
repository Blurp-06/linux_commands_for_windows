use std::env;

fn main(){
    let min_string: String = env::args().nth(1).expect("Didn't get a minimum value!");
    let max_string: String = env::args().nth(2).expect("Didn't get a maximum value!");

    let min: i32 = min_string.parse().expect("Minimum value isn't a whole number!");
    let max: i32 = max_string.parse().expect("Maximum value isn't a whole number!");

    let temp = vec![2, 3];
    let address = &temp as *const Vec<i32>;
    let mut rnd_number = address as i32;
    
    rnd_number = rnd_number.abs();
    rnd_number = (rnd_number % max) + min;

    println!("{}", rnd_number);
}

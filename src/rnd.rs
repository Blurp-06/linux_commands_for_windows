use std::env;

fn main(){
    let min: String = env::args().nth(1).expect("Didn't get a minimum value!");
    let max: String = env::args().nth(2).expect("Didn't get a maximum value!");

    let min: i32 = min.parse().expect("Minimum value isn't a whole number!");
    let max: i32 = max.parse().expect("Maximum value isn't a whole number!");

    let temp = vec![2, 3];
    let address = &temp as *const Vec<i32>;
    let mut rnd_number = address as i32;
    
    rnd_number = rnd_number.abs();
    rnd_number = (rnd_number % max) + min;

    println!("{}", rnd_number);
}

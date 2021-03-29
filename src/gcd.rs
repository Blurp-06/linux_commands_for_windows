use std::env;

fn main(){
    let a_string: String = env::args().nth(1).expect("Didn't receive first argument!");
    let b_string: String = env::args().nth(2).expect("Didn't receive second argument!");
    let a = a_string.parse().expect("First argument isn't a number!");
    let b = b_string.parse().expect("Second argument isn't a number!");
    println!("The greatest common dividor of {} and {} is {}", a, b, gcd(a, b));
}

fn gcd(a: i128, b: i128) -> i128{
    if a == 0{
        return b;
    }
    if b == 0{
        return a;
    }
    if a == b{
        return a;
    }
    if a > b{
        return gcd(a-b, b);
    }
    return gcd(a, b-a);
}
fn main(){
    let number: String = std::env::args().nth(1).expect("Didn't receive a number!");
    let number: i128 = number.parse().expect("Value isn't a number!");

    let mut is_prime: bool = true;

    if number == 1{
        println!("1 isn't a prime.");
    } else if number < 1{
        println!("Please enter a number equal or higher than 1.");
    } else{
        for n in 2..number{
            if number % n == 0 {
                println!("{} is dividable by {}.", number, n);
                is_prime = false;
            }
        }

        if is_prime{
            println!("{} is a prime.", number);
        } else{
            println!("{} isn't a prime.", number);
        }
    }
}
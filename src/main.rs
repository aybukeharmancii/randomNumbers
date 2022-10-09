extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..11); //1-10
    println!("Random number is {}", random_number);

    //Flip a coin
    let random_bool = rand::thread_rng().gen_bool(1.0/2.0);
    println!("Random Boolean: {}", random_bool);
}

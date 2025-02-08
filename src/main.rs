mod lib;
use lib::Contract;
use std::io;

fn main() {
    let mut input_value = String::new();
    println!("Welcome to the Greeting Contract!");

    io::stdin().read_line(&mut input_value).unwrap();

    let mut contract = Contract::new(input_value.clone());
    // println!("{}", contract.greet());
    println!("{:?}", contract.change_greeting(input_value));


}

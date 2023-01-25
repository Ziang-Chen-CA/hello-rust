use std::io;//prelude
use rand::Rng;//trait
use std::cmp::Ordering;
fn main() {
    println!("Let's guess the number! ");
    let secret_number = rand::thread_rng().gen_range(1,101);
    // println!("The number generated is: {}.",secret_number);
    loop{
        println!("Guess a number: "); 
        let mut guess= String::new();
        io::stdin().read_line(&mut guess).expect("Oops,cannot read the line.");
        //shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("The number you guess is {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    };        
}
   
 
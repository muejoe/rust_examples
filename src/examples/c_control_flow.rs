
pub fn main() {
    universe_number();
    guessing_game();
}

pub fn universe_number() {
    println!("Please enter a number:"); 

    // here I get a result and not a number because it may fail
    let result = read_number();

    // If you like to return in case it did not succeed
    if result.is_err() {
        return;
    }
    // request the number if you really are sure it had worked
    // it is okay in this case, because I tested "is_err" before
    let number = result.unwrap(); 
    
    // work with the number
    if number > 42 {
        println!("Your number is bigger than 42!");
    } else if number  == 42 {
        println!("The answer to the universe, the life and all!"); 
    } else {
        println!("Your number is smaller than 42...");
    }
}

pub fn guessing_game() {

    use std::cmp::Ordering;

    println!("Guess my number between 1 and 20:"); 
    let secret_number = get_pseudo_random_1_to_(20);

    loop {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Cannot read input.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }

}


pub fn read_number() -> Result<i32, Box<dyn std::error::Error>> {
    let mut text = String::new();
    std::io::stdin().read_line(&mut text)?;
    let number: i32 = text.trim().parse()?;
    Ok(number)
}

pub fn get_pseudo_random_1_to_(max: u32) -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
        return (duration.subsec_millis() % max) + 1
    }
    42
}
pub fn main() {
    simple();
    with_error_handling();
}

pub fn simple() {
    println!("Please enter your name: ");

    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Cannot read name.");

    println!("And now enter a number:");

    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Cannot read name.");
   
    let number: i32 = number.trim().parse().expect("Is not a number");

    println!("{} chose the number {}.", name.trim(), number);
}

pub fn with_error_handling() {
    println!("Please enter a number:"); 

    // here I get a result and not a number because it may fail
    let result = read_number();

    // check option and get number if existing
    match result {
        Ok(number) => println!("Yes, {number} is a number."),
        Err(ref e) => println!("Error: {}", e.to_string()),
    }
}

// A nice way to handle Results. Each line which ends with a question mark would return a Result.
// The question mark would lead to an early exit if the result is an error, 
// else it unpacks the result and continues. 
// Result is an enum with an Ok value with an embedded i32 and 
// an Err value with an embedded pointer to something which derived Error
pub fn read_number() -> Result<i32, Box<dyn std::error::Error>> {
    let mut text = String::new();
    std::io::stdin().read_line(&mut text)?;
    let number: i32 = text.trim().parse()?;
    Ok(number)
}
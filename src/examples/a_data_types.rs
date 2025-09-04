
pub fn main() {
    integer_variables();
    floats_and_other_simple_variables();
    tuples();
    arrays();
    vectors();
}

fn integer_variables() {
    // Some integer values
    let number = 2 * 21;
    let unsigned_16bit_int : u16 = 82 - 40;
    let signed_32bit_int : i32 = 42;
    
    // If no concrete type is given, rust selects a fitting type, 
    // so number will also be u16
    if unsigned_16bit_int == number { 
        println!("They are the same!");
    }
    
    // Two variables with fixed type are compared, 
    // so you need to cast to avoid seldom errors
    if signed_32bit_int == unsigned_16bit_int as i32 { 
        println!("Unsigned 32 and signed 16 also are the same!");
    }
}

fn floats_and_other_simple_variables() {
    // easy assignment with different formats, also underscores are allowed
    let hex_number = 0x55;
    let octal_number = 0o72;
    let binary_number = (0b0101_0010 | 0b1010_0001) & 0b0011_1111;
    let formatted_number = 1_234_567_890;
    let character = b'A';
    let is_boolean = true;
    let heart_eyed_cat = '😻';
    // It's a char

    println!("Hex: {:X}, Oct: {:o}, Bin: {:b}", hex_number, octal_number, binary_number);
    println!("Formatted No: {}, Character: {}", formatted_number, character);
    println!("Boolean: {}, Unicode Char: {}", is_boolean, heart_eyed_cat);

    // some floating numbers
    let float : f32 = 42.0;
    let double : f64 = 1234.567890123456789;

    println!("Float: {}, Double: {}", float, double);
}

fn tuples() {
    // then there are tuples with large flexibility:
    let treasure_location = 
        (12345, 67890, "near the tree");
    println!("Treasure hint: {}", treasure_location.2);
    let (x, y, hint) = treasure_location;
    println!("Treasure location: x {x}, y {y}, hint: {hint}");
}

fn arrays() {
    // Content of arrays can change but not its length, arrays are stored on the stack

    let mut numbers = [0; 3];

    numbers[0] = 1;
    numbers[1] = 2;
    numbers[2] = 3;
    //numbers[3] = 4; // Wont compile: "index out of bounds: the length is 3 but the index is 3"


    print!("Numbers: ");
    for number in numbers {
        print!("{number} ");
    }

    let other_numbers = [1,2,3,4,5,6,7,8,9,10];

    print!("Other numbers: ");
    for number in other_numbers {
        print!("{number} ");
    }

    // let index = 11;
    // println!("Index 11 of other_numbers is {}", other_numbers[index]); // Wont compile either

    // But it would compile, when the number is not known at compile time. 
    // Then index out of bounds would result in a panick = quitting app to prevent undefined behaviour.
    // so use .get() in this case:
    println!("\nWhich index of other_numbers shall be printed?");
    if let Ok(index) = read_number() {
        //println!("Index {} of other_numbers is {}", index, other_numbers[index]); // Could panick
        println!("Index {} of other_numbers is {:?}", index, other_numbers.get(index)); // is ok, returns Option
    }


 
}

fn vectors() {
    // Vectors have dynamic length but it stores the content on the heap
    let mut vector1 = Vec::new();
    vector1.push(1);
    vector1.push(2);
    vector1.push(3);

    print!("\nVector 1: ");
    for number in &vector1 {
        print!("{number} ");
    }

    print!("\nVector 2: ");
    let vector2 = vec![1,2,3,4];

    for number in vector2 {
        print!("{number} ");
    }

    let second_value = &vector1[1];
    println!("\nSecond value = {}", *second_value);

    vector1.push(4);

    // not allowed because it is modified in between, vector may move so that reference get invalid
    // println!("Second value = {}", *second_value); 

}

// will be explained in next example
pub fn read_number() -> Result<usize, Box<dyn std::error::Error>> {
    let mut text = String::new();
    std::io::stdin().read_line(&mut text)?;
    let number = text.trim().parse()?;
    Ok(number)
}
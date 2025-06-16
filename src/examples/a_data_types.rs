
pub fn example() {
    integer_variables();
    floats_and_other_simple_variables();
    tuples();
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
    let binary_number = 0b1010_1111;
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

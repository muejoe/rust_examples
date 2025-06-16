use examples::{a_data_types, b_parsing_inputs, c_control_flow};

mod examples;

struct Example {
    pub name: String,
    pub function: fn()
}

impl Example {
    fn new(name: &str, function: fn()) -> Self {
        Example { name: String::from(name), function }
    }
}

fn main() {
    println!("Hello, world!");

    let examples: Vec<Example> = vec![
        Example::new("Data Types", a_data_types::example),
        Example::new("Parsing Inputs - simple", b_parsing_inputs::simple),
        Example::new("Parsing Inputs - with error handling", b_parsing_inputs::with_error_handling),
        Example::new("Control Flow - universe number", c_control_flow::universe_number),
        Example::new("Control Flow - guessing game", c_control_flow::guessing_game),
    ];

    loop {

        println!("\nList of topics:");
        println!("┌──────────────────────────────────────────┐");
        
        examples.iter().enumerate()
            .for_each(|(index, e)|{
                let number = format!("{:>width$}", index+1, width = 2);
                let name = format!("{:width$}", e.name, width = 36);
                println!("│ {}: {} │", number, name);    
            });
            
        println!("├──────────────────────────────────────────┤");
        println!("│  0: Exit Application                     │");
        println!("└──────────────────────────────────────────┘");
        println!("Choose one:");

        let mut topic = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut topic) {
            match topic.trim().parse::<u16>() {
                Ok(number) => {

                    if number < 1 {break};

                    let index: usize = (number-1) as usize;
                    if let Some(example) = examples.get(index) {
                        (example.function)();
                        println!("\n< Press enter to continue >");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).expect("-");
                    } 
                },
                Err(_) => continue
            };
        }
    }

}

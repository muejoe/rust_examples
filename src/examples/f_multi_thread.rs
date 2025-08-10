use std::sync::Arc;
//use std::sync::Mutex; // universal alternative to Atomic
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;
use std::io;

pub fn main() {

    let current_number = Arc::new(AtomicI32::new(1));
    //let current_number = Arc::new(Mutex::new(1)); //for complex types, also blocks
    let current_number_clone = Arc::clone(&current_number);

    let input_thread = thread::spawn(move || { input_thread(current_number_clone); });
    let sum_thread = thread::spawn(move || { sum_thread(current_number); });

    input_thread.join().unwrap();
    sum_thread.join().unwrap();
}


fn input_thread(current_number: Arc<AtomicI32>) {
    loop {
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            if let Ok(number) = input.trim().parse() {

                //let mut current_number = current_number.lock().unwrap(); //in case of mutex
                //*current_number = number;
                 current_number.store(number, Ordering::SeqCst);

                if number == 0 {
                    break;
                }
            }
        }
    }
}

fn sum_thread(current_number: Arc<AtomicI32>) {
    let mut sum: i64 = 0;
    loop {
        thread::sleep(Duration::from_secs(3));

        //let number = current_number.lock().unwrap(); // in case of mutex
        let number = current_number.load(Ordering::SeqCst);
        if number == 0 {
            break;
        }
        sum += number as i64;

        println!("\nCurrent sum: {}", sum);
    }
}



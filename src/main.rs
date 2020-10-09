fn main() {
    use std::io::{self,Write};
    println!("Fibonacci generator");
    println!("Type \"q\" to end the program");
    println!();
    loop {
        let mut value = String::new();

        print!("Enter a positive integer: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut value).expect("Failed to read line");

        if value.trim() == "q" {
            break;
        }

        let value: u64 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("{}", fibonacci(&value));
        // println!("{}", fibonacci(value as u64));
    }
}

fn fibonacci(num: &u64) -> u64 {
    match num {
        0 => 0,
        1 => 1,
        _ => fibonacci(&(*num - 1)) + fibonacci(&(*num - 2))
    }
}

// fn fibonacci(num: u64) -> u64 {
//     match num {
//         0 => 0,
//         1 => 1,
//         _ => fibonacci(num - 1) + fibonacci(num - 2)
//     }
// }

// fn fibonacci(num: u32) -> u32 {
//     if num < 2 { return num };
//     return fibonacci(num - 1) + fibonacci(num - 2);
// }
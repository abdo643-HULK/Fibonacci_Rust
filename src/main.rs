fn main() {
    use std::io::{self,Write};
    println!("Fibonacci generator");
    println!("Type \"q\" to end the program");
    println!();
    
    loop {
        let mut value = String::new();

        print!("Enter a positive integer (max 93): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut value).expect("Failed to read line");

        if value.trim() == "q" {
            break;
        }

        let value: u32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fibonacci(&value));
    }
}

fn fibonacci(num: &u32) -> u64 {
    let size: usize = *num as usize + 1;
    let mut f = vec![0; size];
    f[1] = 1;
    // let mut i: usize = 2;
    for i in 2..size {
        f[i] = f[i-1] + f[i-2];
    }
    // while i <= size { 
    //     i+=1;
    //     f[i] = f[i-1] + f[i-2]; 
    // };
    return f[size-1];
}
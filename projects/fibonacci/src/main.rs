use std::io;

fn fibonacci(n: u32) -> u64 {
    if n == 1 || n == 0 {
        return 1;
    } else {
        let mut prev = 1;
        let mut curr = 1;
        for _ in 2..=n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        return curr;
    }
}

fn main() {
    println!("Enter a number: ");
    let num = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<u32>() {
            Ok(val) => {
                break val;
            }
            Err(_) => continue,
        }
    };
    let fib = fibonacci(num - 1);
    println!("{fib}");
}

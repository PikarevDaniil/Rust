use std::io;

fn main() {
    loop {
        println!("Enter a number or 0 to exit...");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong number!");
                continue;
            }
        };

        if input == 0 {
            println!("Bye!");
            break;
        }

        match fibonacci(input) {
            Ok(num) => println!("{}", num),
            Err(last) => println!("Too big number. Last computed: {}", last)
        }
    }
}

fn fibonacci(ind: usize) -> Result<usize, usize> {
    let (mut a, mut b): (usize, usize) = (1, 1);
    for _ in 2..ind {
        match a.checked_add(b) {
            Some(sum) => {
                a = b;
                b = sum
            }
            None => return Err(b),
        }
    }

    Ok(b)
}

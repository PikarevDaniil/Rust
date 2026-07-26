use std::io;

fn main() {
	'root: loop {
        println!("Enter a number or 0 to exit...");

        let mut input = String::new();

        io::stdin()
	        .read_line(&mut input)
	        .expect("Failed to read input");

        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong number!");
                continue
            }
        };

        if input == 0 {
            println!("Bye!"); 
            break 
        }

        let mut nums: Vec<usize> = vec![1, 1];
        for i in 2..input {
            match nums[i-2].checked_add(nums[i-1]) {
                Some(num) => nums.push(num),
                None => {
                    println!("{}+", nums[i-1]);
                    continue 'root
                }
            }
        }
        println!("{}", nums[input-1])
    }
}
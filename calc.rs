use std::io::stdin;

fn main() {
    let mut flag: bool = true;
    println!("Welcome to the calculator app!");
    while flag {
        start();
        println!("Continue?");
        println!("Enter [enter] to exit or any key to continue...");
        let i = get_in();
        if i == "\r\n" {flag = false}
    }
}

fn get_in() -> String {
	let mut res: String = String::new();
    let _ = stdin().read_line(&mut res);
    return res
}

fn calc(a: i32, b: i32, op: String) -> i32{
    let _plus: String = "+\r\n".to_string();
    let _minus: String = "-\r\n".to_string();
    let _mult: String = "*\r\n".to_string();
    let _div: String = "/\r\n".to_string();

    if op == _plus {return a + b}
    else if op == _minus {return a - b}
    else if op == _mult {return a * b}
    else if op == _div {return a / b}
    else {return 0} 
}

fn start() {
    println!("Enter the first number...");
    let a: i32 = get_in().trim().parse().expect("Error in a");
    println!("Enter the second number...");
    let b: i32 = get_in().trim().parse().expect("Error in b");
    println!("Enter the operator...\n[+]\n[-]\n[*]\n[/]");
    let o = get_in();
    println!("Result of the oparetion: {}", calc(a, b, o));
}

use std::io;

fn main() {
    loop {
        println!("Which number of Fibonacci do you want to know? [ EXIT = q ]");

        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("Please type a number!");

        if number.trim() == "q" { break; }

        let number: u8 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue, 
        };

        let result = fibonacci( number );

        println!("The {number}nth number of Fibonacci is: {result}");
    }

}

fn fibonacci( number : u8 ) -> u64 {
    if number == 1 { 0 }
    else if number == 2 { 1 }
    else if number < 1 { println!("Doesn't exist."); 0 }
    else { fibonacci( number - 2 ) + fibonacci( number - 1 ) }
}

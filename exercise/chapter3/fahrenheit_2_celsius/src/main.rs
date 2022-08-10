use std::io;

fn main() {
    loop {
        println!("Type the temperature in Fahrenheit scale. [EXIT = q ]");
    
        let mut temperature = String::new();
        
        read_user_input(&mut temperature);
    
        if temperature.trim() == "q" {
            break; 
        }

        let temperature: f64 = match temperature.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let temperature = fahrenheit_2_celsius(temperature);

        println!("The temperature in Celsius scale is: {temperature}");
    
    }
}

fn fahrenheit_2_celsius( t : f64 ) -> f64 {
    ( t - 32.0 ) * 5.0 / 9.0
}

fn read_user_input( input : &mut String) {
    io::stdin().read_line( input ).expect("Please type a number!");
}


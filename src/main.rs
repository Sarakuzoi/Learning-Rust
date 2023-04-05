use std::{io, process::exit};

fn main() {
    println!("\nHello! Will you be inputting Fahrenheit or Celsius?\nEnter: \n-'F' to convert from Fahrenheit to Celsius \n-'C' to convert from Celsius to Fahrenheit \n-'q' to quit the program");
    let mut f_or_c: String = String::new();
    io::stdin()
        .read_line(&mut f_or_c)
        .expect("Failed to read line");
    let f_or_c = f_or_c.trim();
    match f_or_c {
        "F" => fah_to_cel(),
        "C" => cel_to_fah(),
        "q" => return,
        _ => println!("Please input correct format"),
    }
}

//Converts Fahrenheit to Celsius
fn fah_to_cel() {
    loop {
        println!("\nEnter your Fahrenheit degrees, 'C' to switch to Celsius, or 'q' to quit:");

        let mut fah: String = String::new();
        io::stdin()
            .read_line(&mut fah)
            .expect("Failed to read line");
        if &fah == "q\n" {
            println!("\nHave a nice day!\n");
            exit(1);
        };
        if &fah == "C\n" {
            cel_to_fah();
        };
        let fah: f32 = match fah.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue;
            }
        };
        println!("{fah}°F is equivalent to {:.2}°C!", {
            (fah - 32.0) * 5.0 / 9.0
        });
    }
}

//Converts Celsius to Fahrenehit
fn cel_to_fah() {
    loop {
        println!("\nEnter your Celsius degrees, 'F' to switch to Fahrenheit, or 'q' to quit:");

        let mut cel: String = String::new();
        io::stdin()
            .read_line(&mut cel)
            .expect("Failed to read line");
        if &cel == "q\n" {
            println!("\nHave a nice day!\n");
            exit(1);
        };
        if &cel == "F\n" {
            fah_to_cel();
        };
        let cel: f32 = match cel.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number");
                continue;
            }
        };
        println!("{cel}°C is equivalent to {:.2}°F!", {
            (cel * 9.0 / 5.0) + 32.0
        });
    }
}

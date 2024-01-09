use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn get_user_input(message: String) -> String {
    println!("{message}");
    let mut input_password = String::new();
    io::stdin().read_line(&mut input_password).unwrap();
    String::from(input_password.trim())
}


fn rock(){
    let input_password = get_user_input(String::from("Enter your alphanumeric password"));

    let file = File::open("rockyou.txt").expect("You rock! But the file was not found");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let password = match line {
            Ok(line) => line,
            Err(..) => String::from("")
        };

        if password == input_password {
            println!("HACKED: {}, attempt: {}", password, index);
            return;
        }
    }

    println!("You're password is secure");
}

fn main() {
    number_bruteforce();
    rock();
}


fn number_bruteforce(){
    let input_password = get_user_input(String::from("Enter your numerical password"));
    match input_password.parse::<i32>(){
        Ok(..) => println!("Password is valid"),
        Err(..) => {
            println!("Password is invalid, returning");
            return;
        }
    }
    let length = input_password.len();
    let base: i32 = 10;
    let max = base.pow(length as u32);

    println!("This {}", max);
    for i in 0..max {
        // No we'll fill the gap with zeros
        let current = i.to_string();
        let current_length = current.len();
        let delta = length - current_length;
        let mut guess = "".to_string();
        for d in 0..delta {
            guess = format!("{guess}0")
        }
        guess = format!("{guess}{current}");
        if guess == input_password {
            println!("Hacked: {guess}");
            return
        }
    }
}

use std::io;

use character::new::Class;

mod character;

fn main() {

    println!("Welcome in my game!!!");
    loop {
        println!("Let's dive into fantasy game, do you want to join me?");
        println!("If yes press '1', or '0' if you want to leave the game");

        let mut choice= String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Filed to read line");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
         match choice {
            1 => { 
                println!("let's start new game!!");
                let char_1 = character::new::Character::create_new_character(String::from("John"), Class::Mage);
                char_1.describe();
                break
            },
            0 => {
                println!("Bye, bye.");
                break 
            },
            _ => println!("chose 1 or 0, to play or leave."),
        }


    }
}
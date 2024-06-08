use std::io;

pub fn get_user_input() -> String {
    println!("Enter a string:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    user_input.trim().to_lowercase()
}

pub fn is_palindrome(user_input: String) -> bool {
    let reverse: String = user_input.chars().rev().collect();
    user_input == reverse
}

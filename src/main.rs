use is_palindrome::{get_user_input, is_palindrome};

fn main() {
    let user_input = get_user_input();

    if is_palindrome(user_input) {
        println!("This string is a palindrome.")
    } else {
        println!("This string is not a palindrome")
    }

}

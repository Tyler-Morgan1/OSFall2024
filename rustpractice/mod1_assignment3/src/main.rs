
fn check_guess(guess: i32, secret:i32) -> i32 {
    if guess == secret{
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let mut secret_num = 53;

    let mut my_guess = 1;
    let mut num_guesses = 0;
    let mut result = 0;
    
    loop {
        result = check_guess(my_guess, secret_num);
        num_guesses += 1;
        if result == 0 {
            println!("Correct!");
            break;
        } else if result == 1 {
            println!("Too high. Try again.");
        } else if result == -1 {
            println!("Too low. Try again.");
        } else {
            println!("Error! Error! ERROR ERROR ERROR ERROR ERRORRRRR!!!!!!!!!!!");
        }
        my_guess += 1;
    }

    println!("{}", num_guesses);
}

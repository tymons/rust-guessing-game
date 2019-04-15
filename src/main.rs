use std::io;

fn main() {
	println!("Guess the number!");
	println!("Please input your guess.");
	
	// UTF-8 encoded string
	let mut guess = String::new();
	
	io::stdin().read_line(&mut guess)
		.expect("Failed to read line");
	
	println!(" You've guessed: {}", guess);
	
}
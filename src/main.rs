use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

	let secret_number = rand::thread_rng().gen_range(1, 101);		
	
	println!("Please input your guess.");
	// UTF-8 encoded string	
	loop {
		let mut guess = String::new();
	
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");
			
		let guess: u32 = match guess.trim().parse() {
			Ok(bytes) => bytes,
			Err(_)	=> { 
				println!("Cannot parse. Please enter correct numbers.");
				continue;
			}
		};
		
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("Yes, you win!");
				break;
			}
		}
	}	
}
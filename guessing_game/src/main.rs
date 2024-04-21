use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	println!("Guesse the number!");

	let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {

		println!("Please input your guess.");
	
		let mut guess = String::new(); // mutavel

		io::stdin()
			.read_line(&mut guess)
			.expect("Falha para ler a linha");
		println!("You guessed: {guess}");

		let guess: u32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}

fn main() {
	let condition = true;
	// o bloco if precisa ter o mesmo tipo em cada um dos bracos
	let number = if condition { 5 } else { 6 };
	println!("The value of number is: {number}");
}

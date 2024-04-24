fn main() {
	println!("Hello, world");

	another_function();
	another_function_print(5);
	print_labeled_mesurement(5, 'h');
}

fn another_function() {
	println!("Another function");
}

fn another_function_print(x: i32) {
	println!("The value of x is: {x}");
}

fn print_labeled_mesurement (value: i32, unit_label: char) {
	println!("The mesurement is: {value} {unit_label}");
}

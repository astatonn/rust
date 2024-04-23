fn main() {
// diferentes tipos de valores mas imutaveis
	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let (x, y, z) = tup;

	println!("The value of y is: {y}");
// usa-se o nome da variavel seguido de ponto e o indice para exibir
	let five_hundred = tup.0;

	let one = tup.2;
}

use std::io;

fn main() {
// o tamanho dos arrays eh fixo, nao eh possivel mudar o tipo
// eh diferente do vetor
	let a = [1,2,3,4,5];
	let b: [i32;5] = [1,2,3,4,5];
// inicializar um array com o mesmo valor
	let fixed = [3;5]; // [3,3,3,3,3]
// acessar elementos do array
	let first = a[0];
	let second = a[1];

	println!("Digite um indice: ");
	let mut index = String::new();

	io::stdin()
		.read_line(&mut index)
		.expect("Falha para ler a linha");

	let index: usize = index
		.trim()
		.parse()
		.expect("Indice enviado nao eh um numero");

	let element = a[index];

	println!("O numero do indice {index} eh: {element}");
}

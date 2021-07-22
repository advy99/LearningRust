use std::io;
use std::io::Write;

fn main() {
	let mut numero = String::new();

	print!("Introduce un número: ");
	io::stdout().flush().unwrap();
	io::stdin()
			.read_line(&mut numero)
			.expect("Error al leer la linea.");

	let numero: u32 = numero.trim().parse().expect("Se esperaba un número");

	println!("Comprobando si el numero introducido es menor que cinco");
	if numero < 5 {
		println!("La condición es cierta");
	} else {
		println!("La condicion es falsa");
	}

	if numero % 4 == 0 {
		println!("{} es divisible por 4", numero);
	} else if numero % 3 == 0 {
		println!("{} es divisible por 3", numero);
	} else if numero % 2 == 0 {
		println!("{} es divisible por 2", numero);
	} else {
		println!("{} no es divisible por 4, 3 o 2", numero);
	}

	let mut numero = if true { 5 } else { 6 };

	println!("Ahora el numero vale: {}", numero);

	while numero != 0 {
		println!("{}!", numero);
		numero -= 1;
	}

	println!("\n Ahora recorremos un array");
	let a = [10, 20, 30, 40, 50];

	for elemento in a.iter() {
		println!("{} ", elemento);
	}

}

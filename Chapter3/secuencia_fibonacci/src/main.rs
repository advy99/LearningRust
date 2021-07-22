use std::io;
use std::io::Write;

fn main() {
	let mut entrada = String::new();

	print!("Introduce un numero: ");
	io::stdout().flush().unwrap();
	io::stdin()
			.read_line(&mut entrada)
			.expect("No se ha podido leer la linea");

	let longitud: u32 = entrada.trim().parse().expect("Se esperaba un numero");

	let valor_fib_longitud = obtener_valor_fibonnaci_pos(longitud);

	println!("En la posicion {} de la sucesión de Fibonacci se encuentra el numero {}", longitud, valor_fib_longitud);
}

fn obtener_valor_fibonnaci_pos(longitud: u32) -> u32 {
	let mut valor: u32 = 1;
	let mut anterior: u32 = 1;

	for _ in 1..longitud {
		let valor_actual = valor;
		valor = valor + anterior;
		anterior = valor_actual;
	}

	valor
}

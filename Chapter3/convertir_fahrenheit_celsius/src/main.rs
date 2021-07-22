use std::io;
use std::io::Write;

fn main() {

	let unidad = leer_unidad();
	let temperatura = leer_temperatura();

	
	if unidad == 'F' {
		println!("{} F = {} C", temperatura, convertir_fahrenheit_a_celsius(temperatura));
	} else if unidad == 'C' {
		println!("{} C = {} F", temperatura, convertir_celsius_a_fahrenheit(temperatura));
	}


}

fn leer_unidad() -> char {
	let mut entrada = String::new();

	print!("Selecciona una unidad (F o C): ");
	io::stdout().flush().unwrap();
	io::stdin()
			.read_line(&mut entrada)
			.expect("No se ha podido leer la linea");

	let mut unidad: char = match entrada.trim().parse() {
		Ok(caracter) => caracter,
		Err(_) => ' ',
	};

	while unidad != 'C' && unidad != 'F' {
		entrada.clear();
		print!("Selecciona una unidad (F o C): ");
		io::stdout().flush().unwrap();
		io::stdin()
				.read_line(&mut entrada)
				.expect("No se ha podido leer la linea");

		unidad = match entrada.trim().parse() {
			Ok(caracter) => caracter,
			Err(_) => ' ',
		};
	}

	unidad
}

fn leer_temperatura() -> f64 {
	let mut entrada = String::new();

	print!("Introduce una temperatura: ");
	io::stdout().flush().unwrap();
	io::stdin()
			.read_line(&mut entrada)
			.expect("No se ha podido leer la linea");

	let temperatura: f64 = entrada.trim().parse().expect("Se esperaba un número real");

	temperatura
}

fn convertir_celsius_a_fahrenheit(temperatura: f64) -> f64 {
	(temperatura * 1.8) + 32.0
}

fn convertir_fahrenheit_a_celsius(temperatura: f64) -> f64 {
	(temperatura - 32.0) / 1.8
}

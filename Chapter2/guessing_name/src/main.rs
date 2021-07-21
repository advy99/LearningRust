use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Averigua el número");

	let secret_number = rand::thread_rng().gen_range(1..101);

	println!("El número secreto es: {}", secret_number);

	loop {
		let mut guess = String::new();

		print!("Introduce un número: ");
		io::stdout().flush().unwrap();
		io::stdin()
				.read_line(&mut guess)
				.expect("Error al leer la linea.");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("Has introducido: {}", guess);


		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Demasiado pequeño"),
			Ordering::Greater => println!("Demasiado grande"),
			Ordering::Equal => {
				println!("Has acertado!");
				break;
			}
		}
	}


}

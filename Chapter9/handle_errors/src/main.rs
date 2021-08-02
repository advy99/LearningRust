use std::fs::File;
use std::io::ErrorKind;

fn main() {
	
	let f = File::open("hola.txt");

	let f = match f {
		Ok(fichero) => fichero,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hola.txt") {
				Ok(fichero_nuevo) => fichero_nuevo,
				Err(error_creacion) => panic!("Error al crear el fichero: {:?}", error_creacion),
			},
			other_error => {
				panic!("Error al abrir el fichero: {:?}", other_error)
			}
		},
	};

}

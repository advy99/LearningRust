use std::fs::File;
use std::fs;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
	
	// abrimos el archivo, si no existe
	let f = File::open("hola.txt").unwrap_or_else(|error| {
		// si no existe, lo intentamos crear
		if error.kind() == ErrorKind::NotFound {
			File::create("hola.txt").unwrap_or_else(|error| {
				panic!("Error al crear el fichero: {:?}", error);
			})
		} else {
			panic!("Error al abrir el fichero: {:?}", error)
		}	
	});

	// también podemos utilizar expect, si open devuelve un error
	// let f = File::open("hola.txt").expect("No se ha podido abrir hola.txt");

}

fn read_username_from_file() -> Result<String, io::Error> {
	let mut file = File::open("hola.txt")?;
	let mut s = String::new();

	file.read_to_string(&mut s)?;
	Ok(s)
}

fn read_username_from_file_short() -> Result<String, io::Error> {
	let mut s = String::new();

	File::open("hola.txt")?.read_to_string(&mut s)?;
	Ok(s)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
	fs::read_to_string("hola.txt")
}

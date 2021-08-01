fn main() {
	// vector vacio
	let vector: Vec<i32> = Vec::new();

	let vector_inferencia_tipo = vec![1, 2, 3, 4, 5, 6];

	let mut vector_mutable = Vec::new();

	vector_mutable.push(5);
	vector_mutable.push(2);

	{
		let vector_fuera_scope = vec![1];
	} // aqui vector_fuera_scope sale de scope y se libera su memoria

	let tercer_elemento: &i32 = &vector_inferencia_tipo[2];
	println!("El tercer elemento es: {}", tercer_elemento);

	match vector.get(2) {
	    Some(valor) => println!("El tercer elemento es: {}", valor),
	    None => println!("No hay tercer elemento"),
	}

}

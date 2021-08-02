fn largest_number(lista: &Vec<i32>) -> i32 {
	let mut largest = lista[0];

	for &number in lista {
		if number > largest {
			largest = number;
		}
	}

	largest
}

fn main() {
	let lista_numeros = vec![23, 34, 63,123, 234010, 3];

	let largest = largest_number(&lista_numeros);

	println!("La lista de numeros es: {:?}", lista_numeros);
	
	println!("El número más grande de la lista es: {}", largest);
}

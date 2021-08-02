// esto de momento no funciona, ya veremos más adelante como
/* fn largest<T>(lista: &[T]) -> T {
	let mut largest = lista[0];

	for &number in lista {
		if number > largest {
			largest = number;
		}
	}

	largest
} */

struct Point<T, U> {
	x: T,
	y: U,
}

impl<T, U> Point<T, U> {
	fn x(&self) -> &T {
		&self.x
	}

	fn y(&self) -> &U {
		&self.y
	}

	fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
		Point {
			x: self.x,
			y: other.y,
		}
	}
}

fn main() {
	/* let lista_numeros = vec![23, 34, 63,123, 234010, 3];

	let largest_number = largest(&lista_numeros);

	println!("La lista de numeros es: {:?}", lista_numeros);
	
	println!("El número más grande de la lista es: {}", largest_number);

	let lista_char = vec!['y', 'm', 'q'];

	let largest_char = largest(&lista_char);


	println!("La lista de caracteres es: {:?}", lista_char);
	
	println!("El char más grande de la lista es: {}", largest_char);
 */

	let p1 = Point {x: 5, y: 10.5};
	let p2 = Point {x: "Hello", y: 'c'};

	let p3 = p1.mixup(p2);

	println!("p3.x = {}. p3.y = {}", p3.x, p3.y);
}

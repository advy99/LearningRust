fn main() {

	// variable mutable

	let mut x = 5;
	println!("El valor de x es: {}", x);

	x = 6;
	println!("El valor de x es: {}", x);


	// shadowing

	let y = 5;
	let y = y + 1;
	let y = y * 2;

	println!("El valor de y es: {}", y);

	// ejemplo de tupla
	println!("\nPasamos a tuplas");

	let tuple: (i32, f64, bool) = (23, 34.32, false);

	let (_, y, _) = tuple;

	println!("El valor de y es: {}", y);
	println!("El valor de x es: {}", tuple.0);

	// arrays 
	
	let months = ["Ene", "Feb", "Mar", "Abr", "May", "Jun", "Jul", "Ago", "Sep", "Oct", "Nov", "Dic"];
	
	println!("Primer mes: {}", months[0]);

	// array con cinco veces tres
	// let a = [3; 5];

	// array en el que decimos tipo y tamaño
	let a: [i32; 5] = [1, 2, 3, 4, 5];

	// acceso a un array como toda la vida

	println!("El primer elemento es {} y el segundo es {}", a[0], a[1]);

}

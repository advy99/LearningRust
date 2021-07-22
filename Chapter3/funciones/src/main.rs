fn main() {
	otra_funcion(5, 123);


	// prueba de macro

	let y = {
		let x = 3;
		x + 1
	};

	println!("En main y vale: {}", y);

	println!("Llamando a devolver_cinco: {}", devolver_cinco());
	println!("Llamando a sumar_uno(y): {}", sumar_uno(y));


}

fn otra_funcion(x: i32, y:i32) {
	println!("El valor de x es: {}", x);
	println!("El valor de y es: {}", y);

}

fn devolver_cinco() -> i32 {
	5
}

fn sumar_uno(x: i32) -> i32 {
	x + 1
}

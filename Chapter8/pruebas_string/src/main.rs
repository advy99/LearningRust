fn main() {
	let mut string_vacio = String::new();

	let string = "string como slice".to_string();

	let string_con_from = String::from("mi string");

	string_vacio.push_str("Primero");
	string_vacio.push_str(" segundo ");

	string_vacio.push('c');

	println!("{}", string_vacio);

	let s2 = String::from(" ahora voy a usar +");

	// string se ha movido, así que no lo podemos utilizar
	let s3 = string + &s2;

	println!("{}", s3);

	// ERROR: se ha movido
	// println!("{}", string);
	
	// sin embargo podemos usar s2, porque al sumar hemos usado una referencia
	println!("{}", s2);


	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	let s = s1 + "-" + &s2 + "-" + &s3;

	println!("{}", s);
	// println!("{}", s1); // s1 lo ha movido
	// pero los demás no
	println!("{}", s2);
	println!("{}", s3);

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	// otra forma de hacerlo
	let s = format!("{}-{}-{}", s1, s2, s3);

	println!("{}", s);
	println!("{}", s1); // s1 NO lo ha movido
	println!("{}", s2);
	println!("{}", s3);
}

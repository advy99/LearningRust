fn main() {
	let s = String::from("Hello");

	takes_ownership(s);

	// error, ahora s no tiene este scope
	// println!("En main: {}", s);

	let x = 4;

	makes_copy(x);

	let _s = gives_ownership();

	let s1 = String::from("Hello");

	let s2 = takes_and_gives_back(s1);

	println!("{}", s2);

	let (s2, len) = calculate_length(s2);

	println!("Length of {}: {}", s2, len);

}

fn takes_ownership(string: String) {
	println!("{}", string);
}

fn makes_copy(integer: i32) {
	println!("{}", integer);
}

fn gives_ownership() -> String {
	let str = String::from("Hello");

	str
}

fn takes_and_gives_back(string: String) -> String {
	string
}

fn calculate_length(s: String) -> (String, usize) {
	let length = s.len();

	(s, length)
}

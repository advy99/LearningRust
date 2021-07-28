fn main() {
	let s1 = String::from("hello");

	let len = calculate_length(&s1);

	println!("La longitud de '{}' es {}.", s1, len);

	let mut str = String::from("Hello");

	change(&mut str);

	println!("Mutable: {}", str);

	println!("La longitud de '{}' es {}.", str, calculate_length(&str));

}

fn calculate_length(s: &String) -> usize {
	s.len()
}

fn change(str: &mut String) {
	str.push_str(", world");
}

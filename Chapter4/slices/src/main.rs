fn main() {
	let hola_mundo = "Hello world";

	let word = first_word_good(&hola_mundo);

	println!("{}", word);

	println!("Ahora usando un String y no str");

	let from_string = String::from("hello world");

	let word = first_word_good(&from_string[..]);

	println!("{}", word);

}

// Mal, devolvemos un indice que luego puede modificarse en el string
fn first_word(string: &str) -> usize {
    let bytes = string.as_bytes();
	let mut position = 0;
	let mut found = false;

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' && !found {
			position = i;
			found = true;
		}
	}

	if position > string.len() {
		position = string.len()
	} 

    position
}

fn first_word_good(string: &str) -> &str {

    &string[0..first_word(&string)]
    
}

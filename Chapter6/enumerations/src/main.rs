enum IpAddr {
	V4 (u8, u8, u8, u8),
	V6 (String),
}

enum Message {
	Quit,
	Move { x: i32, y: i32},
	Write (String),
	ChangeColor (i32, i32, i32),
}


impl Message {
	fn call (&self) {
		// TODO: Write call
	}
}

fn main() {

	let home = IpAddr::V4(127, 0, 0,1);

	let loopback = IpAddr::V6(String::from("::1"));

	let message = Message::Write(String::from("hello"));

	message.call();


	// algunos ejemplos de Option
	let numero = Some(5);
	let string = Some("string");

	let numero_none: Option<i32> = None;

	if numero_none.is_some() {
		println!("5 + {} = {}", numero_none.unwrap(), 5 + numero_none.unwrap());
	} else {
		println!("No se puede sumar con None");
	}

}

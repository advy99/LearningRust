#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,

}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("Es del estado: {:?}", state);
			25
		},
	}
}

fn sumar_uno (x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}

fn placeholder (x: u8) {
	match x {
		1 => println!("Uno"),
		_ => (),
	}
}


fn main() {
	let moneda = Coin::Penny;
	let quarter = Coin::Quarter(UsState::Alabama);


	println!("Un penique en céntimos es: {}", value_in_cents(moneda));
	println!("Un quarter en céntimos es: {}", value_in_cents(quarter));


	let five = Some(5);
	let six = sumar_uno(five);
	let none = sumar_uno(None);

}

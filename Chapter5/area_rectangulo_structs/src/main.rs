#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {

	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
		}
	}

	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

}

fn main() {
	let rectangle1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!("El area del rectangulo es: {}", rectangle1.area());

	println!("El rectangulo es: {:#?}", rectangle1);

	let rectangle2 = Rectangle {
		width: 10,
		height: 40,
	};

	let rectangle3 = Rectangle {
		width: 60,
		height: 45,
	};

	println!("Puede el rectangulo 1 almacenar al rectangulo 2? {}", rectangle1.can_hold(&rectangle2));
	println!("Puede el rectangulo 1 almacenar al rectangulo 3? {}", rectangle1.can_hold(&rectangle3));

	let cuadrado = Rectangle::square(3);
	println!("El cuadrado es: {:#?}", cuadrado);
}


#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

}

fn main() {
	let rectangle1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!("El area del rectangulo es: {}", rectangle1.area());

	println!("El rectangulo es: {:#?}", rectangle1);

}


#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}

fn main() {
	let rectangle1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!("El area del rectangulo es: {}", area(&rectangle1));

	println!("El rectangulo es: {:#?}", rectangle1);

}


struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}


fn build_user(email: String, username: String) -> User {
	User {
		email: email,
		username: username,
		active: true,
		sign_in_count: 1,
	}
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
	// crear un usuario con ciertos valores
	let mut user1 = User {
		email: String::from("ejemplo@url.com"),
		username: String::from("nombreUsuario123"),
		active: true,
		sign_in_count: 0,
	};

	// podemos acceder con .
	user1.email = String::from("nuevo_mail@url.com");

	// creando un usuario con la funcion
	let user2 = build_user(String::from("User231@url.com"), String::from("a"));

	// podemos utilizar los datos de un usuario para otro
	
	let user2 = User {
		email: String::from("anotheremail@a.com"),
		username: String::from("o"),
		active: user1.active,
		sign_in_count: user1.sign_in_count,
	}

	// también podemos concatenando el resto:
	let user2 = User {
		email: String::from("anotheremail@a.com"),
		username: String::from("o"),
		..user1
	}

	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);

}


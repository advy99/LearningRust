mod front_of_house;



mod back_of_house {
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}

	impl Breakfast {
		pub fn summer (toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches"),
			}
		}
	}

	pub enum Appetizer {
		Soup,
		Salad,
	}

}


pub fn eat_at_restaurant () {
	let mut meal = back_of_house::Breakfast::summer("Rye");

	meal.toast = String::from("Wheat");
	println!("La tostada es de {}", meal.toast);

	// Esto no compila, es privado
	// meal.seasonal_fruit = String::from("blueberries");
	
	let order1 = back_of_house::Appetizer::Soup;

}

use crate::front_of_house::hosting;
// use self::front_of_house::hosting;

pub fn eat_at_restaurant_2() {

	hosting::add_to_waitlist;
}

// podemos usar as para evitar conflictos en los nombres
use std::fmt::Result;
use std::io::Result as IoResult;

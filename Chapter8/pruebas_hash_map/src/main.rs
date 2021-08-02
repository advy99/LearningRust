use std::collections::HashMap;

fn main() {
	let mut scores = HashMap::new();

	scores.insert(String::from("Azul"), 10);
	scores.insert(String::from("Amarillo"), 50);

	// tambien podemos hacer un zip
	
	let teams = vec![String::from("Azul"), String::from("Amarillo")];
	let scores_iniciales = vec![10, 50];

	let mut scores: HashMap<_, _> = teams.into_iter().zip(scores_iniciales.into_iter()).collect();


	let key = String::from("Clave");
	let value = String::from("Valor");

	let mut map = HashMap::new();
	map.insert(key, value);

	// ahora key y value son inválidos, ya que los String no implementan Copy
	

	let team_name = String::from("Azul");
	let score_azules = scores.get(&team_name);


	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}

	scores.insert(String::from("Azul"), 1000);

	println!("{:?}", scores);

	scores.entry(String::from("Azul")).or_insert(600);
	scores.entry(String::from("Negro")).or_insert(6010);

	println!("{:?}", scores);

}

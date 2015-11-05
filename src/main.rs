use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	for greeting in &args[1..] {
		println!("Hello '{}'", greeting);
	}
}

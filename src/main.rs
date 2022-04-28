use rum::rum::vm;

fn main() {
	
	let mut rum = vm::new();
	rum.boot();
	rum.run();

}
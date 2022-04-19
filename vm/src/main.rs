use std::env;
use std::fs::File;
use std::io::Read;
use rum::virtual_machine::vm;
use std::collections::HashMap;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut grouped_bytes: Vec<[u8; 4]> = vec![];
    
    let file = File::open(filename).unwrap();
	let bytes = file.bytes().map(|wrapped_byte| wrapped_byte.unwrap());
	
	for chunk in bytes.collect::<Vec<u8>>().chunks(4) {
		grouped_bytes.push([
			chunk[0],
			chunk[1],
			chunk[2],
			chunk[3],
		]);
	}
	let words: Vec<u32> = grouped_bytes
			.into_iter()
			.map(|byte_group| u32::from_be_bytes(byte_group))
			.collect();
    
    let mut rum = vm {
    	regs: vec![0_u32; 8],
    	// war crime
    	memory: HashMap::new(),
    	unmapped_segments: vec![],
    	max_mapped_segment: 0,
    	program_counter: 0,
    	is_running: true
    };
    rum.memory.insert(0, words.clone());
    
    let mut instruction: u32;
    while rum.is_running {
    	instruction = rum.fetch();
    	disas(instruction);
    	rum.execute(instruction);
    }
}
// Accepts an instruction as input so run this on each instruction right before executing it
fn disas(word: u32){
	let word_u64 = word as u64;
	let opcode = bitpack::bitpack::getu(word_u64, 4, 28);
	let a = bitpack::bitpack::getu(word_u64, 3, 6);
	let b = bitpack::bitpack::getu(word_u64, 3, 3);
	let c = bitpack::bitpack::getu(word_u64, 3, 0);
	
	match opcode {
		0  => println!("cdm %r{}, %r{}, %r{}", a, b, c),
		1  => println!("sgl %r{}, %r{}, %r{}", a, b, c),
		2  => println!("sgm %r{}, %r{}, %r{}", a, b, c),
		3  => println!("add %r{}, %r{}, %r{}", a, b, c),
		4  => println!("mul %r{}, %r{}, %r{}", a, b, c),
		5  => println!("div %r{}, %r{}, %r{}", a, b, c),
		6  => println!("nan %r{}, %r{}, %r{}", a, b, c),
		7  => println!("hal"),
	 	8  => println!("msg %r{}, %r{}", b, c),
		9  => println!("usg %r{}", c),
		10 => println!("out %r{}", c),
		11 => println!("inp %r{}", c),
		12 => println!("ldp %r{}, %r{}", b, c),
		13 => {
			let value = bitpack::bitpack::getu(word_u64, 25, 0) as u32;
			let r = bitpack::bitpack::getu(word_u64, 3, 25);
			println!("ldv %r{}, {}", r, value)},
		_ => println!("Invalid Instruction")
	}
}

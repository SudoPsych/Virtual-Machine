use crate::instructions::*;
use std::env;
use std::io::Read;
#[allow(non_camel_case_types)]

pub struct vm {
	pub regs: Vec<u32>,
	pub memory: Vec<Vec<u32>>,
	pub unmapped_segments: Vec<usize>,
	pub max_mapped_segment: usize,
	pub program_counter: u32
}

impl vm {
	
	pub fn new() -> Self {
		vm {
			regs: vec![0_u32; 8],
			memory: vec![],
			unmapped_segments: vec![],
			max_mapped_segment: 0,
			program_counter: 0
		}
	}
	
	pub fn boot(&mut self) {
		
		let args: Vec<String> = env::args().collect();
	    let user_input: Option<&str>;
	    
	    if args.len() == 2 {
	    	user_input = Some(&args[1]);
	    } else {
	    	user_input = None;
	    }
	    
	    // Professor Daniel's Code from RUMDump lab
	    // -------------------------------------------------------------
	    let mut raw_reader: Box<dyn std::io::BufRead> = match user_input {
			None => Box::new(std::io::BufReader::new(std::io::stdin())),
			Some(filename) => Box::new(std::io::BufReader::new(
				std::fs::File::open(filename).unwrap(),
			)),
		};
		
		let mut buf = Vec::<u8>::new();
		raw_reader.read_to_end(&mut buf).unwrap();
		
		let instructions: Vec<u32> = buf
			.chunks_exact(4)
			.map(|x| u32::from_be_bytes(x.try_into().unwrap()))
			.collect();
		// -------------------------------------------------------------
		self.memory.push(instructions); 
	}
	
	pub fn run(&mut self) {
		loop {
			let instruction = self.fetch();
			self.execute(instruction);
		}
	}
	
	fn fetch(&mut self) -> u32 {
		let instruction = self.memory[0][self.program_counter as usize];
		self.program_counter += 1;
		instruction
	}

	fn execute(&mut self, word: u32) {
		// decode
		let opcode = (word >> 28) & (1 << 4) - 1;
		// execute
		match opcode {
			0 =>  cdl(self, word),
			1 =>  sgl(self, word),
			2 =>  sgs(self, word),
			3 =>  add(self, word),
			4 =>  mul(self, word),
			5 =>  div(self, word),
			6 =>  nan(self, word),
			7 =>  hal(), // ugly :(
			8 =>  msg(self, word),
			9 =>  usg(self, word),
			10 => out(self, word),
			11 => inp(self, word),
			12 => ldp(self, word),
			13 => ldv(self, word),
			_ => panic!("Invalid instruction.")
		};
	}
}
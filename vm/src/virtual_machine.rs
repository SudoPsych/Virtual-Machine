use crate::instr::*;
use std::collections::HashMap;
#[allow(non_camel_case_types)]

pub struct vm {
	pub regs: Vec<u32>,
	pub memory: HashMap<u32, Vec<u32>>,
	pub unmapped_segments: Vec<u32>,
	pub max_mapped_segment: u32,
	pub program_counter: u32,
	pub is_running: bool
}

impl vm {
	
	pub fn fetch(&mut self) -> u32 {
		let vec_at_location = self.memory.get(&0).unwrap();
		let instruction = vec_at_location[self.program_counter as usize];
		self.program_counter += 1;
		instruction
	}

	pub fn execute(&mut self, word: u32){
		
		// decode
		let opcode: u8 = bitpack::bitpack::getu(word.into(), 4, 28).try_into().unwrap();
		let word_u32: u32 = word.try_into().unwrap();
		// execute
		match opcode {
			// could match on bits to get O(log(n)) time rather than current O(n)
			// but like probably doesn't matter
			0 =>  cdl(self, word_u32),
			1 =>  sgl(self, word_u32),
			2 =>  sgs(self, word_u32),
			3 =>  add(self, word_u32),
			4 =>  mul(self, word_u32),
			5 =>  div(self, word_u32),
			6 =>  nan(self, word_u32),
			7 =>  hal(self),
			8 =>  msg(self, word_u32),
			9 =>  usg(self, word_u32),
			10 => out(self, word_u32),
			11 => inp(self, word_u32),
			12 => ldp(self, word_u32),
			13 => ldv(self, word_u32),
			_ => panic!("Invalid instruction.")
			
		};
	}
}
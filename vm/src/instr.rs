use crate::virtual_machine::vm;
use std::io;
use std::io::Read;

fn unpack_abc_regs(word: u32) -> (usize, usize, usize) {
	let word_u64 = word as u64;
	let a = bitpack::bitpack::getu(word_u64, 3, 6).try_into().unwrap();
	let b = bitpack::bitpack::getu(word_u64, 3, 3).try_into().unwrap();
	let c = bitpack::bitpack::getu(word_u64, 3, 0).try_into().unwrap();
	(a, b, c)
}

fn unpack_bc_regs(word: u32) -> (usize, usize) {
	let word_u64 = word as u64;
	let b = bitpack::bitpack::getu(word_u64, 3, 3).try_into().unwrap();
	let c = bitpack::bitpack::getu(word_u64, 3, 0).try_into().unwrap();
	(b, c)	
}

fn unpack_c_reg(word: u32) -> usize {
	bitpack::bitpack::getu(word as u64, 3, 0).try_into().unwrap()
}

pub fn cdl(vm: &mut vm, word: u32) {
	// 0
	// Conditional Load
	let (a, b, c) = unpack_abc_regs(word);
	
	if vm.regs[c] != 0 {
		vm.regs[a] = vm.regs[b];
	}
}

pub fn sgl(vm: &mut vm, word: u32) {
	// 1
	// Segmented Load
	let (a, b, c) = unpack_abc_regs(word);
	vm.regs[a] = vm.memory[vm.regs[b] as usize][vm.regs[c] as usize];
}

pub fn sgs(vm: &mut vm, word: u32) {
	// 2
	// Segmented Store
	let (a, b, c) = unpack_abc_regs(word);
	vm.memory[vm.regs[a] as usize][vm.regs[b] as usize] = vm.regs[c];
}

pub fn add(vm: &mut vm, word: u32) {
	// 3
	// Add
	let (a, b, c) = unpack_abc_regs(word);
	vm.regs[a] = ((vm.regs[b] as u64 + vm.regs[c] as u64) % (1_u64 << 32)).try_into().unwrap();
}

pub fn mul(vm: &mut vm, word: u32) {
	// 4
	// Multiply
	let (a, b, c) = unpack_abc_regs(word);
	vm.regs[a] = ((vm.regs[b] as u64 * vm.regs[c] as u64) % (1_u64 << 32)).try_into().unwrap();
}

pub fn div(vm: &mut vm, word: u32) {
	// 5
	// Divide
	let (a, b, c) = unpack_abc_regs(word);
	vm.regs[a] = vm.regs[b] / vm.regs[c];	
}

pub fn nan(vm: &mut vm, word: u32) {
	// 6
	// Bitwise NAND
	let (a, b, c) = unpack_abc_regs(word);
	vm.regs[a] = !(vm.regs[b] & vm.regs[c]);	
}

pub fn hal(vm: &mut vm) {
	// 7
	// Halt
	vm.is_running = false;
}

pub fn msg(vm: &mut vm, word: u32) {
	// 8
	// Map Segment
	let (b, c) = unpack_bc_regs(word);
	
	if vm.unmapped_segments.len() != 0 {
		let segment_number = vm.unmapped_segments.pop().unwrap();
		vm.memory[segment_number] = vec![0; vm.regs[c] as usize];
		vm.regs[b] = segment_number as u32;
	} else {
		vm.max_mapped_segment += 1;
		vm.memory.push(vec![0; vm.regs[c] as usize]);
		vm.regs[b] = vm.max_mapped_segment as u32;
	}
}

pub fn usg(vm: &mut vm, word: u32) {
	// 9
	// Unmap Segment
	let c = unpack_c_reg(word);
	
	vm.memory[vm.regs[c] as usize].clear();
	vm.unmapped_segments.push(vm.regs[c].try_into().unwrap());
}

pub fn out(vm: &mut vm, word: u32) {
	// 10
	// Output
	let c = unpack_c_reg(word);
	print!("{}", vm.regs[c] as u8 as char);
}

pub fn inp(vm: &mut vm, word: u32) {
	// 11
	// Input
	let c = unpack_c_reg(word);
	let mut buffer: [u8; 1] = [0; 1];
	let num = io::stdin().read(&mut buffer);
	let value = match num {
		Ok(byte) => byte as u32,
		Err(_) => !0_u32
	};
	vm.regs[c] = value;
}

pub fn ldp(vm: &mut vm, word: u32) {
	// 12
	// Load Program
	let (b, c) = unpack_bc_regs(word);
	
	if vm.regs[b] != 0 {
		vm.memory[0] = vm.memory[vm.regs[b] as usize].clone();
	}
	vm.program_counter = vm.regs[c];
}

pub fn ldv(vm: &mut vm, word: u32) {
	// 13
	// Load Value
	let value = bitpack::bitpack::getu(word as u64, 25, 0) as u32;
	let a = bitpack::bitpack::getu(word as u64, 3, 25);
	
	vm.regs[a as usize] = value;
}

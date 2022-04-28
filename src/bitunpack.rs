// Also from RUMDump lab

pub struct Field {
	width: u32,
	lsb: u32,
}

pub static RA: Field = Field {width: 3, lsb: 6};
pub static RB: Field = Field {width: 3, lsb: 3};
pub static RC: Field = Field {width: 3, lsb: 0};
pub static RL: Field = Field {width: 3, lsb: 25};
pub static VL: Field = Field {width: 25, lsb: 0};

fn mask(bits: u32) -> u32 {
	(1 << bits) - 1
}
pub fn get(field: &Field, instruction: u32) -> usize {
	((instruction >> field.lsb) & mask(field.width)).try_into().unwrap()
}
use std::fmt;

/// Represent the Gameboy's CPU
/// All the 16 bit registers, with the exception of the stack pointer and program counter,
/// can be accessed either as a single 16 bit register or as two 8 bit reisters, addressing
/// the high and low 8 bits separately. The lower 8 bits of the AF register are used
/// for CPU flags: C(arry), H(alf carry), S(ubtraction), Z(ero). 
pub struct CPU {
	af: u16,
	bc: u16,
	de: u16,
	hl: u16,
	sp: u16,
	pc: u16,	
}


impl fmt::Display for CPU {
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "AF: {:b}\nBC: {:b}\nDE: {:b}\nHL: {:b}\nSP: {:b}\nPC: {:b}\n", self.af, self.bc, self.de, self.hl, self.sp, self.pc)
	}
}

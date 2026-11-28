use env::State;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Program {
    pub inst: [u8; 8],
}

impl Program {
    pub fn from_u64(code: u64) -> Self {
        Self {
            inst: code.to_be_bytes(),
        }
    }

    pub fn to_u64(&self) -> u64 {
        u64::from_be_bytes(self.inst)
    }

    //dst = b + c
    fn exec_inst(inst: u8, reg: &mut [f32; 4], state: State) {
        let reg_b: [f32; 4] = [
            reg[0], reg[1], state[2], state[3], 
        ];

        let reg_c: [f32; 4] = [
            reg[2], reg[3], state[0], state[1], 
        ];


        let op = (inst >> 6) & 0b11;
        let dst = ((inst >> 4) & 0b11) as usize;
        let b = ((inst >> 2) & 0b11) as usize;
        let c = (inst & 0b11) as usize;

        let val_b = reg_b[b];
        let val_c = reg_c[c];

        let result = match op {
            0b00 => val_b + val_c,
            0b01 => val_b - val_c,
            0b10 => val_b * val_c,
            0b11 => (val_b * val_c).cos(),
            _ => unreachable!(),
        };

        reg[dst] = if result.is_finite() { result } else { 0.0 };
    }

    pub fn action(&self, state: State) -> f32 {
        let mut reg = [0.0f32; 4];
        for &byte in &self.inst {
            Self::exec_inst(byte, &mut reg, state);
        }
        reg[0]
    }

    pub fn mutate<R: Rng>(&self, rng: &mut R, bit_flip_prob: f64) -> Self {
        let mut bits = self.to_u64();
        for bit in 0..64 {
            if rng.random_bool(bit_flip_prob) {
                bits ^= 1u64 << bit;
            }
        }
        Self::from_u64(bits)
    }
}
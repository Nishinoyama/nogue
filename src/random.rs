trait Random {
    fn gen(&mut self) -> u32;
    fn gen_u8(&mut self) -> u8 {
        (self.gen() & 0xff) as u8
    }
}

struct LfsrRandom {
    state: u32,
}

impl Random for LfsrRandom {
    fn gen(&mut self) -> u32 {
        let state = self.state;
        let res = (state & 0x7f800000).wrapping_shr(23) ^ (state & 0x0003fc00).wrapping_shr(10);
        self.state = self.state.wrapping_shl(8) | res;
        res
    }
}

impl Default for LfsrRandom {
    fn default() -> Self {
        LfsrRandom {
            state: 0x17291729u32,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::random::{LfsrRandom, Random};

    #[test]
    fn rng() {
        let mut t = LfsrRandom::default();
        let mut cnt = [0usize; 256];
        let mean = 10000;
        for _ in 0..mean * 256 {
            cnt[t.gen_u8() as usize] += 1;
        }
        for (i, &c) in cnt.iter().enumerate() {
            println!("{:3} -> {:5}({})", i, c, (c as f64) / (mean as f64));
        }
    }
}

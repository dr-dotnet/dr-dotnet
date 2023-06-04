use std::hash::Hasher;
        
pub struct SimpleHasher(u64);

impl Default for SimpleHasher {
    fn default() -> SimpleHasher {
        SimpleHasher(0)
    }
}

impl Hasher for SimpleHasher {

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, _bytes: &[u8]) { 
        panic!("Not supposed to be called");
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        let mut hash = i as u64;
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xff51afd7ed558ccd);
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xc4ceb9fe1a85ec53);
        hash ^= hash >> 33;
        self.0 = hash;
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        let mut hash = i;
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xff51afd7ed558ccd);
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xc4ceb9fe1a85ec53);
        hash ^= hash >> 33;
        self.0 = hash;
    }
}
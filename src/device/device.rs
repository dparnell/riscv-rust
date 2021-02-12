
/// A device implements this trait
pub trait Device {
    fn store_u8(&mut self, p_address: u64, value: u8);

    fn store_u16(&mut self, p_address: u64, value: u16) {
        for i in 0..2 {
            self.store_u8(p_address.wrapping_add(i), ((value >> (i * 8)) & 0xff) as u8);
        }
    }
    fn store_u32(&mut self, p_address: u64, value: u32) {
        for i in 0..4 {
            self.store_u8(p_address.wrapping_add(i), ((value >> (i * 8)) & 0xff) as u8);
        }
    }
    fn store_u64(&mut self, p_address: u64, value: u64) {
        for i in 0..8 {
            self.store_u8(p_address.wrapping_add(i), ((value >> (i * 8)) & 0xff) as u8);
        }
    }

    fn load_u8(&mut self, p_address: u64) -> u8;

    fn load_u16(&mut self, p_address: u64) -> u16 {
        let mut data = 0 as u16;
        for i in 0..2 {
            data |= (self.load_u8(p_address.wrapping_add(i)) as u16) << (i * 8)
        }
        data
    }

    fn load_u32(&mut self, p_address: u64) -> u32 {
        let mut data = 0 as u32;
        for i in 0..4 {
            data |= (self.load_u8(p_address.wrapping_add(i)) as u32) << (i * 8)
        }
        data
    }

    fn load_u64(&mut self, p_address: u64) -> u64 {
        let mut data = 0 as u64;
        for i in 0..8 {
            data |= (self.load_u8(p_address.wrapping_add(i)) as u64) << (i * 8)
        }
        data
    }
}
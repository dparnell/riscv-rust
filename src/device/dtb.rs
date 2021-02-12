use device::device::Device;

pub const DTB_SIZE: usize = 0xfe0;

pub struct Dtb {
    base_address: u64,
    data: Vec<u8>
}

impl Dtb {
    pub fn new(base_address: u64, content: Vec<u8>) -> Self {
        assert!(content.len() <= DTB_SIZE, "DTB is too big");
        let mut dtb = vec![0; DTB_SIZE];
        for i in 0..content.len() {
            dtb[i] = content[i];
        }

        Dtb {
            base_address,
            data: dtb
        }
    }

    pub fn init(&mut self, data: Vec<u8>) {
        for i in 0..data.len() {
            self.data[i] = data[i];
        }
        for i in data.len()..DTB_SIZE {
            self.data[i] = 0;
        }
    }
}

impl Device for Dtb {
    fn store_u8(&mut self, _p_address: u64, _value: u8) {
        // do nothing
    }

    fn load_u8(&mut self, p_address: u64) -> u8 {
        self.data[(p_address - self.base_address) as usize]
    }
}
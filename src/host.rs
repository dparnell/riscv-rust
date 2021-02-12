use device::device::Device;

pub struct DummyHost {}

impl DummyHost {
    pub fn new() -> Self {
        DummyHost{}
    }
}

impl Device for DummyHost {
    fn store_u8(&mut self, _p_address: u64, _value: u8) {
        // do nothing
    }

    fn load_u8(&mut self, _p_address: u64) -> u8 {
        0
    }
}
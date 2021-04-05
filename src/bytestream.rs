pub struct ByteStream {
    number: u8,
    use_count: u8,
}

impl ByteStream {
    pub fn new(number: u8) -> Self {
        ByteStream {
            number,
            use_count: 0,
        }
    }

    pub fn consume_bit(&mut self) -> u8 {
        if self.is_consumed() {
            return 0;
        }
        let bit = self.number & 1;
        self.number >>= 1;
        self.use_count += 1;
        bit
    }

    pub fn is_consumed(&self) -> bool {
        self.use_count == 8
    }
}

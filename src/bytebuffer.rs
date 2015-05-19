pub struct ByteBuffer(Vec<u8>);

impl ByteBuffer {
    pub fn new(cap: usize) -> ByteBuffer {
        ByteBuffer(Vec::with_capacity(cap))
    }
}

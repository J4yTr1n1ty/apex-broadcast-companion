use bytes::Buf;

pub(crate) struct ByteVector {
    pub(crate) inner: Vec<u8>,
}

impl Buf for ByteVector {
    fn remaining(&self) -> usize {
        self.inner.len()
    }

    fn chunk(&self) -> &[u8] {
        &self.inner
    }

    fn advance(&mut self, cnt: usize) {
        self.inner.drain(..cnt);
    }
}

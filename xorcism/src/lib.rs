use std::borrow::Borrow;

#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    iter_idx: usize,
}

impl<'a, 'b, 'c> Xorcism<'a> {
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: 'a + AsRef<[u8]> + ?Sized,
    {
        Xorcism {
            key: key.as_ref(),
            iter_idx: 0,
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for data_byte in data.iter_mut() {
            *data_byte ^= self.key[self.iter_idx];
            self.iter_idx = (self.iter_idx + 1) % self.key.len();
        }
    }

    pub fn munge<Data, I>(&'c mut self, data: Data) -> impl Iterator<Item = u8> + 'b
    where
        I: Borrow<u8>,
        Data: IntoIterator<Item = I> + 'b,
        'a: 'b,
        'a: 'c,
        'c: 'b,
    {
        data.into_iter().map(|data_byte| {
            let key_byte = self.key[self.iter_idx];
            self.iter_idx = (self.iter_idx + 1) % self.key.len();
            *data_byte.borrow() ^ key_byte
        })
    }
}

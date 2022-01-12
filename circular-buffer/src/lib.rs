#[derive(Default)]
pub struct CircularBuffer<T> {
    buf: Vec<Option<T>>,
    read_p: usize,
    write_p: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buf: (0..capacity).map(|_| None).collect::<Vec<Option<T>>>(),
            read_p: 0,
            write_p: 0
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        let target = &mut self.buf[self.write_p];
        match target {
            Some(_) => Err(Error::FullBuffer),

            None => {
                *target = Some(_element);
                self.write_p = (self.write_p + 1) % self.buf.len();
                Ok(())
            }
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        let target = self.buf[self.read_p].take();
        match target {
            Some(e) => {
                self.read_p = (self.read_p + 1) % self.buf.len();
                Ok(e)
            }

            None => Err(Error::EmptyBuffer)
        }
    }

    pub fn clear(&mut self) {
        for e in self.buf.iter_mut() {
            e.take();
        }

        self.read_p = 0;
        self.write_p = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        let target = &mut self.buf[(self.write_p)];
        *target = Some(_element);
        if self.write_p == self.read_p {
            self.read_p = (self.read_p + 1) % self.buf.len();
        }
        self.write_p = (self.write_p + 1) % self.buf.len();
    }
}

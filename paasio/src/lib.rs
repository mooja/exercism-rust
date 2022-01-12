use std::io::{Read, Result, Write};

// pub struct ReadStats<R>(::std::marker::PhantomData<R>);

pub struct ReadStats<R> {
    target: R,
    bytes_through: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        ReadStats {
            target: _wrapped,
            bytes_through: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.target
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        let r = self.target.read(buf)?;
        self.bytes_through += r;
        Ok(r)
    }
}

pub struct WriteStats<W> {
    target: W,
    bytes_through: usize,
    writes: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        WriteStats {
            target: _wrapped,
            bytes_through: 0,
            writes: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.target
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        let r = self.target.write(buf)?;
        self.bytes_through += r;
        Ok(r)
    }

    fn flush(&mut self) -> Result<()> {
        self.target.flush()
    }
}

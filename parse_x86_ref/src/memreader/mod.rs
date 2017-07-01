use std::cmp;
use std::io;
use std::io::Read;

pub struct MemReader<'a> {
    buffer: &'a Vec<u8>,
    index: usize
}

impl<'a> MemReader<'a> {
    pub fn new(buffer: &'a Vec<u8>) -> MemReader<'a> {
        MemReader {
            buffer: buffer,
            index: 0
        }
    }

    pub fn is_eof(&self) -> bool { self.index >= self.buffer.len() }
}

impl<'a> Read for MemReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_to_read = cmp::min(buf.len(), self.buffer.len() - self.index);
        if bytes_to_read == 0 { 
            return Ok(0)
        }

        buf[0..bytes_to_read].clone_from_slice(&self.buffer[self.index..(self.index+bytes_to_read)]);
        Ok(bytes_to_read)
    }
}

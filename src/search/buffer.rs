use anyhow::Result;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

#[allow(dead_code)]
pub struct BufferManager {
    max_size: usize,
}

#[allow(dead_code)]
impl BufferManager {
    pub fn new(max_size: usize) -> Self {
        Self { max_size }
    }

    pub fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        
        if mmap.len() > self.max_size {
            // Take only the last max_size bytes for recent data
            let start = mmap.len() - self.max_size;
            Ok(mmap[start..].to_vec())
        } else {
            Ok(mmap.to_vec())
        }
    }

    pub fn load_from_string(&self, content: String) -> Result<Vec<u8>> {
        let bytes = content.into_bytes();
        if bytes.len() > self.max_size {
            let start = bytes.len() - self.max_size;
            Ok(bytes[start..].to_vec())
        } else {
            Ok(bytes)
        }
    }

    pub fn chunk_buffer<'a>(&self, buffer: &'a [u8], chunk_size: usize) -> Vec<&'a [u8]> {
        buffer.chunks(chunk_size).collect()
    }
}
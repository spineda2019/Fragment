use std::{fs::File, path::Path};

use common::error::CompilerError;
use memmap2::Mmap;

pub struct CharReader {
    file_map: Mmap,
    byte_pointer: usize,
}

impl CharReader {
    pub fn new(file: &File, file_path: &Path) -> Result<CharReader, CompilerError> {
        let map: Result<Mmap, _> = unsafe { Mmap::map(file) };
        let map: Mmap = match map {
            Ok(m) => m,
            Err(e) => return Err(CompilerError::FileIOError(file_path.to_owned(), e)),
        };

        Ok(CharReader {
            file_map: map,
            byte_pointer: 0,
        })
    }
}

use std::{fs::File, path::Path};

use common::error::CompilerError;
use memmap2::MmapMut;

pub struct CharReader {
    file_map: MmapMut,
}

impl CharReader {
    pub fn new(file: &File, file_path: &Path) -> Result<CharReader, CompilerError> {
        let map: Result<MmapMut, _> = unsafe { MmapMut::map_mut(file) };
        let map: MmapMut = match map {
            Ok(m) => m,
            Err(e) => return Err(CompilerError::FileIOError(file_path.to_owned(), e)),
        };

        Ok(CharReader { file_map: map })
    }
}

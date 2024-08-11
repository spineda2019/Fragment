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

    pub fn getchar(&mut self) -> Option<char> {
        if let Some(c) = self.file_map.get(self.byte_pointer) {
            self.byte_pointer += 1;
            let character: char = char::from(*c);
            return Some(character);
        }

        None
    }
}

#[cfg(test)]
mod charreader_tests {
    use std::{
        env::current_dir,
        fs::{self, File},
        path::PathBuf,
    };

    use common::error::CompilerError;

    use crate::char_reader::CharReader;

    #[test]
    fn test_getchar() {
        let expected: &str;

        #[cfg(target_os = "windows")]
        {
            expected = "I am a group of chars\r\nthat should be consumable\r\n";
        }
        #[cfg(target_os = not("windows"))]
        {
            expected = "I am a group of chars\nthat should be consumable\n";
        }

        let current_dir: Result<PathBuf, _> = current_dir();
        assert!(current_dir.is_ok());
        let mut current_dir: PathBuf = current_dir.unwrap();

        let prev: PathBuf = PathBuf::from("..");
        let test_dir: PathBuf = PathBuf::from("test_utils");
        let test_file: PathBuf = PathBuf::from("char_reader.txt");

        current_dir.push(prev);
        current_dir.push(test_dir);
        current_dir.push(test_file);

        let filepath: PathBuf = current_dir.clone();

        let file: Result<File, _> = File::open(current_dir);
        assert!(file.is_ok());

        let file: File = file.unwrap();

        let reader: Result<CharReader, CompilerError> = CharReader::new(&file, &filepath);
        assert!(reader.is_ok());

        let mut reader: CharReader = reader.unwrap();

        let mut result: String = String::with_capacity(expected.len());

        while let Some(c) = reader.getchar() {
            result.push(c);
        }

        assert_eq!(expected, &result);
    }
}

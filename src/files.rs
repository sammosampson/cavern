use std::fs::*;
use std::fs::ReadDir;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[derive(Debug)]
pub enum FileError {
    DirectoryReadError(String),
    PathError,
    FileOpeningError,
    BufferReadError
}

pub struct FolderFileIterator(ReadDir);

impl Iterator for FolderFileIterator {
    type Item = Result<(String, Vec<u8>), FileError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(read_file_data(self.0.next()?))
    }
}

pub fn read_files_in_folder(folder_path: &str) -> Result<FolderFileIterator, FileError> {
    Ok(FolderFileIterator(read_folder(folder_path)?))
}

fn read_folder(folder_path: &str) -> Result<ReadDir, FileError> {
    Ok(read_dir(folder_path)
        .map_err(|_| FileError::DirectoryReadError(folder_path.to_string()))?)
}

fn read_file_data(path: std::io::Result<DirEntry>) -> Result<(String, Vec<u8>), FileError> {
    let path = get_path(path)?;
    let file_name = filename_from_path(&path)?;
    let file = open_file(path)?;
    let data = read_file(file)?;
    Ok((file_name, data))
}

fn get_path(path: Result<DirEntry, std::io::Error>) -> Result<PathBuf, FileError> {
    Ok(path.map_err(|_| FileError::PathError)?.path())
}

fn filename_from_path(path: &PathBuf) -> Result<String, FileError> {
    Ok(filename_from_path_option(path).ok_or(FileError::PathError)?)
}

fn filename_from_path_option(path: &PathBuf) -> Option<String> {
    Some(path.file_name()?.to_str()?.to_string())
}

fn open_file(path: PathBuf) -> Result<File, FileError> {
    Ok(File::open(path).map_err(|_| FileError::FileOpeningError)?)
}

fn read_file(file: File) -> Result<Vec<u8>, FileError> {
    let mut buf_reader = BufReader::new(file);
    let mut data: Vec<u8> = vec!();
    buf_reader.read_to_end(&mut data).map_err(|_| FileError::BufferReadError)?;
    Ok(data)
}
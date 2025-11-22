use std::path::Path;

pub trait FileRepository {
    fn get_file_content(&self, file_path: &Path) -> Result<String, FileRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileRepositoryError {
    Unknown(String),
    InexistentFile,
}

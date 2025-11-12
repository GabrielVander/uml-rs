use std::path::Path;

pub(crate) trait FileRepository {
    fn get_file_content(&self, file_path: &Path) -> Result<String, FileRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FileRepositoryError {
    Unknown(String),
}

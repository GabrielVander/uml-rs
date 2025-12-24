use std::path::Path;

use lib_core::domain::repositories::file_repository::{FileRepository, FileRepositoryError};

struct FileRepositoryLocalFsImpl;

impl FileRepositoryLocalFsImpl {
    fn new() -> Self {
        Self
    }
}

impl FileRepository for FileRepositoryLocalFsImpl {
    fn get_file_content(&self, file_path: &Path) -> Result<String, FileRepositoryError> {
        if !file_path.exists() {
            return Err(FileRepositoryError::InexistentFile);
        }

        std::fs::read_to_string(file_path).map_err(|e| FileRepositoryError::Unknown(e.to_string()))
    }
}

#[cfg(test)]
mod test {
    use std::{io::Write, path::Path};

    use lib_core::domain::repositories::file_repository::{FileRepository, FileRepositoryError};
    use tempfile::NamedTempFile;

    use crate::adapters::repositories::file_repository_local_fs_impl::FileRepositoryLocalFsImpl;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_file_content_should_fail_given_inexisting_file() {
        let path: &Path = Path::new("non_existent_file.txt");
        let expected = Err(FileRepositoryError::InexistentFile);

        let repository: FileRepositoryLocalFsImpl = FileRepositoryLocalFsImpl::new();

        let result = repository.get_file_content(path);

        assert_eq!(expected, result);
    }

    #[test]
    fn get_file_content_should_fail_given_invalid_utf8() {
        let mut file: NamedTempFile = NamedTempFile::new().unwrap();
        file.write_all(&[0, 159, 146, 150]).unwrap(); // Invalid UTF-8 bytes

        let expected = Err(FileRepositoryError::Unknown(
            "stream did not contain valid UTF-8".to_owned(),
        ));

        let repository: FileRepositoryLocalFsImpl = FileRepositoryLocalFsImpl::new();

        let result = repository.get_file_content(file.path());

        assert_eq!(expected, result);
    }
}

use std::{path::Path, sync::Arc};

use crate::domain::{
    entities::diagram::Diagram,
    repositories::{
        diagram_repository::{DiagramRepository, DiagramRepositoryError},
        file_repository::{FileRepository, FileRepositoryError},
    },
};

struct LoadDiagramFromFileUseCase {
    file_repository: Arc<dyn FileRepository>,
    diagram_repository: Arc<dyn DiagramRepository>,
}

impl LoadDiagramFromFileUseCase {
    fn new(
        file_repository: Arc<dyn FileRepository>,
        diagram_repository: Arc<dyn DiagramRepository>,
    ) -> Self {
        Self {
            file_repository,
            diagram_repository,
        }
    }

    fn execute(&self, path: &str) -> Result<Diagram, LoadDiagramFromFileError> {
        Ok(Path::new(path))
            .and_then(|p: &Path| self.load_file(p))
            .and_then(|content: String| self.parse_content(content))
    }

    fn load_file(&self, path: &Path) -> Result<String, LoadDiagramFromFileError> {
        self.file_repository
            .get_file_content(path)
            .map_err(LoadDiagramFromFileError::from)
    }

    fn parse_content(&self, content: String) -> Result<Diagram, LoadDiagramFromFileError> {
        self.diagram_repository
            .parse_from_content(&content)
            .map_err(LoadDiagramFromFileError::from)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LoadDiagramFromFileError {
    FileLoadError(String),
    ParseError(String),
}

impl From<FileRepositoryError> for LoadDiagramFromFileError {
    fn from(value: FileRepositoryError) -> Self {
        match value {
            FileRepositoryError::Unknown(msg) => LoadDiagramFromFileError::FileLoadError(msg),
            FileRepositoryError::InexistentFile => {
                LoadDiagramFromFileError::FileLoadError("Given file does not exist".to_owned())
            }
        }
    }
}

impl From<DiagramRepositoryError> for LoadDiagramFromFileError {
    fn from(value: DiagramRepositoryError) -> Self {
        match value {
            DiagramRepositoryError::Unknown(msg) => LoadDiagramFromFileError::ParseError(msg),
        }
    }
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use crate::domain::{
        entities::diagram::Diagram,
        repositories::{
            diagram_repository::{DiagramRepository, DiagramRepositoryError},
            file_repository::{FileRepository, FileRepositoryError},
        },
        use_cases::load_diagram_from_file::{LoadDiagramFromFileError, LoadDiagramFromFileUseCase},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn should_fail_if_unable_to_load_file() {
        let test_cases = vec![
            (
                FileRepositoryError::Unknown("Some unknown error".to_owned()),
                Err(LoadDiagramFromFileError::FileLoadError(
                    "Some unknown error".to_owned(),
                )),
            ),
            (
                FileRepositoryError::InexistentFile,
                Err(LoadDiagramFromFileError::FileLoadError(
                    "Given file does not exist".to_owned(),
                )),
            ),
        ];

        for (file_repository_error, expected) in test_cases {
            let file_path: &'static str = "some_file.puml";
            let file_repository_result: Result<String, FileRepositoryError> =
                Err(file_repository_error.clone());
            let diagram_repository_result: Option<Result<Diagram, DiagramRepositoryError>> = None;

            let file_repository: Arc<dyn FileRepository> =
                Arc::new(FileRepositoryMockImpl::new(file_repository_result));
            let diagram_repository: Arc<dyn DiagramRepository> =
                Arc::new(DiagramRepositoryMockImpl::new(diagram_repository_result));
            let use_case: LoadDiagramFromFileUseCase = LoadDiagramFromFileUseCase::new(
                file_repository.clone(),
                diagram_repository.clone(),
            );

            let result: Result<Diagram, LoadDiagramFromFileError> = use_case.execute(file_path);

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn should_fail_if_unable_to_parse_file() {
        let file_path: &'static str = "invalid_content_file.puml";
        let error_msg: String = "Some unknown error".to_owned();
        let file_repository: Arc<dyn FileRepository> =
            Arc::new(FileRepositoryMockImpl::new(Ok("Some content".to_owned())));
        let diagram_repository_result: Option<Result<Diagram, DiagramRepositoryError>> =
            Some(Err(DiagramRepositoryError::Unknown(error_msg.clone())));
        let expected: Result<_, LoadDiagramFromFileError> =
            Err(LoadDiagramFromFileError::ParseError(error_msg));

        let diagram_repository: Arc<dyn DiagramRepository> =
            Arc::new(DiagramRepositoryMockImpl::new(diagram_repository_result));
        let use_case: LoadDiagramFromFileUseCase =
            LoadDiagramFromFileUseCase::new(file_repository.clone(), diagram_repository.clone());

        let result: Result<Diagram, LoadDiagramFromFileError> = use_case.execute(file_path);

        assert_eq!(expected, result);
    }

    #[test]
    fn should_succeed_if_able_to_load_and_parse_file() {
        let file_path: &'static str = "valid_content_file.puml";
        let diagram: Diagram = Diagram::default();
        let file_repository_result: Result<String, FileRepositoryError> =
            Ok("Valid content".to_owned());
        let diagram_repository_result: Option<Result<Diagram, _>> = Some(Ok(diagram.clone()));
        let expected: Result<Diagram, LoadDiagramFromFileError> = Ok(diagram.clone());

        let file_repository: Arc<dyn FileRepository> =
            Arc::new(FileRepositoryMockImpl::new(file_repository_result));
        let diagram_repository: Arc<dyn DiagramRepository> =
            Arc::new(DiagramRepositoryMockImpl::new(diagram_repository_result));
        let use_case: LoadDiagramFromFileUseCase =
            LoadDiagramFromFileUseCase::new(file_repository.clone(), diagram_repository.clone());

        let result: Result<Diagram, LoadDiagramFromFileError> = use_case.execute(file_path);

        assert_eq!(expected, result);
    }

    struct FileRepositoryMockImpl {
        result: Result<String, FileRepositoryError>,
    }

    impl FileRepositoryMockImpl {
        fn new(result: Result<String, FileRepositoryError>) -> Self {
            Self { result }
        }
    }

    impl FileRepository for FileRepositoryMockImpl {
        fn get_file_content(
            &self,
            _file_path: &std::path::Path,
        ) -> Result<String, FileRepositoryError> {
            self.result.clone()
        }
    }

    #[derive(Default)]
    struct DiagramRepositoryMockImpl {
        result: Option<Result<Diagram, DiagramRepositoryError>>,
    }

    impl DiagramRepositoryMockImpl {
        fn new(result: Option<Result<Diagram, DiagramRepositoryError>>) -> Self {
            Self { result }
        }
    }

    impl DiagramRepository for DiagramRepositoryMockImpl {
        fn parse_from_content(&self, _content: &str) -> Result<Diagram, DiagramRepositoryError> {
            if let Some(value) = self.result.as_ref() {
                return value.clone();
            }

            panic!("Unexpected call to MockDiagramRepository.parse_from_content")
        }
    }
}

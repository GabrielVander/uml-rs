use std::{path::Path, sync::Arc};

use crate::domain::{
    entities::diagram::Diagram,
    repositories::{
        diagram_repository::{DiagramRepository, DiagramRepositoryError},
        file_repository::{FileRepository, FileRepositoryError},
    },
};

struct LoadDiagramUseCase<T> {
    file_repository: Arc<dyn FileRepository>,
    diagram_repository: Arc<dyn DiagramRepository>,
    presenter: Arc<dyn LoadDiagramPresenter<T>>,
}

impl<T> LoadDiagramUseCase<T> {
    fn new(
        file_repository: Arc<dyn FileRepository>,
        diagram_repository: Arc<dyn DiagramRepository>,
        presenter: Arc<dyn LoadDiagramPresenter<T>>,
    ) -> Self {
        Self {
            file_repository,
            diagram_repository,
            presenter,
        }
    }

    pub fn execute(&self, source: &str) -> T {
        let result: LoadDiagramResult = self
            .load_from_local_fs_file(source)
            .and_then(|content: String| self.parse_content(content));

        self.presenter.present(result)
    }

    fn load_from_local_fs_file(&self, source: &str) -> Result<String, LoadDiagramError> {
        Ok(Path::new(source))
            .and_then(|p: &Path| self.file_repository.get_file_content(p))
            .map_err(LoadDiagramError::from)
    }

    fn parse_content(&self, content: String) -> Result<Diagram, LoadDiagramError> {
        self.diagram_repository
            .parse_from_content(&content)
            .map_err(LoadDiagramError::from)
    }
}

trait LoadDiagramPresenter<T> {
    fn present(&self, result: LoadDiagramResult) -> T;
}

type LoadDiagramResult = Result<Diagram, LoadDiagramError>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum LoadDiagramError {
    FileLoadError(String),
    ParseError(String),
}

impl From<FileRepositoryError> for LoadDiagramError {
    fn from(value: FileRepositoryError) -> Self {
        match value {
            FileRepositoryError::Unknown(msg) => LoadDiagramError::FileLoadError(msg),
            FileRepositoryError::InexistentFile => {
                LoadDiagramError::FileLoadError("Given file does not exist".to_owned())
            }
        }
    }
}

impl From<DiagramRepositoryError> for LoadDiagramError {
    fn from(value: DiagramRepositoryError) -> Self {
        match value {
            DiagramRepositoryError::Unknown(msg) => LoadDiagramError::ParseError(msg),
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
        use_cases::load_diagram::{
            LoadDiagramError, LoadDiagramPresenter, LoadDiagramResult, LoadDiagramUseCase,
        },
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn should_fail_if_unable_to_load_file() {
        let test_cases = vec![
            (
                FileRepositoryError::Unknown("Some unknown error".to_owned()),
                Err(LoadDiagramError::FileLoadError(
                    "Some unknown error".to_owned(),
                )),
            ),
            (
                FileRepositoryError::InexistentFile,
                Err(LoadDiagramError::FileLoadError(
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
            let no_op_presenter: Arc<LoadDiagramPresenterNoOpImpl> =
                Arc::new(LoadDiagramPresenterNoOpImpl);

            let use_case: LoadDiagramUseCase<LoadDiagramResult> = LoadDiagramUseCase::new(
                file_repository.clone(),
                diagram_repository.clone(),
                no_op_presenter,
            );

            let result: LoadDiagramResult = use_case.execute(file_path);

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn should_fail_if_unable_to_parse_file() {
        let file_path: &'static str = "invalid_content_file.puml";
        let error_msg: String = "Some unknown error".to_owned();
        let diagram_repository_result: Option<Result<Diagram, DiagramRepositoryError>> =
            Some(Err(DiagramRepositoryError::Unknown(error_msg.clone())));
        let expected: Result<_, LoadDiagramError> = Err(LoadDiagramError::ParseError(error_msg));

        let file_repository: Arc<dyn FileRepository> =
            Arc::new(FileRepositoryMockImpl::new(Ok("Some content".to_owned())));
        let diagram_repository: Arc<dyn DiagramRepository> =
            Arc::new(DiagramRepositoryMockImpl::new(diagram_repository_result));
        let no_op_presenter: Arc<LoadDiagramPresenterNoOpImpl> =
            Arc::new(LoadDiagramPresenterNoOpImpl);

        let use_case: LoadDiagramUseCase<LoadDiagramResult> =
            LoadDiagramUseCase::new(file_repository, diagram_repository, no_op_presenter);

        let result: LoadDiagramResult = use_case.execute(file_path);

        assert_eq!(expected, result);
    }

    #[test]
    fn should_succeed_if_able_to_load_and_parse_file() {
        let file_path: &'static str = "valid_content_file.puml";
        let diagram: Diagram = Diagram::default();
        let file_repository_result: Result<String, FileRepositoryError> =
            Ok("Valid content".to_owned());
        let diagram_repository_result: Option<Result<Diagram, _>> = Some(Ok(diagram.clone()));
        let expected: LoadDiagramResult = Ok(diagram.clone());

        let file_repository: Arc<dyn FileRepository> =
            Arc::new(FileRepositoryMockImpl::new(file_repository_result));
        let diagram_repository: Arc<dyn DiagramRepository> =
            Arc::new(DiagramRepositoryMockImpl::new(diagram_repository_result));
        let no_op_presenter: Arc<LoadDiagramPresenterNoOpImpl> =
            Arc::new(LoadDiagramPresenterNoOpImpl);

        let use_case: LoadDiagramUseCase<LoadDiagramResult> =
            LoadDiagramUseCase::new(file_repository, diagram_repository, no_op_presenter);

        let result: LoadDiagramResult = use_case.execute(file_path);

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

    #[derive(Default)]
    struct LoadDiagramPresenterNoOpImpl;

    impl LoadDiagramPresenter<LoadDiagramResult> for LoadDiagramPresenterNoOpImpl {
        fn present(&self, result: LoadDiagramResult) -> LoadDiagramResult {
            result
        }
    }
}

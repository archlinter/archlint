use super::{FileType, Framework};
use std::path::Path;

pub struct FileClassifier;

impl FileClassifier {
    pub fn classify(path: &Path, _frameworks: &[Framework]) -> FileType {
        let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
        let path_str = path.to_str().unwrap_or("");

        if let Some(file_type) = Self::classify_test_file(filename, path_str) {
            return file_type;
        }

        if let Some(file_type) = Self::classify_nestjs(filename, path_str) {
            return file_type;
        }

        if let Some(file_type) = Self::classify_nextjs(filename, path_str) {
            return file_type;
        }

        if let Some(file_type) = Self::classify_common(filename, path_str) {
            return file_type;
        }

        if let Some(file_type) = Self::classify_cli(path_str) {
            return file_type;
        }

        if let Some(file_type) = Self::classify_frontend(filename) {
            return file_type;
        }

        FileType::Unknown
    }

    fn classify_test_file(filename: &str, path_str: &str) -> Option<FileType> {
        if filename.ends_with(".test.ts")
            || filename.ends_with(".test.js")
            || filename.ends_with(".spec.ts")
            || filename.ends_with(".spec.js")
            || path_str.contains("/__tests__/")
            || path_str.contains("/tests/")
            || path_str.contains("/test/")
        {
            Some(FileType::Test)
        } else {
            None
        }
    }

    fn classify_nestjs(filename: &str, path_str: &str) -> Option<FileType> {
        Self::check_nestjs_by_filename(filename)
            .or_else(|| Self::check_nestjs_by_path(filename, path_str))
    }

    fn check_nestjs_by_filename(filename: &str) -> Option<FileType> {
        Self::check_nestjs_controllers(filename)
            .or_else(|| Self::check_nestjs_services(filename))
            .or_else(|| Self::check_nestjs_other(filename))
    }

    fn check_nestjs_controllers(filename: &str) -> Option<FileType> {
        if filename.ends_with(".controller.ts") || filename.ends_with(".controller.js") {
            Some(FileType::Controller)
        } else {
            None
        }
    }

    fn check_nestjs_services(filename: &str) -> Option<FileType> {
        if filename.ends_with(".service.ts") || filename.ends_with(".service.js") {
            Some(FileType::Service)
        } else if filename.ends_with(".module.ts") || filename.ends_with(".module.js") {
            Some(FileType::Module)
        } else {
            None
        }
    }

    fn check_nestjs_other(filename: &str) -> Option<FileType> {
        let patterns = [
            (".entity.ts", FileType::Entity),
            (".entity.js", FileType::Entity),
            (".repository.ts", FileType::Repository),
            (".repository.js", FileType::Repository),
            (".dto.ts", FileType::DTO),
            (".dto.js", FileType::DTO),
            (".guard.ts", FileType::Guard),
            (".guard.js", FileType::Guard),
            (".pipe.ts", FileType::Pipe),
            (".pipe.js", FileType::Pipe),
            (".middleware.ts", FileType::Middleware),
            (".middleware.js", FileType::Middleware),
            (".decorator.ts", FileType::Decorator),
            (".decorator.js", FileType::Decorator),
            (".interceptor.ts", FileType::Interceptor),
            (".interceptor.js", FileType::Interceptor),
        ];

        for (suffix, file_type) in patterns {
            if filename.ends_with(suffix) {
                return Some(file_type);
            }
        }
        None
    }

    fn check_nestjs_by_path(_filename: &str, path_str: &str) -> Option<FileType> {
        if path_str.contains("/entities/") {
            return Some(FileType::Entity);
        }
        if path_str.contains("/repositories/") {
            return Some(FileType::Repository);
        }
        if path_str.contains("/dto/") || path_str.contains("/dtos/") {
            return Some(FileType::DTO);
        }
        if path_str.contains("/guards/") {
            return Some(FileType::Guard);
        }
        if path_str.contains("/pipes/") {
            return Some(FileType::Pipe);
        }
        if path_str.contains("/middlewares/") {
            return Some(FileType::Middleware);
        }
        if path_str.contains("/decorators/") {
            return Some(FileType::Decorator);
        }
        if path_str.contains("/interceptors/") {
            return Some(FileType::Interceptor);
        }
        None
    }

    fn classify_nextjs(filename: &str, path_str: &str) -> Option<FileType> {
        if filename == "route.ts" || filename == "route.js" {
            return Some(FileType::ApiRoute);
        }
        if filename == "page.tsx" || filename == "page.js" || path_str.contains("/pages/") {
            if !path_str.contains("/pages/api/") {
                return Some(FileType::Page);
            } else {
                return Some(FileType::ApiRoute);
            }
        }
        None
    }

    fn classify_common(filename: &str, path_str: &str) -> Option<FileType> {
        if filename.ends_with(".interface.ts")
            || filename.ends_with(".interface.js")
            || path_str.contains("/interfaces/")
        {
            return Some(FileType::Interface);
        }
        if filename.ends_with(".types.ts")
            || filename.ends_with(".types.js")
            || path_str.contains("/types/")
        {
            return Some(FileType::Types);
        }
        if filename.ends_with(".config.ts")
            || filename.ends_with(".config.js")
            || filename.ends_with(".config.json")
            || path_str.contains("/config/")
        {
            return Some(FileType::Config);
        }
        if filename.ends_with(".event.ts") || path_str.contains("/events/") {
            return Some(FileType::Event);
        }
        if filename.ends_with(".error.ts")
            || filename.ends_with(".exception.ts")
            || path_str.contains("/errors/")
            || path_str.contains("/exceptions/")
        {
            return Some(FileType::Exception);
        }
        if filename.contains("migration") {
            return Some(FileType::Migration);
        }
        None
    }

    fn classify_cli(path_str: &str) -> Option<FileType> {
        if path_str.contains("/commands/") {
            return Some(FileType::CliCommand);
        }
        if path_str.contains("/hooks/") {
            return Some(FileType::CliHook);
        }
        None
    }

    fn classify_frontend(filename: &str) -> Option<FileType> {
        if filename.ends_with(".component.tsx")
            || filename.ends_with(".component.ts")
            || filename.ends_with(".component.js")
            || filename.ends_with(".component.jsx")
        {
            return Some(FileType::Component);
        }
        if filename.starts_with("use") && (filename.ends_with(".ts") || filename.ends_with(".js")) {
            return Some(FileType::Hook);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_classify_nestjs() {
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/user.controller.ts"), &[]),
            FileType::Controller
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/user.service.ts"), &[]),
            FileType::Service
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/user.dto.ts"), &[]),
            FileType::DTO
        );
    }

    #[test]
    fn test_classify_nextjs() {
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/pages/index.tsx"), &[]),
            FileType::Page
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/pages/api/user.ts"), &[]),
            FileType::ApiRoute
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/app/dashboard/page.tsx"), &[]),
            FileType::Page
        );
    }

    #[test]
    fn test_classify_priority() {
        // Test files should always be classified as Test
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/user.controller.test.ts"), &[]),
            FileType::Test
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/__tests__/utils.ts"), &[]),
            FileType::Test
        );
    }

    #[test]
    fn test_classify_more_nestjs() {
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/user.entity.ts"), &[]),
            FileType::Entity
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/user.repository.ts"), &[]),
            FileType::Repository
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/auth.guard.ts"), &[]),
            FileType::Guard
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/validation.pipe.ts"), &[]),
            FileType::Pipe
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/logger.middleware.ts"), &[]),
            FileType::Middleware
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/user.decorator.ts"), &[]),
            FileType::Decorator
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/transform.interceptor.ts"), &[]),
            FileType::Interceptor
        );
    }

    #[test]
    fn test_classify_common_patterns() {
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/types/user.ts"), &[]),
            FileType::Types
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/interfaces/api.ts"), &[]),
            FileType::Interface
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/config/database.json"), &[]),
            FileType::Config
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/events/user-created.event.ts"), &[]),
            FileType::Event
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/errors/not-found.error.ts"), &[]),
            FileType::Exception
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/db/migrations/001_migration.ts"), &[]),
            FileType::Migration
        );
    }

    #[test]
    fn test_classify_cli_and_frontend() {
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/commands/deploy.ts"), &[]),
            FileType::CliCommand
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/hooks/init.ts"), &[]),
            FileType::CliHook
        );
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/components/Button.component.tsx"), &[]),
            FileType::Component
        );
        // Note: src/hooks/useAuth.ts will be classified as CliHook because /hooks/ pattern comes first
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/frontend/useAuth.ts"), &[]),
            FileType::Hook
        );
    }

    #[test]
    fn test_classify_unknown() {
        assert_eq!(
            FileClassifier::classify(&PathBuf::from("src/random.txt"), &[]),
            FileType::Unknown
        );
    }
}

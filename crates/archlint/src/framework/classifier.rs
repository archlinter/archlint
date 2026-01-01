use super::{FileType, Framework};
use std::path::Path;

pub struct FileClassifier;

impl FileClassifier {
    pub fn classify(path: &Path, _frameworks: &[Framework]) -> FileType {
        let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
        let path_str = path.to_str().unwrap_or("");

        // 1. Test files (highest priority)
        if filename.ends_with(".test.ts")
            || filename.ends_with(".test.js")
            || filename.ends_with(".spec.ts")
            || filename.ends_with(".spec.js")
            || path_str.contains("/__tests__/")
            || path_str.contains("/tests/")
            || path_str.contains("/test/")
        {
            return FileType::Test;
        }

        // 2. NestJS patterns
        if filename.ends_with(".controller.ts") || filename.ends_with(".controller.js") {
            return FileType::Controller;
        }
        if filename.ends_with(".service.ts") || filename.ends_with(".service.js") {
            return FileType::Service;
        }
        if filename.ends_with(".module.ts") || filename.ends_with(".module.js") {
            return FileType::Module;
        }
        if filename.ends_with(".entity.ts")
            || filename.ends_with(".entity.js")
            || path_str.contains("/entities/")
        {
            return FileType::Entity;
        }
        if filename.ends_with(".repository.ts")
            || filename.ends_with(".repository.js")
            || path_str.contains("/repositories/")
        {
            return FileType::Repository;
        }
        if filename.ends_with(".dto.ts")
            || filename.ends_with(".dto.js")
            || path_str.contains("/dto/")
            || path_str.contains("/dtos/")
        {
            return FileType::DTO;
        }
        if filename.ends_with(".guard.ts")
            || filename.ends_with(".guard.js")
            || path_str.contains("/guards/")
        {
            return FileType::Guard;
        }
        if filename.ends_with(".pipe.ts")
            || filename.ends_with(".pipe.js")
            || path_str.contains("/pipes/")
        {
            return FileType::Pipe;
        }
        if filename.ends_with(".middleware.ts")
            || filename.ends_with(".middleware.js")
            || path_str.contains("/middlewares/")
        {
            return FileType::Middleware;
        }
        if filename.ends_with(".decorator.ts")
            || filename.ends_with(".decorator.js")
            || path_str.contains("/decorators/")
        {
            return FileType::Decorator;
        }
        if filename.ends_with(".interceptor.ts")
            || filename.ends_with(".interceptor.js")
            || path_str.contains("/interceptors/")
        {
            return FileType::Interceptor;
        }

        // 3. Next.js patterns
        if filename == "page.tsx" || filename == "page.js" || path_str.contains("/pages/") {
            if !path_str.contains("/pages/api/") {
                return FileType::Page;
            } else {
                return FileType::ApiRoute;
            }
        }
        if filename == "route.ts" || filename == "route.js" {
            return FileType::ApiRoute;
        }

        // 4. Common patterns
        if filename.ends_with(".interface.ts")
            || filename.ends_with(".interface.js")
            || path_str.contains("/interfaces/")
        {
            return FileType::Interface;
        }
        if filename.ends_with(".types.ts")
            || filename.ends_with(".types.js")
            || path_str.contains("/types/")
        {
            return FileType::Types;
        }
        if filename.ends_with(".config.ts")
            || filename.ends_with(".config.js")
            || filename.ends_with(".config.json")
            || path_str.contains("/config/")
        {
            return FileType::Config;
        }
        if filename.ends_with(".event.ts") || path_str.contains("/events/") {
            return FileType::Event;
        }
        if filename.ends_with(".error.ts")
            || filename.ends_with(".exception.ts")
            || path_str.contains("/errors/")
            || path_str.contains("/exceptions/")
        {
            return FileType::Exception;
        }
        if filename.contains("migration") {
            return FileType::Migration;
        }

        // 6. CLI patterns (oclif)
        if path_str.contains("/commands/") {
            return FileType::CliCommand;
        }
        if path_str.contains("/hooks/") {
            return FileType::CliHook;
        }

        // 5. Frontend patterns
        if filename.ends_with(".component.tsx")
            || filename.ends_with(".component.ts")
            || filename.ends_with(".component.js")
            || filename.ends_with(".component.jsx")
        {
            return FileType::Component;
        }
        if filename.starts_with("use") && (filename.ends_with(".ts") || filename.ends_with(".js")) {
            return FileType::Hook;
        }

        FileType::Unknown
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

use super::Framework;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use ignore::WalkBuilder;

pub struct FrameworkDetector;

impl FrameworkDetector {
    pub fn detect<P: AsRef<Path>>(root: P) -> Vec<Framework> {
        let root = root.as_ref();
        let mut frameworks = HashSet::new();

        let walker = WalkBuilder::new(root)
            .standard_filters(true)
            .hidden(false)
            .build();

        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if path.file_name().is_some_and(|n| n == "package.json") {
                if let Ok(content) = fs::read_to_string(path) {
                        if let Ok(json) = serde_json::from_str::<Value>(&content) {
                            Self::detect_from_json(&json, &mut frameworks);
                        }
                }
            }
        }

        frameworks.into_iter().collect()
    }

    fn detect_from_json(json: &Value, frameworks: &mut HashSet<Framework>) {
        let dependencies = [
            "dependencies",
            "devDependencies",
            "peerDependencies",
        ];

        for dep_type in dependencies {
            if let Some(deps) = json.get(dep_type).and_then(|d| d.as_object()) {
                for dep_name in deps.keys() {
                    match dep_name.as_str() {
                        "@nestjs/core" | "@nestjs/common" => {
                            frameworks.insert(Framework::NestJS);
                        }
                        "next" => {
                            frameworks.insert(Framework::NextJS);
                        }
                        "express" => {
                            frameworks.insert(Framework::Express);
                        }
                        "react" => {
                            frameworks.insert(Framework::React);
                        }
                        "@angular/core" => {
                            frameworks.insert(Framework::Angular);
                        }
                        "vue" => {
                            frameworks.insert(Framework::Vue);
                        }
                        "typeorm" => {
                            frameworks.insert(Framework::TypeORM);
                        }
                        "@prisma/client" => {
                            frameworks.insert(Framework::Prisma);
                        }
                        "@oclif/core" | "@oclif/command" => {
                            frameworks.insert(Framework::Oclif);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_detect_nestjs() {
        let json = json!({
            "dependencies": {
                "@nestjs/core": "^10.0.0"
            }
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.contains(&Framework::NestJS));
    }

    #[test]
    fn test_detect_multiple() {
        let json = json!({
            "dependencies": {
                "next": "latest",
                "typeorm": "latest"
            }
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.contains(&Framework::NextJS));
        assert!(frameworks.contains(&Framework::TypeORM));
    }

    #[test]
    fn test_detect_dev_deps() {
        let json = json!({
            "devDependencies": {
                "react": "latest"
            }
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.contains(&Framework::React));
    }

    #[test]
    fn test_detect_all_frameworks() {
        let cases = [
            ("express", Framework::Express),
            ("@angular/core", Framework::Angular),
            ("vue", Framework::Vue),
            ("@prisma/client", Framework::Prisma),
            ("@oclif/core", Framework::Oclif),
        ];

        for (dep, expected) in cases {
            let json = json!({ "dependencies": { dep: "latest" } });
            let mut frameworks = HashSet::new();
            FrameworkDetector::detect_from_json(&json, &mut frameworks);
            assert!(frameworks.contains(&expected), "Failed to detect {}", dep);
        }
    }

    #[test]
    fn test_detect_peer_deps() {
        let json = json!({
            "peerDependencies": {
                "@nestjs/common": "latest"
            }
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.contains(&Framework::NestJS));
    }

    #[test]
    fn test_detect_no_deps() {
        let json = json!({
            "name": "no-deps"
        });
        let mut frameworks = HashSet::new();
        FrameworkDetector::detect_from_json(&json, &mut frameworks);
        assert!(frameworks.is_empty());
    }
}

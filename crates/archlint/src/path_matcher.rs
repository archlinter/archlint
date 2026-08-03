//! Compiled matching for user-supplied path patterns.
//!
//! Patterns come from the config (`ignore:`) and are written by hand, so they are
//! normalized before compilation: a plain path such as `src/legacy` means "this
//! entry and everything below it", while anything containing glob syntax is
//! compiled verbatim so existing configs keep their exact meaning.
//!
//! Matching is done against the path relative to the project root.

use crate::config::Config;
use log::warn;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

/// Expands one user-written pattern into the globs it stands for.
///
/// Returns an empty vector for patterns that cannot be used (empty, or escaping
/// the project root via `..`).
#[must_use]
pub fn expand_pattern(raw: &str) -> Vec<String> {
    let normalized = raw.replace('\\', "/");
    let is_directory = normalized.ends_with('/');
    let path = normalized.trim_matches('/').trim_start_matches("./");

    if path.is_empty() || path.split('/').any(|part| part == "..") {
        return Vec::new();
    }

    if path.contains(['*', '?', '[']) {
        // Already a glob: keep the user's semantics untouched.
        return if is_directory {
            vec![format!("{path}/**")]
        } else {
            vec![path.to_string()]
        };
    }

    // A plain path matches the entry itself and everything below it, at any depth.
    vec![format!("**/{path}/**"), format!("**/{path}")]
}

/// A pre-compiled set of path patterns, matched relative to a project root.
#[derive(Debug, Clone, Default)]
pub struct PathMatcher {
    root: PathBuf,
    patterns: Vec<glob::Pattern>,
}

impl PathMatcher {
    /// Compiles `raw_patterns`, warning about (and skipping) unusable ones.
    #[must_use]
    pub fn new(root: &Path, raw_patterns: &[String]) -> Self {
        let mut patterns = Vec::new();

        for raw in raw_patterns {
            let expanded = expand_pattern(raw);
            if expanded.is_empty() {
                warn!("Unusable ignore pattern `{raw}`: it matches nothing and was skipped");
                continue;
            }
            for glob in expanded {
                match glob::Pattern::new(&glob) {
                    Ok(pattern) => patterns.push(pattern),
                    Err(e) => warn!("Invalid ignore pattern `{raw}`: {e}"),
                }
            }
        }

        Self {
            root: root.to_path_buf(),
            patterns,
        }
    }

    /// Compiles the global `ignore:` list of `config`.
    #[must_use]
    pub fn from_config(config: &Config, root: &Path) -> Self {
        Self::new(root, &config.ignore)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// Returns `true` if `path` is matched by any pattern.
    #[must_use]
    pub fn matches(&self, path: &Path) -> bool {
        if self.patterns.is_empty() {
            return false;
        }

        let relative = path.strip_prefix(&self.root).unwrap_or(path);
        let relative = relative.to_string_lossy();
        let relative = if relative.contains('\\') {
            Cow::Owned(relative.replace('\\', "/"))
        } else {
            relative
        };

        self.patterns.iter().any(|p| p.matches(&relative))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matcher(patterns: &[&str]) -> PathMatcher {
        let owned: Vec<String> = patterns.iter().map(|p| (*p).to_string()).collect();
        PathMatcher::new(Path::new("/proj"), &owned)
    }

    #[test]
    fn plain_directory_matches_its_subtree() {
        assert_eq!(
            expand_pattern("legacy"),
            vec!["**/legacy/**".to_string(), "**/legacy".to_string()]
        );

        let m = matcher(&["legacy"]);
        assert!(m.matches(Path::new("/proj/legacy/a.ts")));
        assert!(m.matches(Path::new("/proj/packages/x/legacy/deep/a.ts")));
        assert!(!m.matches(Path::new("/proj/src/a.ts")));
    }

    #[test]
    fn plain_nested_path_matches_its_subtree() {
        let m = matcher(&["src/legacy"]);
        assert!(m.matches(Path::new("/proj/src/legacy/a.ts")));
        assert!(!m.matches(Path::new("/proj/src/a.ts")));
    }

    #[test]
    fn plain_file_name_matches_the_file() {
        let m = matcher(&["generated.ts"]);
        assert!(m.matches(Path::new("/proj/src/generated.ts")));
        assert!(m.matches(Path::new("/proj/generated.ts")));
        assert!(!m.matches(Path::new("/proj/src/a.ts")));
    }

    #[test]
    fn leading_and_trailing_separators_are_ignored() {
        assert_eq!(expand_pattern("./legacy"), expand_pattern("legacy"));
        assert_eq!(expand_pattern("/legacy"), expand_pattern("legacy"));
        assert_eq!(expand_pattern("legacy/"), expand_pattern("legacy"));
    }

    #[test]
    fn globs_are_compiled_verbatim() {
        assert_eq!(expand_pattern("**/dist/**"), vec!["**/dist/**".to_string()]);
        assert_eq!(expand_pattern("src/*.ts"), vec!["src/*.ts".to_string()]);
        assert_eq!(expand_pattern("**/*.d.ts"), vec!["**/*.d.ts".to_string()]);

        let m = matcher(&["**/dist/**", "**/*.spec.ts"]);
        assert!(m.matches(Path::new("/proj/dist/bundle.js")));
        assert!(m.matches(Path::new("/proj/src/dist/bundle.js")));
        assert!(m.matches(Path::new("/proj/src/a.spec.ts")));
        assert!(!m.matches(Path::new("/proj/src/a.ts")));
    }

    #[test]
    fn a_glob_with_a_trailing_separator_still_matches_the_subtree() {
        let m = matcher(&["**/dist/"]);
        assert!(m.matches(Path::new("/proj/dist/bundle.js")));
    }

    #[test]
    fn backslashes_are_normalized() {
        assert_eq!(expand_pattern("src\\legacy"), expand_pattern("src/legacy"));
    }

    #[test]
    fn unusable_patterns_are_skipped_without_disabling_the_rest() {
        let m = matcher(&["", "../outside", "src/**legacy", "legacy"]);
        assert!(m.matches(Path::new("/proj/legacy/a.ts")));
        assert!(!m.matches(Path::new("/proj/src/a.ts")));
    }

    #[test]
    fn an_empty_matcher_never_matches() {
        let m = matcher(&[]);
        assert!(m.is_empty());
        assert!(!m.matches(Path::new("/proj/src/a.ts")));
    }

    #[test]
    fn paths_outside_the_root_are_matched_as_is() {
        let m = matcher(&["**/legacy/**"]);
        assert!(m.matches(Path::new("/elsewhere/legacy/a.ts")));
    }
}

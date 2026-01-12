use crate::parser::types::{
    FileSymbols, FunctionComplexity, IgnoredRulesMap, ParsedFile, ParserConfig, SymbolSet,
};
use crate::parser::visitor::UnifiedVisitor;
use crate::Result;
use oxc_allocator::Allocator;
use oxc_ast::visit::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
use rustc_hash::FxHashMap;
use std::fs;
use std::path::Path;

pub struct ImportParser;

impl ImportParser {
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    #[inline]
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<ParsedFile> {
        self.parse_file_with_config(path, &ParserConfig::all())
    }

    pub fn parse_file_with_config<P: AsRef<Path>>(
        &self,
        path: P,
        config: &ParserConfig,
    ) -> Result<ParsedFile> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        self.parse_code_with_config(&content, path, config)
    }

    #[inline]
    pub fn parse_code<P: AsRef<Path>>(&self, content: &str, path: P) -> Result<ParsedFile> {
        self.parse_code_with_config(content, path, &ParserConfig::all())
    }

    pub fn parse_code_with_config<P: AsRef<Path>>(
        &self,
        content: &str,
        path: P,
        config: &ParserConfig,
    ) -> Result<ParsedFile> {
        let path = path.as_ref();
        let allocator = Allocator::default();
        let mut source_type = SourceType::from_path(path).unwrap_or_default();

        if path
            .extension()
            .is_some_and(|ext| ext == "ts" || ext == "tsx" || ext == "mts" || ext == "cts")
        {
            source_type = source_type.with_typescript(true);
        }

        let ret = Parser::new(&allocator, content, source_type).parse();

        let mut visitor = UnifiedVisitor::new(content, *config);
        visitor.visit_program(&ret.program);

        let export_names: SymbolSet = visitor.exports.iter().map(|e| e.name.clone()).collect();
        visitor
            .local_definitions
            .retain(|d| !export_names.contains(d));

        let lines = visitor.line_count();
        let ignored_lines = self.parse_ignore_comments(content, &ret.program.comments, &visitor);

        Ok(ParsedFile {
            symbols: FileSymbols {
                exports: visitor.exports,
                imports: visitor.imports,
                classes: visitor.classes,
                local_definitions: visitor.local_definitions,
                local_usages: visitor.local_usages,
                has_runtime_code: visitor.has_runtime_code,
                env_vars: visitor.env_vars,
            },
            functions: visitor.functions,
            lines,
            ignored_lines,
        })
    }

    fn parse_ignore_comments(
        &self,
        content: &str,
        comments: &oxc_allocator::Vec<'_, oxc_ast::ast::Comment>,
        visitor: &UnifiedVisitor,
    ) -> IgnoredRulesMap {
        let mut ignored = IgnoredRulesMap::default();
        let mut active_blocks: FxHashMap<String, usize> = FxHashMap::default();
        let total_lines = visitor.line_count();

        for comment in comments {
            let start = comment.span.start;
            let comment_text = &content[start as usize..comment.span.end as usize];
            let comment_line = visitor.get_line_number_from_offset(start as usize);

            // Support multiline comments by checking each line
            if comment_text.contains('\n') {
                for (offset, line_text) in comment_text.lines().enumerate() {
                    if let Some((command, rules)) = self.parse_ignore_command(line_text) {
                        self.process_ignore_command(
                            command,
                            rules,
                            comment_line + offset,
                            &mut active_blocks,
                            &mut ignored,
                        );
                    }
                }
            } else if let Some((command, rules)) = self.parse_ignore_command(comment_text) {
                self.process_ignore_command(
                    command,
                    rules,
                    comment_line,
                    &mut active_blocks,
                    &mut ignored,
                );
            }
        }

        self.close_remaining_blocks(active_blocks, total_lines, &mut ignored);
        ignored
    }

    fn process_ignore_command(
        &self,
        command: String,
        rules: SymbolSet,
        line: usize,
        active_blocks: &mut FxHashMap<String, usize>,
        ignored: &mut IgnoredRulesMap,
    ) {
        match command.as_str() {
            "archlint-disable" => self.handle_disable(rules, line, active_blocks),
            "archlint-enable" => self.handle_enable(rules, line, active_blocks, ignored),
            "archlint-disable-line" => {
                ignored
                    .entry(line)
                    .or_default()
                    .extend(rules.into_iter().map(|r| r.to_string()));
            }
            "archlint-disable-next-line" => {
                if let Some(next_line) = line.checked_add(1) {
                    ignored
                        .entry(next_line)
                        .or_default()
                        .extend(rules.into_iter().map(|r| r.to_string()));
                }
            }
            _ => {}
        }
    }

    fn handle_disable(
        &self,
        rules: SymbolSet,
        line: usize,
        active_blocks: &mut FxHashMap<String, usize>,
    ) {
        for rule in rules {
            active_blocks.insert(rule.to_string(), line);
        }
    }

    fn handle_enable(
        &self,
        rules: SymbolSet,
        line: usize,
        active_blocks: &mut FxHashMap<String, usize>,
        ignored: &mut IgnoredRulesMap,
    ) {
        let rules_to_close: Vec<String> = if rules.contains("*") {
            active_blocks.keys().cloned().collect()
        } else {
            rules.into_iter().map(|r| r.to_string()).collect()
        };

        for rule in rules_to_close {
            if let Some(start_line) = active_blocks.remove(&rule) {
                self.mark_range_ignored(ignored, start_line, line, &rule);
            }
        }
    }

    fn close_remaining_blocks(
        &self,
        active_blocks: FxHashMap<String, usize>,
        total_lines: usize,
        ignored: &mut IgnoredRulesMap,
    ) {
        for (rule, start_line) in active_blocks {
            if rule == "*" && start_line <= 1 {
                ignored.entry(0).or_default().insert("*".to_string());
            }
            self.mark_range_ignored(ignored, start_line, total_lines, &rule);
        }
    }

    fn mark_range_ignored(
        &self,
        ignored: &mut IgnoredRulesMap,
        start: usize,
        end: usize,
        rule: &str,
    ) {
        for l in start..=end {
            ignored.entry(l).or_default().insert(rule.to_string());
        }
    }

    fn parse_ignore_command(&self, text: &str) -> Option<(String, SymbolSet)> {
        let text = text.trim();
        let text = if let Some(stripped) = text.strip_prefix("//") {
            stripped
        } else if let Some(stripped) = text.strip_prefix("/*") {
            stripped.strip_suffix("*/").unwrap_or(stripped)
        } else {
            text
        };

        let text = text.trim().trim_start_matches(['/', '*']).trim_start();

        let is_disable = text.starts_with("archlint-disable");
        let is_enable = text.starts_with("archlint-enable");

        if !is_disable && !is_enable {
            return None;
        }

        let parts: Vec<&str> = text.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_string();
        let mut rules = SymbolSet::default();

        if parts.len() > 1 {
            let rules_part = parts[1..].join(" ");
            for rule_chunk in rules_part.split(',') {
                let rule = rule_chunk.split_whitespace().next().unwrap_or("").trim();
                if !rule.is_empty() {
                    rules.insert(rule.into());
                }
            }
        }

        if rules.is_empty() {
            rules.insert("*".into());
        }

        Some((command, rules))
    }

    #[inline]
    pub fn parse_file_symbols<P: AsRef<Path>>(&self, path: P) -> Result<FileSymbols> {
        Ok(self.parse_file(path)?.symbols)
    }

    #[inline]
    pub fn parse_complexity<P: AsRef<Path>>(&self, path: P) -> Result<Vec<FunctionComplexity>> {
        Ok(self.parse_file(path)?.functions)
    }

    #[inline]
    pub fn is_relative_import(import: &str) -> bool {
        import.starts_with("./") || import.starts_with("../")
    }

    #[inline]
    pub fn is_alias_import(import: &str) -> bool {
        import.starts_with('@') || import.starts_with('~')
    }
}

impl Default for ImportParser {
    fn default() -> Self {
        Self::new().expect("Failed to create ImportParser")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_is_relative_import() {
        assert!(ImportParser::is_relative_import("./foo"));
        assert!(ImportParser::is_relative_import("../foo"));
        assert!(!ImportParser::is_relative_import("foo"));
        assert!(!ImportParser::is_relative_import("@foo"));
    }

    #[test]
    fn test_is_alias_import() {
        assert!(ImportParser::is_alias_import("@foo/bar"));
        assert!(ImportParser::is_alias_import("~/foo"));
        assert!(!ImportParser::is_alias_import("./foo"));
    }

    #[test]
    fn test_parse_file_with_temp() {
        let mut tmp = NamedTempFile::new().unwrap();
        // Use write! instead of writeln! to avoid extra newline
        write!(tmp, "import {{ a }} from './b'; export const x = 1;").unwrap();

        let parser = ImportParser::new().unwrap();
        let result = parser.parse_file(tmp.path()).unwrap();

        assert_eq!(result.symbols.imports.len(), 1);
        assert_eq!(result.symbols.exports.len(), 1);
        assert_eq!(result.lines, 1);
    }

    #[test]
    fn test_parse_code_variants() {
        let parser = ImportParser::new().unwrap();

        // TSX with export
        let tsx = "export const App = () => <div>Hello</div>;";
        let res_tsx = parser.parse_code(tsx, "app.tsx").unwrap();
        assert!(res_tsx.symbols.has_runtime_code);

        // JS with export
        let js = "export const x = 1;";
        let res_js = parser.parse_code(js, "script.js").unwrap();
        assert!(res_js.symbols.has_runtime_code);
    }
}

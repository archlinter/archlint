use crate::parser::types::{FileSymbols, FunctionComplexity, ParsedFile, ParserConfig, SymbolSet};
use crate::parser::visitor::UnifiedVisitor;
use crate::Result;
use oxc_allocator::Allocator;
use oxc_ast::visit::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
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
            .is_some_and(|ext| ext == "ts" || ext == "tsx")
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
        })
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

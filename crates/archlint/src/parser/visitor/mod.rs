use crate::detectors::CodeRange;
use crate::parser::line_index::LineIndex;
use crate::parser::types::{
    ClassSymbol, ExportedSymbol, FunctionComplexity, ImportedSymbol, MethodSymbol, ParserConfig,
    SymbolName, SymbolSet,
};
use compact_str::CompactString;
use oxc_ast::visit::Visit;
use oxc_syntax::scope::ScopeFlags;
use smallvec::SmallVec;

pub mod exports;
pub mod imports;
pub mod locals;
pub mod metrics;

/// Static interned strings for common symbols to avoid repeated allocations
pub(crate) mod interned {
    use compact_str::CompactString;
    use std::sync::LazyLock;

    pub static STAR: LazyLock<CompactString> = LazyLock::new(|| CompactString::const_new("*"));
    pub static DEFAULT: LazyLock<CompactString> =
        LazyLock::new(|| CompactString::const_new("default"));
    pub static ANONYMOUS: LazyLock<CompactString> =
        LazyLock::new(|| CompactString::const_new("<anonymous>"));
    pub static CONSTRUCTOR: LazyLock<CompactString> =
        LazyLock::new(|| CompactString::const_new("constructor"));
    pub static CONSTRUCTOR_SUFFIX: LazyLock<CompactString> =
        LazyLock::new(|| CompactString::const_new(".constructor"));
}

pub struct UnifiedVisitor {
    pub exports: Vec<ExportedSymbol>,
    pub imports: Vec<ImportedSymbol>,
    pub classes: Vec<ClassSymbol>,
    pub local_definitions: Vec<SymbolName>,
    pub local_usages: SymbolSet,
    pub has_runtime_code: bool,
    pub functions: Vec<FunctionComplexity>,
    pub config: ParserConfig,
    pub(crate) current_name_override: Option<SymbolName>,
    pub(crate) current_span_override: Option<oxc_span::Span>,
    pub(crate) current_class: Option<SymbolName>,

    pub(crate) temp_fields: SymbolSet,
    pub(crate) temp_methods: SmallVec<[MethodSymbol; 8]>,
    pub(crate) current_method: Option<MethodSymbol>,
    pub(crate) current_top_level_export: Option<usize>,
    pub env_vars: SymbolSet,

    /// Pre-computed line index for O(log n) line/column lookup
    pub(crate) line_index: LineIndex,
}

impl UnifiedVisitor {
    /// Create a new visitor for the given source text.
    pub fn new(source_text: &str, config: ParserConfig) -> Self {
        // Pre-allocate based on file size heuristics to minimize re-allocations.
        // Heuristics:
        // - 1 import per ~500 bytes
        // - 1 export per ~1000 bytes
        // - 1 function per ~200 bytes
        let estimated_imports = (source_text.len() / 500).max(8);
        let estimated_exports = (source_text.len() / 1000).max(4);
        let estimated_functions = (source_text.len() / 200).max(8);

        Self {
            exports: Vec::with_capacity(estimated_exports),
            imports: Vec::with_capacity(estimated_imports),
            classes: Vec::new(),
            local_definitions: Vec::new(),
            local_usages: SymbolSet::default(),
            has_runtime_code: false,
            functions: Vec::with_capacity(estimated_functions),
            config,
            current_name_override: None,
            current_span_override: None,
            current_class: None,
            temp_fields: SymbolSet::default(),
            temp_methods: SmallVec::new(),
            current_method: None,
            current_top_level_export: None,
            env_vars: SymbolSet::default(),
            line_index: LineIndex::new(source_text),
        }
    }

    #[inline]
    pub(crate) fn get_range(&self, span: oxc_span::Span) -> CodeRange {
        let (start_line, start_column) = self.line_index.line_col(span.start as usize);
        let (end_line, end_column) = self.line_index.line_col(span.end as usize);
        CodeRange {
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }

    #[inline]
    pub(crate) fn get_line_number(&self, span: oxc_span::Span) -> usize {
        self.line_index.line(span.start as usize)
    }

    #[inline]
    pub fn get_line_number_from_offset(&self, offset: usize) -> usize {
        self.line_index.line(offset)
    }

    #[inline]
    pub fn line_count(&self) -> usize {
        self.line_index.line_count()
    }

    /// Convert oxc Atom to CompactString efficiently
    #[inline]
    pub(crate) fn atom_to_compact(atom: &oxc_span::Atom) -> CompactString {
        CompactString::new(atom.as_str())
    }

    /// Convert oxc ModuleExportName to CompactString
    #[inline]
    pub(crate) fn export_name_to_compact(name: oxc_span::Atom) -> CompactString {
        CompactString::new(name.as_str())
    }

    pub(crate) fn ts_type_name_to_string(it: &oxc_ast::ast::TSTypeName<'_>) -> String {
        match it {
            oxc_ast::ast::TSTypeName::IdentifierReference(id) => id.name.to_string(),
            oxc_ast::ast::TSTypeName::QualifiedName(qn) => {
                format!(
                    "{}.{}",
                    Self::ts_type_name_to_string(&qn.left),
                    qn.right.name
                )
            }
        }
    }
}

impl<'a> Visit<'a> for UnifiedVisitor {
    fn visit_import_declaration(&mut self, it: &oxc_ast::ast::ImportDeclaration<'a>) {
        self.handle_import_declaration(it);
    }

    fn visit_export_named_declaration(&mut self, it: &oxc_ast::ast::ExportNamedDeclaration<'a>) {
        self.handle_export_named_declaration(it);
    }

    fn visit_export_all_declaration(&mut self, it: &oxc_ast::ast::ExportAllDeclaration<'a>) {
        self.handle_export_all_declaration(it);
    }

    fn visit_export_default_declaration(
        &mut self,
        it: &oxc_ast::ast::ExportDefaultDeclaration<'a>,
    ) {
        self.handle_export_default_declaration(it);
    }

    fn visit_identifier_reference(&mut self, it: &oxc_ast::ast::IdentifierReference<'a>) {
        self.handle_identifier_reference(it);
    }

    fn visit_jsx_identifier(&mut self, it: &oxc_ast::ast::JSXIdentifier<'a>) {
        self.handle_jsx_identifier(it);
    }

    fn visit_member_expression(&mut self, it: &oxc_ast::ast::MemberExpression<'a>) {
        self.handle_member_expression(it);
        oxc_ast::visit::walk::walk_member_expression(self, it);
    }

    fn visit_private_field_expression(&mut self, it: &oxc_ast::ast::PrivateFieldExpression<'a>) {
        self.handle_private_field(it);
        oxc_ast::visit::walk::walk_private_field_expression(self, it);
    }

    fn visit_ts_type_name(&mut self, it: &oxc_ast::ast::TSTypeName<'a>) {
        self.handle_ts_type_name(it);
    }

    fn visit_expression(&mut self, expr: &oxc_ast::ast::Expression<'a>) {
        self.handle_expression(expr);
    }

    fn visit_class(&mut self, it: &oxc_ast::ast::Class<'a>) {
        self.handle_class(it);
    }

    fn visit_variable_declarator(&mut self, it: &oxc_ast::ast::VariableDeclarator<'a>) {
        self.handle_variable_declarator(it);
    }

    fn visit_method_definition(&mut self, it: &oxc_ast::ast::MethodDefinition<'a>) {
        self.handle_method_definition(it);
    }

    fn visit_function(&mut self, it: &oxc_ast::ast::Function<'a>, flags: ScopeFlags) {
        self.handle_function(it, flags);
    }

    fn visit_arrow_function_expression(&mut self, it: &oxc_ast::ast::ArrowFunctionExpression<'a>) {
        self.handle_arrow_function_expression(it);
    }
}

#[cfg(test)]
mod tests;

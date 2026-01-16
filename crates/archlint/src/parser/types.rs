use crate::detectors::CodeRange;
use compact_str::CompactString;
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashSet;

/// Compact string type for symbol names (inline up to 24 bytes, no heap allocation for short strings)
pub type SymbolName = CompactString;

/// Fast hash set optimized for string keys
pub type SymbolSet = FxHashSet<SymbolName>;

/// Map of line number to set of ignored rule IDs
pub type IgnoredRulesMap = FxHashMap<usize, FxHashSet<String>>;

/// Map of file path to its ignored lines and rules
pub type FileIgnoredLines = FxHashMap<std::path::PathBuf, IgnoredRulesMap>;

/// The kind of a code symbol (e.g., function, class, type).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbolKind {
    Function,
    Class,
    Variable,
    Type,
    Interface,
    Enum,
    Unknown,
}

/// Information about a symbol exported from a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedSymbol {
    /// Name of the exported symbol.
    pub name: SymbolName,
    /// Kind of the symbol.
    pub kind: SymbolKind,
    /// Whether this is a re-export from another module.
    pub is_reexport: bool,
    /// Original source module for re-exports.
    pub source: Option<SymbolName>,
    /// Line number of the export (1-based).
    pub line: usize,
    /// Column number of the export (1-based).
    pub column: usize,
    /// Exact code range of the export.
    pub range: CodeRange,
    /// Set of other symbols used within this exported symbol.
    pub used_symbols: SymbolSet,
    /// Whether the exported variable is mutable (e.g., `let` vs `const`).
    pub is_mutable: bool,
    /// Whether this is a default export.
    pub is_default: bool,
}

/// Information about a symbol imported into a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedSymbol {
    /// Original name of the symbol in the source module.
    pub name: SymbolName,
    /// Alias used for the import (e.g., `import { name as alias }`).
    pub alias: Option<SymbolName>,
    /// Resolved source module or file path.
    pub source: SymbolName,
    /// Line number of the import (1-based).
    pub line: usize,
    /// Column number of the import (1-based).
    pub column: usize,
    /// Exact code range of the import statement.
    pub range: CodeRange,
    /// Whether this is a type-only import.
    pub is_type_only: bool,
    /// Whether this import is being re-exported.
    pub is_reexport: bool,
    /// Whether this is a dynamic import (e.g., `import()` or `require()`).
    pub is_dynamic: bool,
}

/// Accessibility level for class methods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MethodAccessibility {
    Public,
    Protected,
    Private,
}

/// Detailed information about a class method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodSymbol {
    /// Name of the method.
    pub name: SymbolName,
    /// Set of class fields used by this method.
    pub used_fields: SymbolSet,
    /// Set of other class methods called by this method.
    pub used_methods: SymbolSet,
    /// Line number of the method definition.
    pub line: usize,
    /// Column number of the method definition.
    pub column: usize,
    /// Exact code range of the method.
    pub range: CodeRange,
    /// Whether the method has any decorators.
    pub has_decorators: bool,
    /// Whether this is a getter or setter.
    pub is_accessor: bool,
    /// Optional accessibility level.
    pub accessibility: Option<MethodAccessibility>,
    /// Whether the method is abstract.
    pub is_abstract: bool,
}

/// Builder for creating MethodSymbol instances.
pub struct MethodSymbolBuilder {
    name: SymbolName,
    line: usize,
    column: usize,
    range: CodeRange,
    has_decorators: bool,
    is_accessor: bool,
    accessibility: Option<MethodAccessibility>,
    is_abstract: bool,
}

impl MethodSymbolBuilder {
    pub fn new(name: impl Into<SymbolName>, line: usize, column: usize, range: CodeRange) -> Self {
        Self {
            name: name.into(),
            line,
            column,
            range,
            has_decorators: false,
            is_accessor: false,
            accessibility: None,
            is_abstract: false,
        }
    }

    pub fn has_decorators(mut self, value: bool) -> Self {
        self.has_decorators = value;
        self
    }

    pub fn is_accessor(mut self, value: bool) -> Self {
        self.is_accessor = value;
        self
    }

    pub fn accessibility(mut self, value: Option<MethodAccessibility>) -> Self {
        self.accessibility = value;
        self
    }

    pub fn is_abstract(mut self, value: bool) -> Self {
        self.is_abstract = value;
        self
    }

    pub fn build(self) -> MethodSymbol {
        MethodSymbol {
            name: self.name,
            used_fields: SymbolSet::default(),
            used_methods: SymbolSet::default(),
            line: self.line,
            column: self.column,
            range: self.range,
            has_decorators: self.has_decorators,
            is_accessor: self.is_accessor,
            accessibility: self.accessibility,
            is_abstract: self.is_abstract,
        }
    }
}

impl MethodSymbol {
    /// Create a new method symbol with the given metadata.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: impl Into<SymbolName>,
        line: usize,
        column: usize,
        range: CodeRange,
        has_decorators: bool,
        is_accessor: bool,
        accessibility: Option<MethodAccessibility>,
        is_abstract: bool,
    ) -> Self {
        Self {
            name: name.into(),
            used_fields: SymbolSet::default(),
            used_methods: SymbolSet::default(),
            line,
            column,
            range,
            has_decorators,
            is_accessor,
            accessibility,
            is_abstract,
        }
    }
}

/// Information about a class definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSymbol {
    /// Name of the class.
    pub name: SymbolName,
    /// Name of the super class (if any).
    pub super_class: Option<SymbolName>,
    /// List of interfaces implemented by the class.
    pub implements: Vec<SymbolName>,
    /// List of field names defined in the class.
    pub fields: SmallVec<[SymbolName; 8]>,
    /// List of method symbols defined in the class.
    pub methods: SmallVec<[MethodSymbol; 8]>,
    /// Whether the class is abstract.
    pub is_abstract: bool,
}

impl ClassSymbol {
    /// Create a new class symbol with the given name.
    #[inline]
    pub fn new(name: impl Into<SymbolName>) -> Self {
        Self {
            name: name.into(),
            super_class: None,
            implements: Vec::new(),
            fields: SmallVec::new(),
            methods: SmallVec::new(),
            is_abstract: false,
        }
    }
}

/// Collection of all symbols extracted from a single source file.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileSymbols {
    /// Symbols exported from the file.
    pub exports: Vec<ExportedSymbol>,
    /// Symbols imported into the file.
    pub imports: Vec<ImportedSymbol>,
    /// Class definitions found in the file.
    pub classes: Vec<ClassSymbol>,
    /// Names of all symbols defined locally in the file.
    pub local_definitions: Vec<SymbolName>,
    /// Set of all symbol names used in the file.
    pub local_usages: SymbolSet,
    /// Whether the file contains any executable runtime code.
    pub has_runtime_code: bool,
    /// Environment variables accessed in the file.
    pub env_vars: SymbolSet,
}

/// Quantitative details about a function's complexity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    /// Name of the function.
    pub name: SymbolName,
    /// Line number of the function definition.
    pub line: usize,
    /// Exact code range of the function.
    pub range: CodeRange,
    /// Cyclomatic complexity score.
    pub cyclomatic_complexity: usize,
    /// Cognitive complexity score.
    pub cognitive_complexity: usize,
    /// Maximum nesting depth.
    pub max_depth: usize,
    /// Total number of parameters.
    pub param_count: usize,
    /// Number of parameters with primitive types.
    pub primitive_params: usize,
    /// Whether the function is a constructor.
    pub is_constructor: bool,
}

/// Full results of parsing a single file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedFile {
    /// Symbols extracted from the file.
    pub symbols: FileSymbols,
    /// Complexity details for all functions in the file.
    pub functions: Vec<FunctionComplexity>,
    /// Total number of lines in the file.
    pub lines: usize,
    /// Map of ignored rules for specific lines.
    pub ignored_lines: IgnoredRulesMap,
}

/// Configuration for the code parser.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Whether to collect function complexity metrics.
    pub collect_complexity: bool,
    /// Whether to count primitive parameters.
    pub collect_primitive_params: bool,
    /// Whether to extract class definitions and method details.
    pub collect_classes: bool,
    /// Whether to track environment variable usage.
    pub collect_env_vars: bool,
    /// Whether to track all symbol usages (required for some design detectors).
    pub collect_used_symbols: bool,
}

impl Default for ParserConfig {
    #[inline]
    fn default() -> Self {
        Self::all()
    }
}

impl ParserConfig {
    #[inline]
    pub const fn all() -> Self {
        Self {
            collect_complexity: true,
            collect_primitive_params: true,
            collect_classes: true,
            collect_env_vars: true,
            collect_used_symbols: true,
        }
    }

    #[inline]
    pub const fn minimal() -> Self {
        Self {
            collect_complexity: false,
            collect_primitive_params: false,
            collect_classes: false,
            collect_env_vars: false,
            collect_used_symbols: false,
        }
    }

    pub fn from_active_detectors(active_ids: &HashSet<String>) -> Self {
        Self {
            collect_complexity: active_ids.iter().any(|id| {
                matches!(
                    id.as_str(),
                    "complexity"
                        | "cyclomatic_complexity"
                        | "cognitive_complexity"
                        | "deep_nesting"
                        | "long_params"
                        | "hub_module"
                        | "god_module"
                        | "hub_dependency"
                )
            }),
            collect_primitive_params: active_ids.contains("primitive_obsession"),
            collect_classes: active_ids.contains("lcom")
                || active_ids.contains("dead_symbols")
                || active_ids.contains("abstractness"),
            collect_env_vars: active_ids.contains("scattered_config"),
            collect_used_symbols: active_ids.contains("scattered_module")
                || active_ids.contains("lcom"),
        }
    }
}

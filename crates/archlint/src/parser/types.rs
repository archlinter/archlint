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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedSymbol {
    pub name: SymbolName,
    pub kind: SymbolKind,
    pub is_reexport: bool,
    pub source: Option<SymbolName>,
    pub line: usize,
    pub column: usize,
    pub range: CodeRange,
    pub used_symbols: SymbolSet,
    pub is_mutable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedSymbol {
    pub name: SymbolName,
    pub alias: Option<SymbolName>,
    pub source: SymbolName,
    pub line: usize,
    pub column: usize,
    pub range: CodeRange,
    pub is_type_only: bool,
    pub is_reexport: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MethodAccessibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodSymbol {
    pub name: SymbolName,
    pub used_fields: SymbolSet,
    pub used_methods: SymbolSet,
    pub line: usize,
    pub column: usize,
    pub range: CodeRange,
    pub has_decorators: bool,
    pub is_accessor: bool,
    pub accessibility: Option<MethodAccessibility>,
    pub is_abstract: bool,
}

impl MethodSymbol {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSymbol {
    pub name: SymbolName,
    pub super_class: Option<SymbolName>,
    pub implements: Vec<SymbolName>,
    pub fields: SmallVec<[SymbolName; 8]>,
    pub methods: SmallVec<[MethodSymbol; 8]>,
    pub is_abstract: bool,
}

impl ClassSymbol {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileSymbols {
    pub exports: Vec<ExportedSymbol>,
    pub imports: Vec<ImportedSymbol>,
    pub classes: Vec<ClassSymbol>,
    pub local_definitions: Vec<SymbolName>,
    pub local_usages: SymbolSet,
    pub has_runtime_code: bool,
    pub env_vars: SymbolSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionComplexity {
    pub name: SymbolName,
    pub line: usize,
    pub range: CodeRange,
    pub complexity: usize,
    pub max_depth: usize,
    pub param_count: usize,
    pub primitive_params: usize,
    pub is_constructor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedFile {
    pub symbols: FileSymbols,
    pub functions: Vec<FunctionComplexity>,
    pub lines: usize,
    pub ignored_lines: IgnoredRulesMap,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ParserConfig {
    pub collect_complexity: bool,
    pub collect_primitive_params: bool,
    pub collect_classes: bool,
    pub collect_env_vars: bool,
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
                        | "deep_nesting"
                        | "long_params"
                        | "hub_module"
                        | "god_module"
                        | "hub_dependency"
                )
            }),
            collect_primitive_params: active_ids.contains("primitive_obsession"),
            collect_classes: active_ids.contains("lcom") || active_ids.contains("dead_symbols"),
            collect_env_vars: active_ids.contains("scattered_config"),
            collect_used_symbols: active_ids.contains("scattered_module"),
        }
    }
}

use crate::detectors::detector;

/// Initializes the detector module.
/// This function is used for module registration side-effects.
pub const fn init() {}

impl_complexity_detector! {
    struct: CyclomaticComplexityDetector,
    smell: HighCyclomaticComplexity,
    id: "cyclomatic_complexity",
    title: "High Cyclomatic Complexity Functions",
    reason: "High cyclomatic complexity indicates that the function has too many decision points (if, for, while, etc.), making it difficult to understand, test, and maintain.",
    risks: [
        "Higher probability of bugs due to complex logic",
        "Difficult to achieve high test coverage",
        "Hard for other developers to read and understand",
        "Refactoring becomes dangerous and difficult"
    ],
    recs: [
        "Extract complex nested logic into smaller, focused helper functions",
        "Use early returns to reduce nesting depth",
        "Simplify logical expressions",
        "Consider using design patterns like Strategy or Command for complex branching",
        "Break down large switch statements"
    ]
}

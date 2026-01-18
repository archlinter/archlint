use crate::detectors::detector;

/// Initializes the detector module.
pub const fn init() {}

impl_complexity_detector! {
    struct: CognitiveComplexityDetector,
    smell: HighCognitiveComplexity,
    id: "cognitive_complexity",
    title: "High Cognitive Complexity Functions",
    reason: "High cognitive complexity indicates that the function is difficult to understand and maintain due to complex logic and deep nesting.",
    risks: [
        "Extremely hard for developers to follow the logic",
        "High probability of bugs during maintenance",
        "Difficult to test all possible execution paths",
        "Mental overload for anybody reading the code"
    ],
    recs: [
        "Extract nested blocks into well-named helper functions",
        "Use early returns (guard clauses) to flatten the code",
        "Simplify complex boolean expressions",
        "Replace nested if/else with polymorphism or a lookup table",
        "Break down large loops into smaller processing steps"
    ]
}

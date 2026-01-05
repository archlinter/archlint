use super::types::{ExplainBlock, Regression, RegressionType};
use crate::snapshot::SnapshotSmell;

pub fn generate_explain(regression: &Regression) -> ExplainBlock {
    match &regression.regression_type {
        RegressionType::NewSmell => explain_for_smell_type(&regression.smell),
        RegressionType::SeverityIncrease { from, to } => {
            explain_severity_increase(&regression.smell, from, to)
        }
        RegressionType::MetricWorsening {
            metric,
            from,
            to,
            change_percent,
        } => explain_metric_worsening(&regression.smell, metric, *from, *to, *change_percent),
    }
}

pub fn explain_for_smell_type(smell: &SnapshotSmell) -> ExplainBlock {
    match smell.smell_type.as_str() {
        "CyclicDependency" | "CyclicDependencyCluster" => ExplainBlock {
            why_bad: "Bidirectional dependencies create tight coupling between modules. \
                Changes in one module force changes in others, and testing requires \
                complex mocking of the entire cycle."
                .to_string(),
            consequences: "As the codebase grows, cycles become harder to break. \
                Build times increase, and refactoring becomes risky. \
                New developers struggle to understand module boundaries."
                .to_string(),
            how_to_fix: "1. Extract shared code into a new module that both can depend on.\n\
                2. Use dependency injection to invert one direction.\n\
                3. Introduce an interface/abstract layer to break the direct dependency.\n\
                4. Consider if the modules should be merged."
                .to_string(),
        },

        "LayerViolation" => ExplainBlock {
            why_bad: "Bypassing architectural layers breaks encapsulation. \
                Inner layers should not know about outer layers. \
                This creates hidden dependencies and makes the architecture fragile."
                .to_string(),
            consequences:
                "Layer violations spread - once one exists, developers copy the pattern. \
                Testing becomes harder as you need to mock multiple layers. \
                Changes in inner layers unexpectedly break outer layers."
                    .to_string(),
            how_to_fix: "1. Route access through the proper layer interface.\n\
                2. Move the required functionality to the appropriate layer.\n\
                3. Create a proper abstraction if cross-layer communication is needed.\n\
                4. Review if layer boundaries are correctly defined."
                .to_string(),
        },

        "GodModule" => ExplainBlock {
            why_bad: "High fan-in/fan-out indicates a module with too many responsibilities. \
                It becomes a bottleneck for changes and a single point of failure. \
                Understanding and maintaining it requires knowing the entire codebase."
                .to_string(),
            consequences: "God modules attract more code ('gravity effect'). \
                Multiple developers frequently conflict on the same file. \
                Testing requires massive setup, and bugs have wide blast radius."
                .to_string(),
            how_to_fix: "1. Identify distinct responsibilities and extract them.\n\
                2. Group related functions into cohesive submodules.\n\
                3. Use the Facade pattern to maintain API while splitting internals.\n\
                4. Apply Single Responsibility Principle progressively."
                .to_string(),
        },

        "HubModule" => ExplainBlock {
            why_bad: "Hub modules sit at the center of the dependency graph. \
                They are imported by many modules and import many others. \
                Any change has cascading effects throughout the codebase."
                .to_string(),
            consequences: "Hub modules become change bottlenecks. \
                CI/CD pipelines slow down as most changes trigger full rebuilds. \
                Code ownership becomes unclear."
                .to_string(),
            how_to_fix: "1. Split into smaller, focused modules.\n\
                2. Use barrel files only for public API, not internal wiring.\n\
                3. Move utility functions closer to their consumers.\n\
                4. Consider if some dependencies should be inverted."
                .to_string(),
        },

        "DeadCode" | "DeadSymbol" => ExplainBlock {
            why_bad: "Unused code increases cognitive load and maintenance burden. \
                Developers waste time understanding code that's never executed. \
                Dead code can hide security vulnerabilities."
                .to_string(),
            consequences: "Dead code accumulates technical debt. \
                It confuses new team members and slows onboarding. \
                Refactoring becomes harder as boundaries are unclear."
                .to_string(),
            how_to_fix: "1. Remove the unused code if confident it's not needed.\n\
                2. Check if it's used in tests or configuration.\n\
                3. If keeping for future use, add documentation explaining why.\n\
                4. Consider feature flags instead of dead code."
                .to_string(),
        },

        "HighComplexity" => ExplainBlock {
            why_bad: "High cyclomatic complexity means too many code paths. \
                Such functions are hard to understand, test, and debug. \
                Bug probability increases exponentially with complexity."
                .to_string(),
            consequences: "Complex functions accumulate bugs. \
                Code coverage requires exponential test cases. \
                Future changes are risky without comprehensive tests."
                .to_string(),
            how_to_fix: "1. Extract conditional branches into separate functions.\n\
                2. Use early returns to reduce nesting.\n\
                3. Apply strategy/state patterns for complex conditionals.\n\
                4. Consider if the function does too many things."
                .to_string(),
        },

        "LowCohesion" => ExplainBlock {
            why_bad: "Low cohesion (high LCOM) indicates a class doing unrelated things. \
                Methods don't share fields, suggesting the class should be split. \
                This violates the Single Responsibility Principle."
                .to_string(),
            consequences: "Changes require understanding unrelated code. \
                Testing requires mocking unrelated dependencies. \
                Class becomes a dumping ground for miscellaneous code."
                .to_string(),
            how_to_fix: "1. Group methods that use common fields into separate classes.\n\
                2. Extract utility methods into standalone functions.\n\
                3. Use composition instead of one large class.\n\
                4. Consider if data and behavior are properly co-located."
                .to_string(),
        },

        "SdpViolation" => ExplainBlock {
            why_bad: "Stable modules depending on unstable modules violates the \
                Stable Dependencies Principle. Stable code should not depend on \
                frequently changing code."
                .to_string(),
            consequences: "Stable, well-tested code becomes fragile. \
                Changes in unstable dependencies break stable consumers. \
                Architecture erodes as stability boundaries blur."
                .to_string(),
            how_to_fix: "1. Invert the dependency using interfaces.\n\
                2. Move shared code to a stable common module.\n\
                3. Re-evaluate module stability classifications.\n\
                4. Use dependency injection to decouple."
                .to_string(),
        },

        // Default for other types
        _ => ExplainBlock {
            why_bad: format!(
                "This {} introduces architectural debt that makes the codebase \
                harder to maintain and evolve.",
                smell.smell_type
            ),
            consequences: "Without addressing this, the problem will grow and \
                spread to other parts of the codebase."
                .to_string(),
            how_to_fix: "Review the architectural guidelines and refactor \
                to align with established patterns."
                .to_string(),
        },
    }
}

pub fn explain_severity_increase(smell: &SnapshotSmell, from: &str, to: &str) -> ExplainBlock {
    let base = explain_for_smell_type(smell);

    ExplainBlock {
        why_bad: format!(
            "An existing {} got worse (severity: {} → {}). {}",
            smell.smell_type, from, to, base.why_bad
        ),
        consequences: format!(
            "The problem is growing, not shrinking. {}",
            base.consequences
        ),
        how_to_fix: format!(
            "Address this before it becomes Critical. {}",
            base.how_to_fix
        ),
    }
}

pub fn explain_metric_worsening(
    smell: &SnapshotSmell,
    metric: &str,
    from: f64,
    to: f64,
    change_percent: f64,
) -> ExplainBlock {
    let metric_explanation = match metric {
        "fanIn" => "More modules now depend on this code. Changes here affect more consumers.",
        "fanOut" => "This module now depends on more external code. It's becoming harder to test.",
        "complexity" => "More code paths added. Testing and debugging become harder.",
        "cycleLength" => "The dependency cycle grew larger. Breaking it becomes more expensive.",
        "lcom" => "Class cohesion decreased. Methods share less state, suggesting split needed.",
        "cbo" => "Coupling increased. The module is now harder to change independently.",
        _ => "This metric indicates growing architectural debt.",
    };

    ExplainBlock {
        why_bad: format!(
            "{} increased from {} to {} (+{:.0}%). {}",
            metric, from as i64, to as i64, change_percent, metric_explanation
        ),
        consequences: format!(
            "If {} continues growing, the {} will become unmaintainable. \
            Consider this a warning sign.",
            metric, smell.smell_type
        ),
        how_to_fix: format!(
            "Review recent changes to understand why {} increased. \
            Consider refactoring to reduce coupling.",
            metric
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_test_smell(smell_type: &str) -> SnapshotSmell {
        SnapshotSmell {
            id: "test".to_string(),
            smell_type: smell_type.to_string(),
            severity: "High".to_string(),
            files: vec!["test.ts".to_string()],
            metrics: HashMap::new(),
            details: None,
            locations: vec![],
        }
    }

    #[test]
    fn test_explain_cycle() {
        let smell = make_test_smell("CyclicDependency");
        let explain = explain_for_smell_type(&smell);

        assert!(explain.why_bad.contains("coupling"));
        assert!(explain.how_to_fix.contains("Extract"));
    }

    #[test]
    fn test_explain_metric_worsening() {
        let smell = make_test_smell("GodModule");
        let explain = explain_metric_worsening(&smell, "fanIn", 10.0, 25.0, 150.0);

        assert!(explain.why_bad.contains("150%"));
        assert!(explain.why_bad.contains("fanIn"));
    }

    #[test]
    fn test_all_smell_types_have_explain() {
        let smell_types = [
            "CyclicDependency",
            "LayerViolation",
            "GodModule",
            "DeadCode",
            "HighComplexity",
            "HubModule",
            "LowCohesion",
        ];

        for smell_type in smell_types {
            let smell = make_test_smell(smell_type);
            let explain = explain_for_smell_type(&smell);

            assert!(!explain.why_bad.is_empty());
            assert!(!explain.how_to_fix.is_empty());
        }
    }
}

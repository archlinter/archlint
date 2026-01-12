/// Helper macro to create Vec<String> from string literals
#[macro_export]
macro_rules! strings {
    ($($s:expr),* $(,)?) => {
        vec![$($s.to_string()),*]
    };
}

#[macro_export]
macro_rules! impl_detector_report {
    // Shared logic for render_markdown to avoid duplication
    (@render_markdown
        $title:expr,
        $smell_row:ident,
        $loc_var:ident,
        $pts_var:ident,
        $variant:ident,
        [ $($field:ident),* ],
        $col:tt,
        $val:tt
    ) => {
        fn render_markdown(
            &self,
            smells: &[&$crate::detectors::SmellWithExplanation],
            severity_config: &$crate::config::SeverityConfig,
            _graph: Option<&$crate::graph::DependencyGraph>,
        ) -> String {
            use $crate::detectors::SmellType;
            use $crate::report::{format_location, format_location_detail};

            $crate::impl_detector_report!(@expand_table $title, smells, severity_config, $smell_row, $loc_var, $pts_var, $variant, { $($field),* }, $col, $val)
        }
    };

    (@render_markdown
        $title:expr,
        $smell_row:ident,
        $loc_var:ident,
        $pts_var:ident,
        $variant:ident,
        [],
        $col:tt,
        $val:tt
    ) => {
        fn render_markdown(
            &self,
            smells: &[&$crate::detectors::SmellWithExplanation],
            severity_config: &$crate::config::SeverityConfig,
            _graph: Option<&$crate::graph::DependencyGraph>,
        ) -> String {
            use $crate::detectors::SmellType;
            use $crate::report::{format_location, format_location_detail};

            $crate::impl_detector_report!(@expand_table $title, smells, severity_config, $smell_row, $loc_var, $pts_var, $variant, (), $col, $val)
        }
    };

    (@expand_table $title:expr, $smells:ident, $severity_config:ident, $smell_row:ident, $loc_var:ident, $pts_var:ident, $variant:ident, { $($field:ident),* }, [ $($col:expr),* ], [ $($val:expr),* ]) => {
        $crate::define_report_section!($title, $smells, {
            $crate::render_table!(
                vec![$($col),*],
                $smells,
                |&($smell_row, _): &&$crate::detectors::SmellWithExplanation| {
                    if let SmellType::$variant { $($field,)* .. } = &$smell_row.smell_type {
                        let $loc_var = {
                            if let Some(file_path) = $smell_row.files.first() {
                                $smell_row.locations.first().map(format_location_detail).unwrap_or_else(|| {
                                    format_location(file_path, 0, None)
                                })
                            } else {
                                String::from("N/A")
                            }
                        };
                        let _ = &$loc_var;
                        let $pts_var = format!("{} pts", $smell_row.score($severity_config));
                        let _ = &$pts_var;

                        $(let _ = &$field;)*

                        vec![
                            $($val.to_string()),*
                        ]
                    } else {
                        // Count columns at compile-time by creating an array of unit expressions
                        const COL_COUNT: usize = [$( {
                            let _ = &$col;
                        } ),*]
                        .len();
                        vec!["-".into(); COL_COUNT]
                    }
                }
            )
        })
    };

    (@expand_table $title:expr, $smells:ident, $severity_config:ident, $smell_row:ident, $loc_var:ident, $pts_var:ident, $variant:ident, (), [ $($col:expr),* ], [ $($val:expr),* ]) => {
        $crate::define_report_section!($title, $smells, {
            $crate::render_table!(
                vec![$($col),*],
                $smells,
                |&($smell_row, _): &&$crate::detectors::SmellWithExplanation| {
                    if let SmellType::$variant = &$smell_row.smell_type {
                        let $loc_var = {
                            if let Some(file_path) = $smell_row.files.first() {
                                $smell_row.locations.first().map(format_location_detail).unwrap_or_else(|| {
                                    format_location(file_path, 0, None)
                                })
                            } else {
                                String::from("N/A")
                            }
                        };
                        let _ = &$loc_var;
                        let $pts_var = format!("{} pts", $smell_row.score($severity_config));
                        let _ = &$pts_var;

                    vec![
                        $($val.to_string()),*
                    ]
                } else {
                    // Count columns at compile-time by creating an array of unit expressions
                    const COL_COUNT: usize = [$( {
                        let _ = &$col;
                    } ),*]
                    .len();
                    vec!["-".into(); COL_COUNT]
                }
                }
            )
        })
    };

    // Pattern 1: Structured explain (problem/reason/risks/recommendations) - uses () delimiters
    (
        name: $name:expr,
        explain: $smell_expl:ident => (
            problem: $problem:expr,
            reason: $reason:expr,
            risks: [$($risk:expr),* $(,)?],
            recommendations: [$($rec:expr),* $(,)?]
        )
        $(, table: {
            title: $title:expr,
            columns: [$($col:expr),*],
            row: $variant:ident $( { $($field:ident),* } )? ($smell_row:ident, $loc_var:ident, $pts_var:ident) => [$($val:expr),*]
        })?
        $(,)?
    ) => {
        fn name(&self) -> &'static str {
            $name
        }

        fn explain(&self, $smell_expl: &$crate::detectors::ArchSmell) -> $crate::detectors::Explanation {
            let _ = $smell_expl;
            $crate::detectors::Explanation {
                problem: String::from($problem),
                reason: String::from($reason),
                risks: vec![$($risk.to_string()),*],
                recommendations: vec![$($rec.to_string()),*],
            }
        }

        $(
            $crate::impl_detector_report!(@render_markdown $title, $smell_row, $loc_var, $pts_var, $variant, [ $($($field),*)? ], [$($col),*], [$($val),*]);
        )?
    };

    // Pattern 2: Block-based explain (returns Explanation directly) - uses {} block
    (
        name: $name:expr,
        explain: $smell_expl:ident => $explain_body:block
        $(, table: {
            title: $title:expr,
            columns: [$($col:expr),*],
            row: $variant:ident $( { $($field:ident),* } )? ($smell_row:ident, $loc_var:ident, $pts_var:ident) => [$($val:expr),*]
        })?
        $(,)?
    ) => {
        fn name(&self) -> &'static str {
            $name
        }

        fn explain(&self, $smell_expl: &$crate::detectors::ArchSmell) -> $crate::detectors::Explanation {
            let _ = $smell_expl;
            $explain_body
        }

        $(
            $crate::impl_detector_report!(@render_markdown $title, $smell_row, $loc_var, $pts_var, $variant, [ $($($field),*)? ], [$($col),*], [$($val),*]);
        )?
    };
}

#[macro_export]
macro_rules! define_report_section {
    ($title:expr, $items:ident, $body:expr) => {{
        if $items.is_empty() {
            String::new()
        } else {
            let mut output = String::new();
            output.push_str(&format!("## {} ({} items)\n\n", $title, $items.len()));

            if let Some((_, explanation)) = $items.first() {
                $crate::report::markdown::common::append_explanation(&mut output, explanation);
            }

            let body_str: String = $body;
            output.push_str(&body_str);
            output.push('\n');
            output
        }
    }};
}

/// Helper macro to render a markdown table.
///
/// NOTE: The $row_gen closure MUST return a Vec<String> with the same
/// length as the $headers vector.
#[macro_export]
macro_rules! render_table {
    ($headers:expr, $items:ident, $row_gen:expr) => {{
        let mut table = String::new();
        table.push_str("| ");
        table.push_str(&$headers.join(" | "));
        table.push_str(" |\n|");
        for _ in 0..$headers.len() {
            table.push_str("----------|");
        }
        table.push('\n');

        for item in $items {
            let row: Vec<String> = $row_gen(item);
            table.push_str("| ");
            table.push_str(&row.join(" | "));
            table.push_str(" |\n");
        }
        table
    }};
}

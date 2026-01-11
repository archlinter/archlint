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

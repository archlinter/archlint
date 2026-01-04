use super::common::{append_explanation, group_files_by_directory, SmellWithExplanation};

pub fn generate(dead: &[&SmellWithExplanation]) -> String {
    if dead.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    output.push_str(&format!("## Dead Code ({} files)\n\n", dead.len()));

    if let Some((_, explanation)) = dead.first() {
        append_explanation(&mut output, explanation);
    }

    output.push_str("### Files by Directory\n\n");
    let grouped = group_files_by_directory(dead);

    for (dir, files) in grouped {
        output.push_str(&format!("**{}** ({} files):\n", dir, files.len()));
        for file in files {
            output.push_str(&format!("- `{}`\n", file));
        }
        output.push('\n');
    }

    output
}

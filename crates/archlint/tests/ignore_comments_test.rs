use archlint::parser::ImportParser;

#[test]
fn test_parse_ignore_comments_line() {
    let parser = ImportParser::new().unwrap();
    let code = r#"
// archlint-disable-line complexity
function complex() {}

function another() {} // archlint-disable-line *
"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    // Line numbers are 1-based in our LineIndex
    // Line 2: // archlint-disable-line complexity
    // Line 3: function complex() {}
    // Line 5: function another() {} // archlint-disable-line *

    println!("Ignored lines: {:?}", result.ignored_lines);

    assert!(result.ignored_lines.get(&2).unwrap().contains("complexity"));
    assert!(result.ignored_lines.get(&5).unwrap().contains("*"));
}

#[test]
fn test_parse_ignore_comments_next_line() {
    let parser = ImportParser::new().unwrap();
    let code = r#"
// archlint-disable-next-line complexity
function complex() {}

// archlint-disable-next-line *
function another() {}
"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    // Line 2: // archlint-disable-next-line complexity (affects line 3)
    // Line 5: // archlint-disable-next-line * (affects line 6)

    println!("Ignored lines: {:?}", result.ignored_lines);

    assert!(result.ignored_lines.get(&3).unwrap().contains("complexity"));
    assert!(result.ignored_lines.get(&6).unwrap().contains("*"));
}

#[test]
fn test_parse_ignore_comments_file_wide() {
    let parser = ImportParser::new().unwrap();
    let code = r#"
// archlint-disable
function complex() {}
"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    // archlint-disable at the top (line 2) marks line 0 as "ignore all"
    println!("Ignored lines: {:?}", result.ignored_lines);

    assert!(result.ignored_lines.get(&0).unwrap().contains("*"));
}

#[test]
fn test_parse_ignore_comments_with_rules() {
    let parser = ImportParser::new().unwrap();
    let code = r#"
// archlint-disable-line complexity, large_file, custom-rule
function complex() {}
"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    let rules = result.ignored_lines.get(&2).unwrap();
    assert!(rules.contains("complexity"));
    assert!(rules.contains("large_file"));
    assert!(rules.contains("custom-rule"));
    assert_eq!(rules.len(), 3);
}

use archlint::parser::ImportParser;

const FILE_WIDE_IGNORE: usize = 0;

#[test]
fn test_parse_ignore_comments_line() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable-line complexity

function complex() {}

function another() {} // archlint-disable-line *"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    // Line numbers are 1-based in our LineIndex
    // Line 1: // archlint-disable-line complexity
    // Line 5: function another() {} // archlint-disable-line *

    assert!(result
        .ignored_lines
        .get(&1)
        .expect("expected ignore rules for line 1")
        .contains("complexity"));
    assert!(result
        .ignored_lines
        .get(&5)
        .expect("expected ignore rules for line 5")
        .contains("*"));
}

#[test]
fn test_parse_ignore_comments_next_line() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable-next-line complexity
function complex() {}

// archlint-disable-next-line *
function another() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    // Line 1: // archlint-disable-next-line complexity (affects line 2)
    // Line 4: // archlint-disable-next-line * (affects line 5)

    assert!(result
        .ignored_lines
        .get(&2)
        .expect("expected ignore rules for line 2")
        .contains("complexity"));
    assert!(result
        .ignored_lines
        .get(&5)
        .expect("expected ignore rules for line 5")
        .contains("*"));
}

#[test]
fn test_parse_ignore_comments_file_wide() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable
function complex() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    // Should contain line 0 (magic), 1 (the comment itself), and 2 (the function)
    assert!(result
        .ignored_lines
        .get(&FILE_WIDE_IGNORE)
        .expect("expected file-wide ignore rules")
        .contains("*"));
    assert!(result.ignored_lines.get(&1).unwrap().contains("*"));
    assert!(result.ignored_lines.get(&2).unwrap().contains("*"));
}

#[test]
fn test_parse_ignore_comments_block() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable complexity
function a() {}
function b() {}
// archlint-enable complexity
function c() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    // complexity should be ignored on lines 1-4
    assert!(result.ignored_lines.get(&1).unwrap().contains("complexity"));
    assert!(result.ignored_lines.get(&2).unwrap().contains("complexity"));
    assert!(result.ignored_lines.get(&3).unwrap().contains("complexity"));
    assert!(result.ignored_lines.get(&4).unwrap().contains("complexity"));

    // line 5 should not be ignored
    assert!(!result.ignored_lines.contains_key(&5));
}

#[test]
fn test_parse_ignore_comments_block_all() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable
function a() {}
// archlint-enable
function b() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    assert!(result.ignored_lines.get(&1).unwrap().contains("*"));
    assert!(result.ignored_lines.get(&2).unwrap().contains("*"));
    assert!(result.ignored_lines.get(&3).unwrap().contains("*"));

    assert!(!result.ignored_lines.contains_key(&4));
}

#[test]
fn test_parse_ignore_comments_with_rules() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable-line complexity, large_file, custom-rule
function complex() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    let rules = result.ignored_lines.get(&1).unwrap();
    assert!(rules.contains("complexity"));
    assert!(rules.contains("large_file"));
    assert!(rules.contains("custom-rule"));
    assert_eq!(rules.len(), 3);
}

#[test]
fn test_parse_ignore_comments_with_reasons() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable-line complexity because this function is just huge
function complex() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    let rules = result.ignored_lines.get(&1).unwrap();
    assert!(rules.contains("complexity"));
    assert_eq!(rules.len(), 1);
}

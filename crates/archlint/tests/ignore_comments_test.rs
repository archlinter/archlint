use archlint::parser::ImportParser;

const FILE_WIDE_IGNORE: usize = 0;

fn get_line_number(code: &str, pattern: &str) -> usize {
    code.lines()
        .position(|line| line.contains(pattern))
        .map(|pos| pos + 1)
        .unwrap_or_else(|| panic!("Pattern '{}' not found in code", pattern))
}

#[test]
fn test_parse_ignore_comments_line() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable-line cyclomatic_complexity

function complex() {}

function another() {} // archlint-disable-line *"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    let line_cyclomatic_complexity =
        get_line_number(code, "archlint-disable-line cyclomatic_complexity");
    let line_another = get_line_number(code, "function another()");

    assert!(result
        .ignored_lines
        .get(&line_cyclomatic_complexity)
        .expect("expected ignore rules for cyclomatic_complexity line")
        .contains("cyclomatic_complexity"));
    assert!(result
        .ignored_lines
        .get(&line_another)
        .expect("expected ignore rules for another() line")
        .contains("*"));
}

#[test]
fn test_parse_ignore_comments_next_line() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable-next-line cyclomatic_complexity
function complex() {}

// archlint-disable-next-line *
function another() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    let line_complex_func = get_line_number(code, "function complex()");
    let line_another_func = get_line_number(code, "function another()");

    assert!(result
        .ignored_lines
        .get(&line_complex_func)
        .expect("expected ignore rules for complex() function line")
        .contains("cyclomatic_complexity"));
    assert!(result
        .ignored_lines
        .get(&line_another_func)
        .expect("expected ignore rules for another() function line")
        .contains("*"));
}

#[test]
fn test_parse_ignore_comments_file_wide() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable
function complex() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    assert!(result
        .ignored_lines
        .get(&FILE_WIDE_IGNORE)
        .expect("expected file-wide ignore rules")
        .contains("*"));

    let line_comment = get_line_number(code, "// archlint-disable");
    let line_func = get_line_number(code, "function complex()");

    assert!(result
        .ignored_lines
        .get(&line_comment)
        .unwrap()
        .contains("*"));
    assert!(result.ignored_lines.get(&line_func).unwrap().contains("*"));
}

#[test]
fn test_parse_ignore_comments_block() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable cyclomatic_complexity
function a() {}
function b() {}
// archlint-enable cyclomatic_complexity
function c() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    let line_a = get_line_number(code, "function a()");
    let line_b = get_line_number(code, "function b()");
    let line_c = get_line_number(code, "function c()");
    let line_disable = get_line_number(code, "// archlint-disable cyclomatic_complexity");
    let line_enable = get_line_number(code, "// archlint-enable cyclomatic_complexity");

    assert!(result
        .ignored_lines
        .get(&line_disable)
        .unwrap()
        .contains("cyclomatic_complexity"));
    assert!(result
        .ignored_lines
        .get(&line_a)
        .unwrap()
        .contains("cyclomatic_complexity"));
    assert!(result
        .ignored_lines
        .get(&line_b)
        .unwrap()
        .contains("cyclomatic_complexity"));
    assert!(result
        .ignored_lines
        .get(&line_enable)
        .unwrap()
        .contains("cyclomatic_complexity"));

    assert!(!result.ignored_lines.contains_key(&line_c));
}

#[test]
fn test_parse_ignore_comments_block_all() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable
function a() {}
// archlint-enable
function b() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    let line_disable = get_line_number(code, "// archlint-disable");
    let line_a = get_line_number(code, "function a()");
    let line_enable = get_line_number(code, "// archlint-enable");
    let line_b = get_line_number(code, "function b()");

    assert!(result
        .ignored_lines
        .get(&line_disable)
        .unwrap()
        .contains("*"));
    assert!(result.ignored_lines.get(&line_a).unwrap().contains("*"));
    assert!(result
        .ignored_lines
        .get(&line_enable)
        .unwrap()
        .contains("*"));

    assert!(!result.ignored_lines.contains_key(&line_b));
}

#[test]
fn test_parse_ignore_comments_with_rules() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable-line cyclomatic_complexity, large_file, custom-rule
function complex() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    let line_comment = get_line_number(code, "archlint-disable-line");
    let rules = result.ignored_lines.get(&line_comment).unwrap();
    assert!(rules.contains("cyclomatic_complexity"));
    assert!(rules.contains("large_file"));
    assert!(rules.contains("custom-rule"));
    assert_eq!(rules.len(), 3);
}

#[test]
fn test_parse_ignore_comments_with_reasons() {
    let parser = ImportParser::new().unwrap();
    let code = r#"// archlint-disable-line cyclomatic_complexity because this function is just huge
function complex() {}"#;
    let result = parser.parse_code(code, "test.ts").unwrap();

    let line_comment = get_line_number(code, "archlint-disable-line");
    let rules = result.ignored_lines.get(&line_comment).unwrap();
    assert!(rules.contains("cyclomatic_complexity"));
    assert_eq!(rules.len(), 1);
}

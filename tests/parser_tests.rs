use doommy::ConfigParser;

#[test]
fn test_define_constants() {
    let input = r#"(def A 10); (def B 20);"#;
    let (yaml, _) = ConfigParser::parse(input).unwrap();
    let output = serde_yaml::to_string(&yaml).unwrap();
    assert_eq!(output.trim(), "{}"); // Пустой YAML
}

#[test]
fn test_list_parsing() {
    let input = r#"(def A 5); (list 1 2 A 10);"#;
    let (yaml, _) = ConfigParser::parse(input).unwrap();
    let output = serde_yaml::to_string(&yaml).unwrap();
    let expected = r"
List:
- 1
- 2
- 5
- 10
";
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_dictionary_parsing() {
    let input = r#"$[ KEY: 10, OTHERKEY: (list 1 2 3) ];"#;
    let expected = r#"
KEY: 10
OTHERKEY:
- 1
- 2
- 3
"#;
    let (yaml, _) = ConfigParser::parse(input).unwrap();
    let output = serde_yaml::to_string(&yaml).unwrap();
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_expression_parsing() {
    let input = r#"(def A 10); (def B 20); $[RES: ^{A + B + pow(A, 2)}];"#;
    let (yaml, _) = ConfigParser::parse(input).unwrap();
    let output = serde_yaml::to_string(&yaml).unwrap();
    let expected = r#"
RES: 130
"#;
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_nested_structures() {
    let input = "(def A 2);$[KEY: (list A 3 $[ SUBKEY: ^{pow(A, 3)}])];";
    let (yaml, _) = ConfigParser::parse(input).unwrap();
    let output = serde_yaml::to_string(&yaml).unwrap();
    let expected = r#"
KEY:
- 2
- 3
- SUBKEY: 8
"#;
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_invalid_name() {
    let input = r#"[A];"#;
    let result = ConfigParser::parse(input);
    assert!(result.is_err());
}

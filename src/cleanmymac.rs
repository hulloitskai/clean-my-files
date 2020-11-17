use crate::prelude::*;
use regex::Regex;

lazy_static! {
    static ref ERROR_REGEX: Regex =
        Regex::new(r#"^"(?P<path>.*)" couldn’t be removed (?P<reason>.*)\.$"#)
            .unwrap();
}

pub fn parse_report(report: &str) -> Result<Vec<CleanError>> {
    let mut errors: Vec<CleanError> = Vec::new();
    for (i, line) in report.lines().enumerate() {
        let captures = match ERROR_REGEX.captures_iter(line).next() {
            Some(captures) => captures,
            None => continue,
        };

        let path = match captures.name("path") {
            Some(r#match) => r#match.as_str().to_owned(),
            None => panic!("missing path on line {}", i),
        };

        let reason = match captures.name("reason") {
            Some(r#match) => r#match.as_str().to_owned(),
            None => panic!("missing error reason on line {}", i),
        };
        let reason = strip_prefix_if_exists(reason, "due to ");
        let reason = strip_prefix_if_exists(reason, "because of ");

        let error = CleanError::new(path, reason);
        errors.push(error);
    }
    Ok(errors)
}

fn strip_prefix_if_exists<'a>(s: String, prefix: &'a str) -> String {
    match s.strip_prefix(prefix) {
        Some(remaining) => remaining.to_owned(),
        None => s,
    }
}

#[derive(Debug, Clone, Hash)]
pub struct CleanError {
    pub file_path: String,
    pub reason: String,
}

impl CleanError {
    fn new(file_path: String, reason: String) -> Self {
        Self { file_path, reason }
    }
}

use lsp_types::*;

use crate::document::Document;

pub fn validate_document(doc: &Document) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let lines: Vec<&str> = doc.text.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let line_num = line_idx as u32;

        if trimmed.is_empty() {
            continue;
        }

        check_unmatched_braces(trimmed, line_num, &mut diagnostics);
        check_missing_semicolon(trimmed, line_num, &mut diagnostics);
        check_unmatched_parens(trimmed, line_num, &mut diagnostics);
        check_invalid_indentation(line, line_num, &mut diagnostics);
        check_duplicate_let(trimmed, line_num, &mut diagnostics);
        check_func_syntax(trimmed, line_num, &mut diagnostics);
        check_class_syntax(trimmed, line_num, &mut diagnostics);
    }

    diagnostics
}

fn check_unmatched_braces(line: &str, line_num: u32, diagnostics: &mut Vec<Diagnostic>) {
    let open = line.chars().filter(|&c| c == '{').count();
    let close = line.chars().filter(|&c| c == '}').count();
    if open != close {
        diagnostics.push(Diagnostic {
            range: Range::new(
                Position::new(line_num, 0),
                Position::new(line_num, line.len() as u32),
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("unmatched-braces".to_string())),
            message: format!("Unmatched braces: {} opening, {} closing", open, close),
            ..Default::default()
        });
    }
}

fn check_missing_semicolon(line: &str, line_num: u32, diagnostics: &mut Vec<Diagnostic>) {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.ends_with('{') || trimmed.ends_with('}')
        || trimmed.starts_with("//") || trimmed.starts_with("func ")
        || trimmed.starts_with("class ") || trimmed.starts_with("if ")
        || trimmed.starts_with("else") || trimmed.starts_with("while ")
        || trimmed.starts_with("for ") || trimmed.starts_with("try")
        || trimmed.starts_with("catch") || trimmed.starts_with("finally")
    {
        return;
    }

    if trimmed.starts_with("let ") || trimmed.starts_with("let\t") {
        if !trimmed.ends_with(';') && !trimmed.ends_with('{') {
            diagnostics.push(Diagnostic {
                range: Range::new(
                    Position::new(line_num, 0),
                    Position::new(line_num, line.len() as u32),
                ),
                severity: Some(DiagnosticSeverity::WARNING),
                code: Some(NumberOrString::String("missing-semicolon".to_string())),
                message: "Missing semicolon at end of statement".to_string(),
                ..Default::default()
            });
        }
    }

    if trimmed.starts_with("return ") {
        if !trimmed.ends_with(';') {
            diagnostics.push(Diagnostic {
                range: Range::new(
                    Position::new(line_num, 0),
                    Position::new(line_num, line.len() as u32),
                ),
                severity: Some(DiagnosticSeverity::WARNING),
                code: Some(NumberOrString::String("missing-semicolon".to_string())),
                message: "Missing semicolon after return statement".to_string(),
                ..Default::default()
            });
        }
    }
}

fn check_unmatched_parens(line: &str, line_num: u32, diagnostics: &mut Vec<Diagnostic>) {
    let open = line.chars().filter(|&c| c == '(').count();
    let close = line.chars().filter(|&c| c == ')').count();
    if open != close {
        diagnostics.push(Diagnostic {
            range: Range::new(
                Position::new(line_num, 0),
                Position::new(line_num, line.len() as u32),
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(NumberOrString::String("unmatched-parens".to_string())),
            message: format!("Unmatched parentheses: {} opening, {} closing", open, close),
            ..Default::default()
        });
    }
}

fn check_invalid_indentation(line: &str, line_num: u32, diagnostics: &mut Vec<Diagnostic>) {
    if line.starts_with('\t') && line.contains("    ") {
        diagnostics.push(Diagnostic {
            range: Range::new(
                Position::new(line_num, 0),
                Position::new(line_num, line.len() as u32),
            ),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("mixed-indentation".to_string())),
            message: "Mixed tabs and spaces for indentation".to_string(),
            ..Default::default()
        });
    }
}

fn check_duplicate_let(line: &str, line_num: u32, diagnostics: &mut Vec<Diagnostic>) {
    if line.starts_with("let ") || line.starts_with("let\t") {
        let rest = line.trim_start_matches("let").trim_start();
        if let Some(name_end) = rest.find(|c: char| !c.is_alphanumeric() && c != '_') {
            let name = &rest[..name_end];
            if name == "let" || name == "true" || name == "false" || name == "null" {
                diagnostics.push(Diagnostic {
                    range: Range::new(
                        Position::new(line_num, 0),
                        Position::new(line_num, line.len() as u32),
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("invalid-name".to_string())),
                    message: format!("'{}' is a reserved keyword and cannot be used as a variable name", name),
                    ..Default::default()
                });
            }
        }
    }
}

fn check_func_syntax(line: &str, line_num: u32, diagnostics: &mut Vec<Diagnostic>) {
    if line.starts_with("func ") || line.starts_with("func\t") {
        if !line.contains('(') || !line.contains(')') {
            diagnostics.push(Diagnostic {
                range: Range::new(
                    Position::new(line_num, 0),
                    Position::new(line_num, line.len() as u32),
                ),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("invalid-func-syntax".to_string())),
                message: "Function declaration missing parentheses".to_string(),
                ..Default::default()
            });
        }
    }
}

fn check_class_syntax(line: &str, line_num: u32, diagnostics: &mut Vec<Diagnostic>) {
    if line.starts_with("class ") || line.starts_with("class\t") {
        let after = &line[6..].trim();
        if after.is_empty() || !after.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_') {
            diagnostics.push(Diagnostic {
                range: Range::new(
                    Position::new(line_num, 0),
                    Position::new(line_num, line.len() as u32),
                ),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("invalid-class-name".to_string())),
                message: "Class declaration missing valid class name".to_string(),
                ..Default::default()
            });
        }
    }
}

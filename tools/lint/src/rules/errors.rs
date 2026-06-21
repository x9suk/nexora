use std::collections::HashMap;
use crate::{LintDiagnostic, Severity};
use crate::linter::LintRule;

pub struct UndefinedVariableRule;

impl LintRule for UndefinedVariableRule {
    fn name(&self) -> &str {
        "no-undefined-variable"
    }

    fn lint(&self, source: &str, file: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = source.lines().collect();
        let mut defined: HashMap<String, usize> = HashMap::new();

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Track variable definitions
            if trimmed.starts_with("let ") || trimmed.starts_with("const ") {
                let is_const = trimmed.starts_with("const ");
                let rest = if is_const { &trimmed[6..] } else { &trimmed[4..] };
                if let Some(name) = rest.split_whitespace().next() {
                    let name = name.trim_end_matches('=').trim().to_string();
                    if !name.is_empty() {
                        defined.insert(name, line_num + 1);
                    }
                }
            }

            // Track function parameters
            if trimmed.starts_with("fn ") {
                if let Some(params_start) = trimmed.find('(') {
                    if let Some(params_end) = trimmed.find(')') {
                        let params = &trimmed[params_start + 1..params_end];
                        for param in params.split(',') {
                            let param = param.trim().split(':').next().unwrap_or("").trim();
                            if !param.is_empty() {
                                defined.insert(param.to_string(), line_num + 1);
                            }
                        }
                    }
                }
            }

            // Track function definitions
            if trimmed.starts_with("fn ") {
                if let Some(name) = trimmed.split_whitespace().nth(1) {
                    let name = name.trim_end_matches('(').trim().to_string();
                    defined.insert(name, line_num + 1);
                }
            }
        }

        // Second pass: check for undefined variables
        let keywords = vec![
            "fn", "let", "const", "mut", "if", "else", "while", "for",
            "return", "class", "new", "this", "import", "from", "export",
            "match", "case", "default", "break", "continue", "try", "catch",
            "throw", "async", "await", "true", "false", "null", "undefined", "print",
        ];

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Skip definition lines
            if trimmed.starts_with("let ") || trimmed.starts_with("const ") || trimmed.starts_with("fn ") {
                continue;
            }

            // Check identifiers in expressions
            let words: Vec<&str> = trimmed.split_whitespace().collect();
            for word in words {
                let word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                if word.is_empty() || word.chars().next().map_or(false, |c| c.is_numeric()) {
                    continue;
                }

                if !keywords.contains(&word) && !defined.contains_key(word) {
                    // Check if it's a property access or method call
                    if !trimmed.contains(&format!("{}.", word)) && !trimmed.contains(&format!("{}(", word)) {
                        diagnostics.push(LintDiagnostic {
                            file: file.to_string(),
                            line: line_num + 1,
                            column: line.find(word).unwrap_or(0) + 1,
                            severity: Severity::Error,
                            rule: self.name().to_string(),
                            message: format!("Variable '{}' is not defined", word),
                        });
                    }
                }
            }
        }

        diagnostics
    }
}

pub struct ConstReassignmentRule;

impl LintRule for ConstReassignmentRule {
    fn name(&self) -> &str {
        "no-const-reassignment"
    }

    fn lint(&self, source: &str, file: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = source.lines().collect();
        let mut constants: HashMap<String, usize> = HashMap::new();

        // First pass: collect const declarations
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("const ") {
                if let Some(name) = trimmed.split_whitespace().nth(1) {
                    let name = name.trim_end_matches('=').trim().to_string();
                    if !name.is_empty() {
                        constants.insert(name, line_num + 1);
                    }
                }
            }
        }

        // Second pass: check for reassignments
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Check for simple reassignment: constName = value
            if let Some(name) = trimmed.split('=').next() {
                let name = name.trim();
                if constants.contains_key(name) && !trimmed.starts_with("const ") && !trimmed.starts_with("let ") {
                    diagnostics.push(LintDiagnostic {
                        file: file.to_string(),
                        line: line_num + 1,
                        column: 1,
                        severity: Severity::Error,
                        rule: self.name().to_string(),
                        message: format!("Cannot reassign constant '{}'", name),
                    });
                }
            }

            // Check for +=, -=, etc.
            for op in &["+=","-=", "*=", "/="] {
                if trimmed.contains(op) {
                    let name = trimmed.split(op).next().unwrap_or("").trim();
                    if constants.contains_key(name) {
                        diagnostics.push(LintDiagnostic {
                            file: file.to_string(),
                            line: line_num + 1,
                            column: 1,
                            severity: Severity::Error,
                            rule: self.name().to_string(),
                            message: format!("Cannot reassign constant '{}'", name),
                        });
                    }
                }
            }
        }

        diagnostics
    }
}

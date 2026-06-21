use crate::{LintDiagnostic, Severity};
use crate::linter::LintRule;

pub struct UnusedVariablesRule;

impl LintRule for UnusedVariablesRule {
    fn name(&self) -> &str {
        "no-unused-variables"
    }

    fn lint(&self, source: &str, file: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Check for let declarations
            if trimmed.starts_with("let ") || trimmed.starts_with("const ") {
                let is_const = trimmed.starts_with("const ");
                let rest = if is_const { &trimmed[6..] } else { &trimmed[4..] };

                if let Some(name) = rest.split_whitespace().next() {
                    let name = name.trim_end_matches('=').trim();
                    if !name.is_empty() && !name.starts_with('_') {
                        // Check if variable is used elsewhere
                        let used = lines.iter().enumerate().any(|(i, l)| {
                            i != line_num && l.contains(name)
                        });

                        if !used {
                            diagnostics.push(LintDiagnostic {
                                file: file.to_string(),
                                line: line_num + 1,
                                column: line.find(name).unwrap_or(0) + 1,
                                severity: Severity::Warning,
                                rule: self.name().to_string(),
                                message: format!("Variable '{}' is declared but never used", name),
                            });
                        }
                    }
                }
            }
        }

        diagnostics
    }
}

pub struct UnusedImportsRule;

impl LintRule for UnusedImportsRule {
    fn name(&self) -> &str {
        "no-unused-imports"
    }

    fn lint(&self, source: &str, file: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("import ") && trimmed.contains("from") {
                if let Some(import_part) = trimmed.split("from").next() {
                    let import_part = import_part.trim().trim_start_matches("import ");
                    let names: Vec<&str> = if import_part.starts_with('{') {
                        import_part
                            .trim_matches('{')
                            .trim_matches('}')
                            .split(',')
                            .map(|s| s.trim())
                            .collect()
                    } else {
                        vec![import_part.trim()]
                    };

                    for name in names {
                        if !name.is_empty() {
                            let used = lines.iter().enumerate().any(|(i, l)| {
                                i != line_num && l.contains(name)
                            });

                            if !used {
                                diagnostics.push(LintDiagnostic {
                                    file: file.to_string(),
                                    line: line_num + 1,
                                    column: line.find(name).unwrap_or(0) + 1,
                                    severity: Severity::Warning,
                                    rule: self.name().to_string(),
                                    message: format!("Import '{}' is never used", name),
                                });
                            }
                        }
                    }
                }
            }
        }

        diagnostics
    }
}

pub struct ConsoleLogRule;

impl LintRule for ConsoleLogRule {
    fn name(&self) -> &str {
        "no-console-log"
    }

    fn lint(&self, source: &str, file: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            if line.contains("console.log") || line.contains("console.error") || line.contains("console.warn") {
                diagnostics.push(LintDiagnostic {
                    file: file.to_string(),
                    line: line_num + 1,
                    column: line.find("console.").unwrap_or(0) + 1,
                    severity: Severity::Warning,
                    rule: self.name().to_string(),
                    message: "Use 'print' instead of 'console.log' in Nexora".to_string(),
                });
            }
        }

        diagnostics
    }
}

pub struct ComparisonWithBoolRule;

impl LintRule for ComparisonWithBoolRule {
    fn name(&self) -> &str {
        "no-comparison-with-bool"
    }

    fn lint(&self, source: &str, file: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.contains("== true") || trimmed.contains("== false") ||
               trimmed.contains("!= true") || trimmed.contains("!= false") {
                diagnostics.push(LintDiagnostic {
                    file: file.to_string(),
                    line: line_num + 1,
                    column: 1,
                    severity: Severity::Info,
                    rule: self.name().to_string(),
                    message: "Comparison with boolean is unnecessary. Use the value directly".to_string(),
                });
            }
        }

        diagnostics
    }
}

pub struct LooseEqualityRule;

impl LintRule for LooseEqualityRule {
    fn name(&self) -> &str {
        "no-loose-equality"
    }

    fn lint(&self, source: &str, file: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            let trimmed = line.trim();

            if (trimmed.contains("==") && !trimmed.contains("===")) ||
               (trimmed.contains("!=") && !trimmed.contains("!==")) {
                diagnostics.push(LintDiagnostic {
                    file: file.to_string(),
                    line: line_num + 1,
                    column: 1,
                    severity: Severity::Warning,
                    rule: self.name().to_string(),
                    message: "Use '===' instead of '==' for strict equality".to_string(),
                });
            }
        }

        diagnostics
    }
}

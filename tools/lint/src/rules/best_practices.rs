use crate::{LintDiagnostic, Severity};
use crate::linter::LintRule;

pub struct EmptyCatchRule;

impl LintRule for EmptyCatchRule {
    fn name(&self) -> &str {
        "no-empty-catch"
    }

    fn lint(&self, source: &str, file: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = source.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            let trimmed = lines[i].trim();

            // Look for catch blocks
            if trimmed.starts_with("catch") {
                let catch_line = i + 1;
                let catch_col = lines[i].find("catch").unwrap_or(0) + 1;

                // Check if next line is just closing brace
                if i + 1 < lines.len() {
                    let next_trimmed = lines[i + 1].trim();
                    if next_trimmed == "}" {
                        diagnostics.push(LintDiagnostic {
                            file: file.to_string(),
                            line: catch_line,
                            column: catch_col,
                            severity: Severity::Warning,
                            rule: self.name().to_string(),
                            message: "Empty catch block. Consider handling the error or logging it".to_string(),
                        });
                    }
                }

                // Check for try { } catch { } pattern on same lines
                if trimmed == "catch {" && i + 1 < lines.len() && lines[i + 1].trim() == "}" {
                    diagnostics.push(LintDiagnostic {
                        file: file.to_string(),
                        line: catch_line,
                        column: catch_col,
                        severity: Severity::Warning,
                        rule: self.name().to_string(),
                        message: "Empty catch block. Consider handling the error or logging it".to_string(),
                    });
                }
            }

            i += 1;
        }

        diagnostics
    }
}

use crate::LintDiagnostic;
use crate::rules::{style, errors, best_practices};

pub struct Linter {
    rules: Vec<Box<dyn LintRule>>,
}

pub trait LintRule {
    fn name(&self) -> &str;
    fn lint(&self, source: &str, file: &str) -> Vec<LintDiagnostic>;
}

impl Linter {
    pub fn new() -> Self {
        let mut rules: Vec<Box<dyn LintRule>> = Vec::new();

        // Style rules
        rules.push(Box::new(style::UnusedVariablesRule));
        rules.push(Box::new(style::UnusedImportsRule));
        rules.push(Box::new(style::ConsoleLogRule));
        rules.push(Box::new(style::ComparisonWithBoolRule));
        rules.push(Box::new(style::LooseEqualityRule));

        // Error rules
        rules.push(Box::new(errors::UndefinedVariableRule));
        rules.push(Box::new(errors::ConstReassignmentRule));

        // Best practices
        rules.push(Box::new(best_practices::EmptyCatchRule));

        Self { rules }
    }

    pub fn lint(&self, source: &str, file: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();

        for rule in &self.rules {
            diagnostics.extend(rule.lint(source, file));
        }

        diagnostics.sort_by(|a, b| a.line.cmp(&b.line).then(a.column.cmp(&b.column)));
        diagnostics
    }
}

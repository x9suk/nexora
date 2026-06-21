use lsp_types::*;

use crate::document::Document;

fn strip_prefix_any<'a>(line: &'a str, prefixes: &[&str]) -> Option<&'a str> {
    for prefix in prefixes {
        if let Some(rest) = line.strip_prefix(prefix) {
            return Some(rest);
        }
    }
    None
}

pub fn extract_document_symbols(doc: &Document) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();
    let lines: Vec<&str> = doc.text.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if let Some(rest) = strip_prefix_any(trimmed, &["func ", "func\t"]) {
            if let Some(name) = extract_name(rest) {
                let name_start = trimmed.len() - rest.len();
                symbols.push(DocumentSymbol {
                    name,
                    detail: Some(extract_func_signature(trimmed)),
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    #[allow(deprecated)]
                    deprecated: None,
                    range: Range::new(
                        Position::new(line_idx as u32, 0),
                        Position::new(line_idx as u32, line.len() as u32),
                    ),
                    selection_range: Range::new(
                        Position::new(line_idx as u32, name_start as u32),
                        Position::new(line_idx as u32, (name_start + trimmed.len() - rest.len()) as u32),
                    ),
                    children: None,
                });
            }
        }

        if let Some(rest) = strip_prefix_any(trimmed, &["class ", "class\t"]) {
            if let Some(name) = extract_name(rest) {
                let name_start = trimmed.len() - rest.len();
                symbols.push(DocumentSymbol {
                    name,
                    detail: Some("class".to_string()),
                    kind: SymbolKind::CLASS,
                    tags: None,
                    #[allow(deprecated)]
                    deprecated: None,
                    range: Range::new(
                        Position::new(line_idx as u32, 0),
                        Position::new(line_idx as u32, line.len() as u32),
                    ),
                    selection_range: Range::new(
                        Position::new(line_idx as u32, name_start as u32),
                        Position::new(line_idx as u32, (name_start + trimmed.len() - rest.len()) as u32),
                    ),
                    children: None,
                });
            }
        }

        if let Some(rest) = strip_prefix_any(trimmed, &["let ", "let\t"]) {
            if let Some(name) = extract_name(rest) {
                let name_start = trimmed.len() - rest.len();
                symbols.push(DocumentSymbol {
                    name,
                    detail: Some("variable".to_string()),
                    kind: SymbolKind::VARIABLE,
                    tags: None,
                    #[allow(deprecated)]
                    deprecated: None,
                    range: Range::new(
                        Position::new(line_idx as u32, 0),
                        Position::new(line_idx as u32, line.len() as u32),
                    ),
                    selection_range: Range::new(
                        Position::new(line_idx as u32, name_start as u32),
                        Position::new(line_idx as u32, (name_start + trimmed.len() - rest.len()) as u32),
                    ),
                    children: None,
                });
            }
        }
    }

    symbols
}

fn extract_func_signature(line: &str) -> String {
    if line.find('(').is_some() {
        if let Some(end) = line.rfind(')') {
            return line[..=end].to_string();
        }
    }
    "func".to_string()
}

fn extract_name(rest: &str) -> Option<String> {
    let end = rest.find(|c: char| !c.is_alphanumeric() && c != '_');
    match end {
        Some(0) => None,
        Some(idx) => Some(rest[..idx].to_string()),
        None => {
            if rest.chars().all(|c| c.is_alphanumeric() || c == '_') && !rest.is_empty() {
                Some(rest.to_string())
            } else {
                None
            }
        }
    }
}

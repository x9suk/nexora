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

pub fn find_references(doc: &Document, position: Position, include_declaration: bool) -> Vec<Location> {
    let word = match get_word_at_position(doc, position) {
        Some(w) => w,
        None => return vec![],
    };

    let uri = doc.uri.clone();
    let lines: Vec<&str> = doc.text.lines().collect();
    let mut references = Vec::new();

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if !include_declaration {
            let mut is_decl = false;
            if let Some(rest) = strip_prefix_any(trimmed, &["func ", "func\t"]) {
                if let Some(name) = extract_name(rest) {
                    if name == word {
                        is_decl = true;
                    }
                }
            }
            if let Some(rest) = strip_prefix_any(trimmed, &["class ", "class\t"]) {
                if let Some(name) = extract_name(rest) {
                    if name == word {
                        is_decl = true;
                    }
                }
            }
            if let Some(rest) = strip_prefix_any(trimmed, &["let ", "let\t"]) {
                if let Some(name) = extract_name(rest) {
                    if name == word {
                        is_decl = true;
                    }
                }
            }
            if is_decl {
                continue;
            }
        }

        let mut col = 0;
        let chars: Vec<char> = line.chars().collect();
        while col < chars.len() {
            if chars[col].is_alphabetic() || chars[col] == '_' {
                let start = col;
                while col < chars.len() && (chars[col].is_alphanumeric() || chars[col] == '_') {
                    col += 1;
                }
                let found: String = chars[start..col].iter().collect();
                if found == word {
                    references.push(Location {
                        uri: uri.clone(),
                        range: Range::new(
                            Position::new(line_idx as u32, start as u32),
                            Position::new(line_idx as u32, col as u32),
                        ),
                    });
                }
            } else {
                col += 1;
            }
        }
    }

    references
}

fn get_word_at_position(doc: &Document, position: Position) -> Option<String> {
    let line = doc.text.lines().nth(position.line as usize)?;
    let char_pos = position.character as usize;
    if char_pos > line.len() {
        return None;
    }

    let bytes: Vec<u8> = line.bytes().collect();
    let mut start = char_pos;
    let mut end = char_pos;

    while start > 0 && ((bytes[start - 1] as char).is_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }
    while end < bytes.len() && ((bytes[end] as char).is_alphanumeric() || bytes[end] == b'_') {
        end += 1;
    }

    if start == end {
        return None;
    }

    Some(String::from_utf8_lossy(&bytes[start..end]).to_string())
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

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

pub fn find_definition(doc: &Document, position: Position) -> Option<Location> {
    let word = get_word_at_position(doc, position)?;
    let uri = doc.uri.clone();
    let lines: Vec<&str> = doc.text.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if let Some(rest) = strip_prefix_any(trimmed, &["func ", "func\t"]) {
            if let Some(name) = extract_name(rest) {
                if name == word {
                    return Some(Location {
                        uri: uri.clone(),
                        range: Range::new(
                            Position::new(line_idx as u32, 0),
                            Position::new(line_idx as u32, line.len() as u32),
                        ),
                    });
                }
            }
        }

        if let Some(rest) = strip_prefix_any(trimmed, &["class ", "class\t"]) {
            if let Some(name) = extract_name(rest) {
                if name == word {
                    return Some(Location {
                        uri: uri.clone(),
                        range: Range::new(
                            Position::new(line_idx as u32, 0),
                            Position::new(line_idx as u32, line.len() as u32),
                        ),
                    });
                }
            }
        }

        if let Some(rest) = strip_prefix_any(trimmed, &["let ", "let\t"]) {
            if let Some(name) = extract_name(rest) {
                if name == word {
                    return Some(Location {
                        uri: uri.clone(),
                        range: Range::new(
                            Position::new(line_idx as u32, 0),
                            Position::new(line_idx as u32, line.len() as u32),
                        ),
                    });
                }
            }
        }
    }

    None
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

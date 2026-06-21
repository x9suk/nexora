use lsp_types::*;

use crate::document::Document;

pub fn rename_symbol(doc: &Document, position: Position, new_name: &str) -> Option<WorkspaceEdit> {
    let word = get_word_at_position(doc, position)?;
    let uri = doc.uri.clone();
    let lines: Vec<&str> = doc.text.lines().collect();
    let mut text_edits = Vec::new();

    for (line_idx, line) in lines.iter().enumerate() {
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
                    text_edits.push(TextEdit {
                        range: Range::new(
                            Position::new(line_idx as u32, start as u32),
                            Position::new(line_idx as u32, col as u32),
                        ),
                        new_text: new_name.to_string(),
                    });
                }
            } else {
                col += 1;
            }
        }
    }

    if text_edits.is_empty() {
        return None;
    }

    let mut changes = std::collections::HashMap::new();
    changes.insert(uri, text_edits);

    Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    })
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

    while start > 0 && (bytes[start - 1] as char).is_alphanumeric() || (start > 0 && bytes[start - 1] == b'_') {
        start -= 1;
    }
    while end < bytes.len() && (bytes[end] as char).is_alphanumeric() || (end < bytes.len() && bytes[end] == b'_') {
        end += 1;
    }

    if start == end {
        return None;
    }

    Some(String::from_utf8_lossy(&bytes[start..end]).to_string())
}

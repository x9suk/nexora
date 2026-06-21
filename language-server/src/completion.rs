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

fn keyword_completions() -> Vec<CompletionItem> {
    let keywords = vec![
        "let", "func", "return", "if", "else", "while", "for", "class", "new",
        "this", "extends", "super", "try", "catch", "finally", "throw", "match",
        "async", "await", "import", "from", "assert", "test",
    ];
    keywords
        .into_iter()
        .map(|kw| CompletionItem {
            label: kw.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some(format!("Keyword: {}", kw)),
            documentation: Some(Documentation::String(format!("Nexora keyword: {}", kw))),
            ..Default::default()
        })
        .collect()
}

fn builtin_function_completions() -> Vec<CompletionItem> {
    let builtins = vec![
        ("print", "println(message)", "Print a message to stdout"),
        ("input", "input(prompt)", "Read user input"),
        ("type_of", "type_of(value)", "Get the type of a value"),
        ("str", "str(value)", "Convert value to string"),
        ("int", "int(value)", "Convert value to integer"),
        ("float", "float(value)", "Convert value to float"),
        ("len", "len(collection)", "Get length of a collection"),
        ("push", "push(collection, item)", "Add item to collection"),
        ("pop", "pop(collection)", "Remove last item from collection"),
        ("sort", "sort(collection)", "Sort a collection"),
        ("reverse", "reverse(collection)", "Reverse a collection"),
        ("unique", "unique(collection)", "Get unique items"),
        ("flatten", "flatten(collection)", "Flatten nested collections"),
        ("range", "range(start, end, step)", "Generate a range of numbers"),
        ("map", "map(collection, func)", "Apply function to each element"),
        ("filter", "filter(collection, func)", "Filter elements by predicate"),
        ("reduce", "reduce(collection, func)", "Reduce collection to single value"),
        ("sqrt", "sqrt(value)", "Square root"),
        ("pow", "pow(base, exponent)", "Power function"),
        ("abs", "abs(value)", "Absolute value"),
        ("floor", "floor(value)", "Floor function"),
        ("ceil", "ceil(value)", "Ceiling function"),
        ("round", "round(value)", "Round to nearest integer"),
        ("read_file", "read_file(path)", "Read file contents"),
        ("write_file", "write_file(path, content)", "Write content to file"),
        ("json_parse", "json_parse(json_string)", "Parse JSON string"),
        ("json_stringify", "json_stringify(value)", "Convert to JSON string"),
        ("http_get", "http_get(url)", "Perform HTTP GET request"),
        ("http_post", "http_post(url, body)", "Perform HTTP POST request"),
        ("now", "now()", "Get current timestamp"),
        ("timestamp", "timestamp()", "Get Unix timestamp"),
        ("env", "env(name)", "Get environment variable"),
        ("exec_command", "exec_command(command)", "Execute shell command"),
    ];
    builtins
        .into_iter()
        .map(|(name, sig, desc)| CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some(sig.to_string()),
            documentation: Some(Documentation::String(desc.to_string())),
            insert_text: Some(format!("{}(${{1:}})", name)),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
}

fn html_tag_completions() -> Vec<CompletionItem> {
    let tags = vec![
        "div", "h1", "h2", "h3", "h4", "h5", "h6", "p", "span", "a",
        "ul", "ol", "li", "table", "tr", "td", "button", "form",
        "input", "textarea", "select", "option", "img", "br", "hr",
        "header", "footer", "main", "section", "article", "nav",
    ];
    tags
        .into_iter()
        .map(|tag| CompletionItem {
            label: tag.to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some(format!("<{}>", tag)),
            documentation: Some(Documentation::String(format!(
                "HTML <{}> element",
                tag
            ))),
            insert_text: Some(format!("<{}>{{}}</{}>", tag, tag)),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
}

fn type_completions() -> Vec<CompletionItem> {
    let types = vec![
        ("int", "Integer type"),
        ("float", "Float type"),
        ("string", "String type"),
        ("bool", "Boolean type"),
        ("array", "Array type"),
        ("object", "Object type"),
        ("null", "Null type"),
    ];
    types
        .into_iter()
        .map(|(name, desc)| CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            detail: Some("Type".to_string()),
            documentation: Some(Documentation::String(desc.to_string())),
            ..Default::default()
        })
        .collect()
}

fn extract_symbols_from_document(doc: &Document) -> Vec<CompletionItem> {
    let mut symbols = Vec::new();
    let lines: Vec<&str> = doc.text.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if let Some(rest) = strip_prefix_any(trimmed, &["func ", "func\t"]) {
            if let Some(name) = extract_name(rest) {
                let detail = extract_func_signature(trimmed);
                symbols.push(CompletionItem {
                    label: name,
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some(detail),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                        range: Range::new(
                            Position::new(line_idx as u32, 0),
                            Position::new(line_idx as u32, line.len() as u32),
                        ),
                        new_text: String::new(),
                    })),
                    ..Default::default()
                });
            }
        }

        if let Some(rest) = strip_prefix_any(trimmed, &["class ", "class\t"]) {
            if let Some(name) = extract_name(rest) {
                symbols.push(CompletionItem {
                    label: name,
                    kind: Some(CompletionItemKind::CLASS),
                    detail: Some("Class".to_string()),
                    ..Default::default()
                });
            }
        }

        if let Some(rest) = strip_prefix_any(trimmed, &["let ", "let\t"]) {
            if let Some(name) = extract_name(rest) {
                symbols.push(CompletionItem {
                    label: name,
                    kind: Some(CompletionItemKind::VARIABLE),
                    detail: Some("Variable".to_string()),
                    ..Default::default()
                });
            }
        }
    }

    symbols
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

fn extract_func_signature(line: &str) -> String {
    if let Some(start) = line.find('(') {
        if let Some(end) = line.rfind(')') {
            return line[start..=end].to_string();
        }
    }
    "func".to_string()
}

pub fn provide_completions(doc: &Document, position: Position) -> CompletionList {
    let line = doc
        .text
        .lines()
        .nth(position.line as usize)
        .unwrap_or("");
    let char_pos = position.character as usize;
    let prefix = if char_pos <= line.len() {
        &line[..char_pos]
    } else {
        line
    };

    let mut items = Vec::new();

    if prefix.contains("<") || prefix.starts_with("<") {
        items.extend(html_tag_completions());
    } else if prefix.ends_with('.') {
        items.extend(method_completions());
    } else {
        items.extend(keyword_completions());
        items.extend(builtin_function_completions());
        items.extend(type_completions());
        items.extend(extract_symbols_from_document(doc));
    }

    CompletionList {
        is_incomplete: false,
        items,
    }
}

fn method_completions() -> Vec<CompletionItem> {
    let methods = vec![
        ("push", "push(item)", "Add item to array"),
        ("pop", "pop()", "Remove last item"),
        ("sort", "sort()", "Sort array"),
        ("reverse", "reverse()", "Reverse array"),
        ("map", "map(func)", "Map function over array"),
        ("filter", "filter(func)", "Filter array"),
        ("reduce", "reduce(func)", "Reduce array"),
        ("includes", "includes(item)", "Check if item exists"),
        ("indexOf", "indexOf(item)", "Find index of item"),
        ("slice", "slice(start, end)", "Get slice of array"),
        ("join", "join(separator)", "Join array to string"),
        ("split", "split(separator)", "Split string to array"),
        ("trim", "trim()", "Trim whitespace"),
        ("toLowerCase", "toLowerCase()", "Convert to lowercase"),
        ("toUpperCase", "toUpperCase()", "Convert to uppercase"),
    ];
    methods
        .into_iter()
        .map(|(name, sig, desc)| CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::METHOD),
            detail: Some(sig.to_string()),
            documentation: Some(Documentation::String(desc.to_string())),
            insert_text: Some(format!("{}(${{1:}})", name)),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
}

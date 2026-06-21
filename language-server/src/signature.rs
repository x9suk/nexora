use lsp_types::*;

use crate::document::Document;

pub fn provide_signature_help(doc: &Document, position: Position) -> Option<SignatureHelp> {
    let line = doc.text.lines().nth(position.line as usize)?;
    let char_pos = position.character as usize;

    let before_cursor = &line[..char_pos.min(line.len())];

    let func_name = extract_calling_function(before_cursor)?;

    let signature = get_function_signature(&func_name)?;

    let active_param = count_commas_before_cursor(before_cursor);

    Some(SignatureHelp {
        signatures: vec![SignatureInformation {
            label: signature.label.clone(),
            documentation: Some(lsp_types::Documentation::String(signature.documentation)),
            parameters: Some(
                signature
                    .parameters
                    .iter()
                    .map(|p| ParameterInformation {
                        label: lsp_types::ParameterLabel::Simple(p.label.clone()),
                        documentation: p.documentation.clone().map(lsp_types::Documentation::String),
                    })
                    .collect(),
            ),
            active_parameter: Some(active_param as u32),
        }],
        active_signature: Some(0),
        active_parameter: Some(active_param as u32),
    })
}

struct FuncSignature {
    label: String,
    documentation: String,
    parameters: Vec<ParamInfo>,
}

struct ParamInfo {
    label: String,
    documentation: Option<String>,
}

fn extract_calling_function(line: &str) -> Option<String> {
    let trimmed = line.trim_end();
    let mut depth = 0;
    let mut func_end = trimmed.len();

    for (i, c) in trimmed.char_indices().rev() {
        match c {
            ')' => depth += 1,
            '(' => {
                if depth == 0 {
                    func_end = i;
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }

    let before_paren = trimmed[..func_end].trim();
    let mut name_end = before_paren.len();
    for (i, c) in before_paren.char_indices().rev() {
        if c.is_alphanumeric() || c == '_' {
            name_end = i + 1;
        } else if i > 0 {
            break;
        } else {
            name_end = 0;
            break;
        }
    }

    let name = &before_paren[name_end..];
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn count_commas_before_cursor(line: &str) -> usize {
    let mut depth = 0;
    let mut count = 0;
    for c in line.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            ',' if depth == 1 => count += 1,
            _ => {}
        }
    }
    count
}

fn get_function_signature(name: &str) -> Option<FuncSignature> {
    match name {
        "print" => Some(FuncSignature {
            label: "print(message: any) -> void".to_string(),
            documentation: "Prints a message to stdout.".to_string(),
            parameters: vec![ParamInfo {
                label: "message: any".to_string(),
                documentation: Some("The value to print".to_string()),
            }],
        }),
        "input" => Some(FuncSignature {
            label: "input(prompt: string) -> string".to_string(),
            documentation: "Reads user input from stdin.".to_string(),
            parameters: vec![ParamInfo {
                label: "prompt: string".to_string(),
                documentation: Some("The prompt to display".to_string()),
            }],
        }),
        "len" => Some(FuncSignature {
            label: "len(collection: any) -> int".to_string(),
            documentation: "Returns the length of a collection.".to_string(),
            parameters: vec![ParamInfo {
                label: "collection: any".to_string(),
                documentation: Some("The collection to measure".to_string()),
            }],
        }),
        "push" => Some(FuncSignature {
            label: "push(collection: array, item: any) -> void".to_string(),
            documentation: "Adds an item to the end of an array.".to_string(),
            parameters: vec![
                ParamInfo {
                    label: "collection: array".to_string(),
                    documentation: Some("The target array".to_string()),
                },
                ParamInfo {
                    label: "item: any".to_string(),
                    documentation: Some("The item to add".to_string()),
                },
            ],
        }),
        "map" => Some(FuncSignature {
            label: "map(collection: array, func: function) -> array".to_string(),
            documentation: "Applies a function to each element.".to_string(),
            parameters: vec![
                ParamInfo {
                    label: "collection: array".to_string(),
                    documentation: Some("The source array".to_string()),
                },
                ParamInfo {
                    label: "func: function".to_string(),
                    documentation: Some("The function to apply".to_string()),
                },
            ],
        }),
        "filter" => Some(FuncSignature {
            label: "filter(collection: array, func: function) -> array".to_string(),
            documentation: "Filters elements by a predicate.".to_string(),
            parameters: vec![
                ParamInfo {
                    label: "collection: array".to_string(),
                    documentation: Some("The source array".to_string()),
                },
                ParamInfo {
                    label: "func: function".to_string(),
                    documentation: Some("The predicate function".to_string()),
                },
            ],
        }),
        "reduce" => Some(FuncSignature {
            label: "reduce(collection: array, func: function, initial: any) -> any".to_string(),
            documentation: "Reduces a collection to a single value.".to_string(),
            parameters: vec![
                ParamInfo {
                    label: "collection: array".to_string(),
                    documentation: Some("The source array".to_string()),
                },
                ParamInfo {
                    label: "func: function".to_string(),
                    documentation: Some("The reducer function".to_string()),
                },
                ParamInfo {
                    label: "initial: any".to_string(),
                    documentation: Some("The initial value".to_string()),
                },
            ],
        }),
        "range" => Some(FuncSignature {
            label: "range(start: int, end: int, step: int) -> array".to_string(),
            documentation: "Generates a range of numbers.".to_string(),
            parameters: vec![
                ParamInfo {
                    label: "start: int".to_string(),
                    documentation: Some("The start value (inclusive)".to_string()),
                },
                ParamInfo {
                    label: "end: int".to_string(),
                    documentation: Some("The end value (exclusive)".to_string()),
                },
                ParamInfo {
                    label: "step: int".to_string(),
                    documentation: Some("The step value".to_string()),
                },
            ],
        }),
        "read_file" => Some(FuncSignature {
            label: "read_file(path: string) -> string".to_string(),
            documentation: "Reads the contents of a file.".to_string(),
            parameters: vec![ParamInfo {
                label: "path: string".to_string(),
                documentation: Some("The file path".to_string()),
            }],
        }),
        "write_file" => Some(FuncSignature {
            label: "write_file(path: string, content: string) -> void".to_string(),
            documentation: "Writes content to a file.".to_string(),
            parameters: vec![
                ParamInfo {
                    label: "path: string".to_string(),
                    documentation: Some("The file path".to_string()),
                },
                ParamInfo {
                    label: "content: string".to_string(),
                    documentation: Some("The content to write".to_string()),
                },
            ],
        }),
        "http_get" => Some(FuncSignature {
            label: "http_get(url: string) -> response".to_string(),
            documentation: "Performs an HTTP GET request.".to_string(),
            parameters: vec![ParamInfo {
                label: "url: string".to_string(),
                documentation: Some("The URL to request".to_string()),
            }],
        }),
        "http_post" => Some(FuncSignature {
            label: "http_post(url: string, body: any) -> response".to_string(),
            documentation: "Performs an HTTP POST request.".to_string(),
            parameters: vec![
                ParamInfo {
                    label: "url: string".to_string(),
                    documentation: Some("The URL to post to".to_string()),
                },
                ParamInfo {
                    label: "body: any".to_string(),
                    documentation: Some("The request body".to_string()),
                },
            ],
        }),
        _ => None,
    }
}

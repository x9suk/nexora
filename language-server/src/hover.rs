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

pub fn provide_hover(doc: &Document, position: Position) -> Option<Hover> {
    let word = get_word_at_position(doc, position)?;
    let word_str = word.as_str();

    if let Some(info) = get_keyword_info(word_str) {
        return Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(info)),
            range: Some(Range::new(
                position,
                Position::new(position.line, position.character + word.len() as u32),
            )),
        });
    }

    if let Some(info) = get_builtin_info(word_str) {
        return Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(info)),
            range: Some(Range::new(
                position,
                Position::new(position.line, position.character + word.len() as u32),
            )),
        });
    }

    if let Some(info) = get_symbol_info(doc, word_str) {
        return Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(info)),
            range: Some(Range::new(
                position,
                Position::new(position.line, position.character + word.len() as u32),
            )),
        });
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

fn get_keyword_info(word: &str) -> Option<String> {
    match word {
        "let" => Some("let - Variable declaration\n\n```nexora\nlet name = value;\n```\n\nDeclares a new variable in the current scope.".to_string()),
        "func" => Some("func - Function declaration\n\n```nexora\nfunc name(params) {\n    // body\n}\n```\n\nDeclares a new function.".to_string()),
        "return" => Some("return - Return statement\n\n```nexora\nreturn value;\n```\n\nReturns a value from a function.".to_string()),
        "if" => Some("if - Conditional statement\n\n```nexora\nif (condition) {\n    // true branch\n} else {\n    // false branch\n}\n```".to_string()),
        "else" => Some("else - Alternative branch\n\nUsed with if to provide an alternative execution path.".to_string()),
        "while" => Some("while - While loop\n\n```nexora\nwhile (condition) {\n    // loop body\n}\n```".to_string()),
        "for" => Some("for - For loop\n\n```nexora\nfor (let i = 0; i < 10; i = i + 1) {\n    // loop body\n}\n```".to_string()),
        "class" => Some("class - Class declaration\n\n```nexora\nclass MyClass extends Parent {\n    let field = value;\n\n    func method() {\n        // method body\n    }\n}\n```".to_string()),
        "new" => Some("new - Create instance\n\n```nexora\nlet obj = new MyClass();\n```\n\nCreates a new instance of a class.".to_string()),
        "this" => Some("this - Current instance\n\nRefers to the current object instance inside methods.".to_string()),
        "extends" => Some("extends - Inheritance\n\n```nexora\nclass Child extends Parent {\n}\n```\n\nCreates a class that inherits from another.".to_string()),
        "super" => Some("super - Parent reference\n\nRefers to the parent class. Used to call parent methods or constructor.".to_string()),
        "try" => Some("try - Try block\n\n```nexora\ntry {\n    // risky code\n} catch (e) {\n    // handle error\n} finally {\n    // cleanup\n}\n```".to_string()),
        "catch" => Some("catch - Catch block\n\nHandles exceptions thrown in a try block.".to_string()),
        "finally" => Some("finally - Finally block\n\nAlways executes after try/catch, regardless of outcome.".to_string()),
        "throw" => Some("throw - Throw exception\n\n```nexora\nthrow new Error(\"message\");\n```\n\nThrows an exception.".to_string()),
        "match" => Some("match - Pattern matching\n\n```nexora\nmatch (value) {\n    pattern1 => result1,\n    pattern2 => result2,\n    _ => default\n}\n```".to_string()),
        "async" => Some("async - Async function\n\n```nexora\nasync func fetchData() {\n    let data = await http_get(url);\n    return data;\n}\n```".to_string()),
        "await" => Some("await - Await promise\n\nWaits for an async operation to complete.".to_string()),
        "import" => Some("import - Import module\n\n```nexora\nimport { func } from \"module\";\n```".to_string()),
        "from" => Some("from - Import source\n\nSpecifies the source module in an import statement.".to_string()),
        "assert" => Some("assert - Assertion\n\n```nexora\nassert(condition, \"error message\");\n```\n\nAsserts a condition is true.".to_string()),
        "test" => Some("test - Test block\n\n```nexora\ntest(\"should work\") {\n    assert(result == expected);\n}\n```".to_string()),
        _ => None,
    }
}

fn get_builtin_info(word: &str) -> Option<String> {
    match word {
        "print" => Some("print(message) -> void\n\nPrints a message to stdout.\n\n```nexora\nprint(\"Hello, World!\");\n```".to_string()),
        "input" => Some("input(prompt) -> string\n\nReads user input from stdin.\n\n```nexora\nlet name = input(\"Enter your name: \");\n```".to_string()),
        "type_of" => Some("type_of(value) -> string\n\nReturns the type of a value as a string.\n\n```nexora\nlet t = type_of(42); // \"int\"\n```".to_string()),
        "str" => Some("str(value) -> string\n\nConverts a value to a string.\n\n```nexora\nlet s = str(42); // \"42\"\n```".to_string()),
        "int" => Some("int(value) -> int\n\nConverts a value to an integer.\n\n```nexora\nlet n = int(\"42\"); // 42\n```".to_string()),
        "float" => Some("float(value) -> float\n\nConverts a value to a float.\n\n```nexora\nlet f = float(\"3.14\"); // 3.14\n```".to_string()),
        "len" => Some("len(collection) -> int\n\nReturns the length of a collection.\n\n```nexora\nlet n = len([1, 2, 3]); // 3\n```".to_string()),
        "push" => Some("push(collection, item) -> void\n\nAdds an item to the end of an array.\n\n```nexora\nlet arr = [1, 2];\npush(arr, 3); // [1, 2, 3]\n```".to_string()),
        "pop" => Some("pop(collection) -> item\n\nRemoves and returns the last item.\n\n```nexora\nlet arr = [1, 2, 3];\nlet last = pop(arr); // 3\n```".to_string()),
        "sort" => Some("sort(collection) -> collection\n\nReturns a sorted copy of the collection.\n\n```nexora\nlet sorted = sort([3, 1, 2]); // [1, 2, 3]\n```".to_string()),
        "reverse" => Some("reverse(collection) -> collection\n\nReturns a reversed copy of the collection.\n\n```nexora\nlet rev = reverse([1, 2, 3]); // [3, 2, 1]\n```".to_string()),
        "unique" => Some("unique(collection) -> collection\n\nReturns unique items from the collection.".to_string()),
        "flatten" => Some("flatten(collection) -> collection\n\nFlattens a nested array one level.".to_string()),
        "range" => Some("range(start, end, step) -> array\n\nGenerates a range of numbers.\n\n```nexora\nlet nums = range(0, 10, 2); // [0, 2, 4, 6, 8]\n```".to_string()),
        "map" => Some("map(collection, func) -> collection\n\nApplies a function to each element.\n\n```nexora\nlet doubled = map([1, 2, 3], func(x) { return x * 2; });\n```".to_string()),
        "filter" => Some("filter(collection, func) -> collection\n\nFilters elements by a predicate.\n\n```nexora\nlet evens = filter([1, 2, 3, 4], func(x) { return x % 2 == 0; });\n```".to_string()),
        "reduce" => Some("reduce(collection, func) -> value\n\nReduces a collection to a single value.\n\n```nexora\nlet sum = reduce([1, 2, 3], func(acc, x) { return acc + x; }, 0);\n```".to_string()),
        "sqrt" => Some("sqrt(value) -> float\n\nReturns the square root.\n\n```nexora\nlet s = sqrt(16.0); // 4.0\n```".to_string()),
        "pow" => Some("pow(base, exponent) -> float\n\nReturns base raised to the power.\n\n```nexora\nlet p = pow(2.0, 10.0); // 1024.0\n```".to_string()),
        "abs" => Some("abs(value) -> number\n\nReturns the absolute value.".to_string()),
        "floor" => Some("floor(value) -> int\n\nRounds down to the nearest integer.".to_string()),
        "ceil" => Some("ceil(value) -> int\n\nRounds up to the nearest integer.".to_string()),
        "round" => Some("round(value) -> int\n\nRounds to the nearest integer.".to_string()),
        "read_file" => Some("read_file(path) -> string\n\nReads the contents of a file.\n\n```nexora\nlet content = read_file(\"data.txt\");\n```".to_string()),
        "write_file" => Some("write_file(path, content) -> void\n\nWrites content to a file.".to_string()),
        "json_parse" => Some("json_parse(json_string) -> value\n\nParses a JSON string into a value.".to_string()),
        "json_stringify" => Some("json_stringify(value) -> string\n\nConverts a value to a JSON string.".to_string()),
        "http_get" => Some("http_get(url) -> response\n\nPerforms an HTTP GET request.\n\n```nexora\nlet response = http_get(\"https://api.example.com/data\");\n```".to_string()),
        "http_post" => Some("http_post(url, body) -> response\n\nPerforms an HTTP POST request.".to_string()),
        "now" => Some("now() -> timestamp\n\nReturns the current date and time.".to_string()),
        "timestamp" => Some("timestamp() -> int\n\nReturns the current Unix timestamp.".to_string()),
        "env" => Some("env(name) -> string\n\nGets an environment variable.".to_string()),
        "exec_command" => Some("exec_command(command) -> string\n\nExecutes a shell command and returns output.".to_string()),
        _ => None,
    }
}

fn get_symbol_info(doc: &Document, word: &str) -> Option<String> {
    let lines: Vec<&str> = doc.text.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if let Some(rest) = strip_prefix_any(trimmed, &["func ", "func\t"]) {
            if let Some(name) = extract_name(rest) {
                if name == word {
                    let signature = extract_func_signature(trimmed);
                    return Some(format!(
                        "```nexora\n{}\n```\n\nFunction defined at line {}",
                        signature,
                        line_idx + 1
                    ));
                }
            }
        }

        if let Some(rest) = strip_prefix_any(trimmed, &["class ", "class\t"]) {
            if let Some(name) = extract_name(rest) {
                if name == word {
                    return Some(format!(
                        "```nexora\nclass {}\n```\n\nClass defined at line {}",
                        name,
                        line_idx + 1
                    ));
                }
            }
        }

        if let Some(rest) = strip_prefix_any(trimmed, &["let ", "let\t"]) {
            if let Some(name) = extract_name(rest) {
                if name == word {
                    let type_info = extract_var_type(trimmed);
                    return Some(format!(
                        "```nexora\nlet {}{}\n```\n\nVariable defined at line {}",
                        name,
                        type_info,
                        line_idx + 1
                    ));
                }
            }
        }
    }

    None
}

fn extract_func_signature(line: &str) -> String {
    if let Some(_start) = line.find('(') {
        if let Some(end) = line.rfind(')') {
            return line[..=end].to_string();
        }
    }
    line.to_string()
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

fn extract_var_type(line: &str) -> String {
    if let Some(_eq_pos) = line.find('=') {
        if let Some(colon_pos) = line.find(':') {
            let eq_pos = line.find('=').unwrap_or(line.len());
            if colon_pos < eq_pos {
                return format!(": {}", line[colon_pos + 1..eq_pos].trim());
            }
        }
    }
    String::new()
}

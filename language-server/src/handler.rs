use lsp_server::Connection;
use lsp_types::*;
use serde_json::Value;

use crate::completion::provide_completions;
use crate::diagnostics::validate_document;
use crate::document::DocumentStore;
use crate::goto::find_definition;
use crate::hover::provide_hover;
use crate::references::find_references;
use crate::rename::rename_symbol;
use crate::signature::provide_signature_help;
use crate::symbols::extract_document_symbols;

pub fn handle_completion(store: &DocumentStore, params: Value) -> Value {
    let params: CompletionParams = serde_json::from_value(params).unwrap();
    let uri = &params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    if let Some(doc) = store.get_document(uri) {
        let completions = provide_completions(doc, position);
        serde_json::to_value(&completions).unwrap()
    } else {
        serde_json::to_value(&CompletionList {
            is_incomplete: false,
            items: vec![],
        })
        .unwrap()
    }
}

pub fn handle_hover(store: &DocumentStore, params: Value) -> Value {
    let params: HoverParams = serde_json::from_value(params).unwrap();
    let uri = &params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    if let Some(doc) = store.get_document(uri) {
        if let Some(hover) = provide_hover(doc, position) {
            serde_json::to_value(&hover).unwrap()
        } else {
            Value::Null
        }
    } else {
        Value::Null
    }
}

pub fn handle_goto_definition(store: &DocumentStore, params: Value) -> Value {
    let params: GotoDefinitionParams = serde_json::from_value(params).unwrap();
    let uri = &params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    if let Some(doc) = store.get_document(uri) {
        if let Some(location) = find_definition(doc, position) {
            serde_json::to_value(&location).unwrap()
        } else {
            Value::Null
        }
    } else {
        Value::Null
    }
}

pub fn handle_references(store: &DocumentStore, params: Value) -> Value {
    let params: ReferenceParams = serde_json::from_value(params).unwrap();
    let uri = &params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    if let Some(doc) = store.get_document(uri) {
        let references = find_references(doc, position, params.context.include_declaration);
        serde_json::to_value(&references).unwrap()
    } else {
        serde_json::to_value(&Vec::<Location>::new()).unwrap()
    }
}

pub fn handle_rename(store: &DocumentStore, params: Value) -> Value {
    let params: RenameParams = serde_json::from_value(params).unwrap();
    let uri = &params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let new_name = params.new_name;

    if let Some(doc) = store.get_document(uri) {
        if let Some(workspace_edit) = rename_symbol(doc, position, &new_name) {
            serde_json::to_value(&workspace_edit).unwrap()
        } else {
            Value::Null
        }
    } else {
        Value::Null
    }
}

pub fn handle_signature_help(store: &DocumentStore, params: Value) -> Value {
    let params: SignatureHelpParams = serde_json::from_value(params).unwrap();
    let uri = &params.text_document_position_params.text_document.uri;
    let position = params.text_document_position_params.position;

    if let Some(doc) = store.get_document(uri) {
        if let Some(sig_help) = provide_signature_help(doc, position) {
            serde_json::to_value(&sig_help).unwrap()
        } else {
            Value::Null
        }
    } else {
        Value::Null
    }
}

pub fn handle_document_symbols(store: &DocumentStore, params: Value) -> Value {
    let params: DocumentSymbolParams = serde_json::from_value(params).unwrap();
    let uri = &params.text_document.uri;

    if let Some(doc) = store.get_document(uri) {
        let symbols = extract_document_symbols(doc);
        serde_json::to_value(&symbols).unwrap()
    } else {
        serde_json::to_value(&Vec::<DocumentSymbol>::new()).unwrap()
    }
}

pub fn publish_diagnostics(connection: &Connection, store: &DocumentStore) {
    for (uri, doc) in store.documents() {
        let diagnostics = validate_document(doc);
        let params = PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics,
            version: Some(doc.version),
        };
        let notification = lsp_server::Notification::new(
            "textDocument/publishDiagnostics".to_string(),
            params,
        );
        connection
            .sender
            .send(lsp_server::Message::Notification(notification))
            .unwrap();
    }
}

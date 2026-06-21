use lsp_server::{Connection, Message};

use crate::document::DocumentStore;
use crate::handler;

pub fn run() {
    let (connection, _threads) = Connection::stdio();
    let (initialize_id, _initialize_params) = connection.initialize_start().unwrap();
    let initialize_data = serde_json::json!({
        "capabilities": {
            "textDocumentSync": {
                "openClose": true,
                "change": 1,
                "save": { "includeText": true }
            },
            "completionProvider": {
                "triggerCharacters": ["."],
                "resolveProvider": false
            },
            "hoverProvider": true,
            "definitionProvider": true,
            "referencesProvider": true,
            "renameProvider": true,
            "signatureHelpProvider": {
                "triggerCharacters": ["("]
            },
            "documentSymbolProvider": true
        }
    });
    connection
        .initialize_finish(initialize_id, initialize_data)
        .unwrap();

    let mut document_store = DocumentStore::new();

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req).unwrap() {
                    return;
                }
                handle_request(&connection, &mut document_store, req);
            }
            Message::Response(_resp) => {}
            Message::Notification(not) => {
                handle_notification(&connection, &mut document_store, not);
            }
        }
    }
}

fn handle_request(
    connection: &Connection,
    store: &mut DocumentStore,
    req: lsp_server::Request,
) {
    match req.method.as_str() {
        "textDocument/completion" => {
            let result = handler::handle_completion(store, req.params.clone());
            let resp = lsp_server::Response::new_ok(req.id, result);
            connection.sender.send(Message::Response(resp)).unwrap();
        }
        "textDocument/hover" => {
            let result = handler::handle_hover(store, req.params.clone());
            let resp = lsp_server::Response::new_ok(req.id, result);
            connection.sender.send(Message::Response(resp)).unwrap();
        }
        "textDocument/definition" => {
            let result = handler::handle_goto_definition(store, req.params.clone());
            let resp = lsp_server::Response::new_ok(req.id, result);
            connection.sender.send(Message::Response(resp)).unwrap();
        }
        "textDocument/references" => {
            let result = handler::handle_references(store, req.params.clone());
            let resp = lsp_server::Response::new_ok(req.id, result);
            connection.sender.send(Message::Response(resp)).unwrap();
        }
        "textDocument/rename" => {
            let result = handler::handle_rename(store, req.params.clone());
            let resp = lsp_server::Response::new_ok(req.id, result);
            connection.sender.send(Message::Response(resp)).unwrap();
        }
        "textDocument/signatureHelp" => {
            let result = handler::handle_signature_help(store, req.params.clone());
            let resp = lsp_server::Response::new_ok(req.id, result);
            connection.sender.send(Message::Response(resp)).unwrap();
        }
        "textDocument/documentSymbol" => {
            let result = handler::handle_document_symbols(store, req.params.clone());
            let resp = lsp_server::Response::new_ok(req.id, result);
            connection.sender.send(Message::Response(resp)).unwrap();
        }
        _ => {}
    }
}

fn handle_notification(
    connection: &Connection,
    store: &mut DocumentStore,
    not: lsp_server::Notification,
) {
    match not.method.as_str() {
        "textDocument/didOpen" => {
            let params: lsp_types::DidOpenTextDocumentParams =
                serde_json::from_value(not.params).unwrap();
            store.open_document(params);
        }
        "textDocument/didChange" => {
            let params: lsp_types::DidChangeTextDocumentParams =
                serde_json::from_value(not.params).unwrap();
            store.change_document(params);
        }
        "textDocument/didSave" => {
            let params: lsp_types::DidSaveTextDocumentParams =
                serde_json::from_value(not.params).unwrap();
            store.save_document(params);
            handler::publish_diagnostics(connection, store);
        }
        _ => {}
    }
}

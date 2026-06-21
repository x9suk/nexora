use lsp_types::*;
use std::collections::HashMap;

pub struct Document {
    pub uri: Url,
    pub text: String,
    pub version: i32,
}

pub struct DocumentStore {
    documents: HashMap<Url, Document>,
}

impl DocumentStore {
    pub fn new() -> Self {
        DocumentStore {
            documents: HashMap::new(),
        }
    }

    pub fn open_document(&mut self, params: DidOpenTextDocumentParams) {
        let doc = Document {
            uri: params.text_document.uri.clone(),
            text: params.text_document.text,
            version: params.text_document.version,
        };
        self.documents.insert(params.text_document.uri, doc);
    }

    pub fn change_document(&mut self, params: DidChangeTextDocumentParams) {
        if let Some(doc) = self.documents.get_mut(&params.text_document.uri) {
            for change in params.content_changes {
                doc.text = change.text;
            }
            doc.version = params.text_document.version;
        }
    }

    pub fn save_document(&self, _params: DidSaveTextDocumentParams) {
        // Document is already in memory; diagnostics are published on save.
    }

    pub fn get_document(&self, uri: &Url) -> Option<&Document> {
        self.documents.get(uri)
    }

    pub fn documents(&self) -> &HashMap<Url, Document> {
        &self.documents
    }
}

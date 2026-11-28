use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use tower_lsp_server::ls_types::{Range, TextDocumentContentChangeEvent, Uri};

pub struct Document {
    pub content: String,
    version: i32,
}

impl Document {
    fn resolve_range(&self, range: Range) -> (usize, usize) {
        let mut start_found = false;
        let mut end_found = false;
        let mut start_idx = 0;
        let mut end_idx = 0;
        let mut current_line = 0;
        let mut current_line_idx = 0;
        for (i, char) in self.content.char_indices() {
            if current_line == range.start.line {
                start_idx = current_line_idx + range.start.character as usize;
                start_found = true;
            }
            if current_line == range.end.line {
                end_idx = current_line_idx + range.end.character as usize;
                end_found = true;
            }
            if start_found && end_found {
                break;
            }

            if char == '\n' {
                current_line += 1;
                current_line_idx = i + 1;
            }
        }

        if start_idx == 0 && current_line == range.start.line {
            let idx = self.content.len();
            return (idx, idx);
        }
        if end_idx == 0 && current_line == range.end.line {
            let idx = self.content.len();
            return (start_idx, idx);
        }
        (start_idx, end_idx)
    }
}

pub struct Documents {
    documents: Arc<RwLock<HashMap<Uri, Document>>>,
}

impl Documents {
    pub async fn add(&self, uri: Uri, content: String, version: i32) {
        let mut documents = self.documents.write().await;
        documents.insert(uri, Document { content, version });
    }

    pub async fn update_partially(
        &self,
        uri: &Uri,
        version: i32,
        changes: Vec<TextDocumentContentChangeEvent>,
    ) {
        let mut documents = self.documents.write().await;
        if let Some(document) = documents.get_mut(uri) {
            document.version = version;
            for change in changes {
                if let Some(range) = change.range {
                    let range = document.resolve_range(range);
                    document
                        .content
                        .replace_range(range.0..range.1, change.text.as_str());
                }
            }
        }
    }
    
    pub async fn remove(&self, uri: &Uri) {
        let mut documents = self.documents.write().await;
        documents.remove(uri);
    }
}

impl Default for Documents {
    fn default() -> Self {
        Documents {
            documents: Arc::new(RwLock::new(HashMap::with_capacity(10))),
        }
    }
}

pub static DOCUMENTS: LazyLock<Documents> = LazyLock::new(|| Documents::default());

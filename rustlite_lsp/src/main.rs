use std::sync::Arc;
use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    documents: Arc<DashMap<Url, String>>, // Store opened document content
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".", "("].iter().map(|s| s.to_string()).collect()),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                ..Default::default()
            },
            server_info: None,
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "RustLite-LSP initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.documents.insert(params.text_document.uri, params.text_document.text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(content) = params.content_changes.into_iter().last() {
            self.documents.insert(params.text_document.uri, content.text);
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let pos = params.text_document_position_params.position;
        let uri = params.text_document_position_params.text_document.uri;
        let location = Location {
            uri,
            range: Range {
                start: pos,
                end: pos,
            },
        };
        Ok(Some(GotoDefinitionResponse::Scalar(location)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;

        // Get document text from stored cache
        let document_text = self.documents.get(uri).map(|v| v.clone()).unwrap_or_default();
        
        // Extract the word under the cursor
        let word = get_word_at_position(&document_text, pos);

        // Define hover responses for specific words
        let hover_text = match word.as_str() {
            "fn" => "Defines a function in RustLite",
            "let" => "Declares a variable in RustLite",
            "for" => "For loop construct",
            "while" => "While loop construct",
            _ => "RustLite Syntax Support",
        };

        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(hover_text.into())),
            range: None,
        }))
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let items = vec![
            CompletionItem { label: "fn".into(), kind: Some(CompletionItemKind::FUNCTION), detail: Some("Defines a function".into()), ..Default::default() },
            CompletionItem { label: "let".into(), kind: Some(CompletionItemKind::VARIABLE), detail: Some("Declares a variable".into()), ..Default::default() },
            CompletionItem { label: "for".into(), kind: Some(CompletionItemKind::KEYWORD), detail: Some("For loop construct".into()), ..Default::default() },
            CompletionItem { label: "while".into(), kind: Some(CompletionItemKind::KEYWORD), detail: Some("While loop construct".into()), ..Default::default() },
        ];
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let pos = params.text_document_position.position;
        let uri = params.text_document_position.text_document.uri;
        let location = Location {
            uri,
            range: Range {
                start: pos,
                end: pos,
            },
        };
        Ok(Some(vec![location]))
    }
}

/// Extracts the word at a given position in a document
fn get_word_at_position(text: &str, position: Position) -> String {
    let line = text.lines().nth(position.line as usize).unwrap_or_default();
    let words: Vec<&str> = line.split_whitespace().collect();
    
    words.get(position.character as usize / 2).unwrap_or(&"").to_string()
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        documents: Arc::new(DashMap::new()),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

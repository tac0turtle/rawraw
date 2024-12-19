use crate::db::FileSource;
use crate::frontend::parser;
use crate::frontend::syntax::{SyntaxKind, SyntaxNode};
use crate::lsp_server::line_col::{build_line_index, to_lsp_position, to_lsp_range};
use crate::lsp_server::server::LSPServer;
use line_index::LineIndex;
use rowan::NodeOrToken::Token;
use salsa::Database;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    MessageType, SemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensParams,
    SemanticTokensResult,
};

impl LSPServer {
    pub async fn on_semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        if let Some(src) = self.document_map.get(&params.text_document.uri) {
            let db = self.db.lock().unwrap();
            let data = semantic_tokens_from_ast(&*db, *src);
            Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                result_id: None,
                data,
            })))
        } else {
            Ok(None)
        }
    }
}

pub const LEGEND_TYPE: &[SemanticTokenType] = &[
    SemanticTokenType::FUNCTION,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::STRING,
    SemanticTokenType::COMMENT,
    SemanticTokenType::NUMBER,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::PARAMETER,
];

#[salsa::tracked]
pub fn semantic_tokens_from_ast(db: &dyn Database, src: FileSource) -> Vec<SemanticToken> {
    let ast = parser::parse(&*db, src);
    let node = ast.root(&*db);
    let root = SyntaxNode::new_root(node.clone());
    let line_index = build_line_index(db, src);
    let mut semantic_tokens = vec![];
    let mut last_line = 0;
    root.descendants_with_tokens().for_each(|node| {
        if let Token(token) = node {
            let kind = token.kind();
            let range = token.text_range();
            let len = range.len();
            let pos = to_lsp_position(line_index, range.start());
            let line = pos.line;
            let delta_line = line - last_line;
            match kind {
                SyntaxKind::IDENT => {
                    semantic_tokens.push(SemanticToken {
                        delta_line,
                        delta_start: pos.character,
                        length: len.into(),
                        token_type: 1,
                        token_modifiers_bitset: 0,
                    });
                    last_line = line;
                }
                _ => {}
            }
        }
    });
    semantic_tokens
}

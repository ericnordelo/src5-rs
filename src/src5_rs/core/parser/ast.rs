use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_diagnostics::DiagnosticsBuilder;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::{FileLongId, VirtualFile};
use cairo_lang_parser::parser::Parser;
use cairo_lang_starknet::plugin::StarkNetPlugin;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use std::sync::Arc;

pub fn get_database_with_starknet_plugin() -> RootDatabase {
    RootDatabase::builder()
        .with_semantic_plugin(Arc::new(StarkNetPlugin::default()))
        .build()
        .unwrap()
}

/// Get the AST from cairo code represented as a string
pub fn get_syntax_tree(db: &RootDatabase, content: String) -> SyntaxNode {
    let virtual_file = db.intern_file(FileLongId::Virtual(VirtualFile {
        parent: None,
        name: "string_to_parse".into(),
        content: Arc::new(content.clone()),
    }));
    let mut diagnostics = DiagnosticsBuilder::new();
    Parser::parse_file(db, &mut diagnostics, virtual_file, content.as_str()).as_syntax_node()
}

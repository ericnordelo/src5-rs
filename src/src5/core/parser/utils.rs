use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;

pub fn find_children(db: &RootDatabase, node: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxNode> {
    for child in node.children(db) {
        if kind == child.kind(db) {
            return Some(child);
        }
    }
    None
}

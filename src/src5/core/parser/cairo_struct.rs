// Module for handling Cairo structs
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;

use super::ast::get_syntax_tree;
use super::utils::find_children;

#[derive(Debug)]
pub struct CairoStruct {
    pub name: String,
    pub generics: Vec<String>,
    pub members_types: Vec<SyntaxNode>,
}

pub fn get_corelib_structs(db: &RootDatabase) -> Vec<CairoStruct> {
    // Get the syntax tree
    let tree = get_syntax_tree(db, CORELIB_STRUCTS.into());

    get_cairo_structs_no_corelib(db, &tree)
}

pub fn get_cairo_structs(db: &RootDatabase, syntax_tree: &SyntaxNode) -> Vec<CairoStruct> {
    let mut cairo_structs = get_cairo_structs_no_corelib(db, syntax_tree);
    // Include corelib structs
    cairo_structs.extend(get_corelib_structs(db));
    cairo_structs
}

pub fn get_cairo_structs_no_corelib(db: &RootDatabase, syntax_tree: &SyntaxNode) -> Vec<CairoStruct> {
    let mut cairo_structs = Vec::new();
    for node in syntax_tree.descendants(db) {
        if SyntaxKind::ItemStruct == node.kind(db) {
            // Look up the Struct name
            let id_node = find_children(db, &node, SyntaxKind::TerminalIdentifier).unwrap();
            let struct_name = id_node.get_text_without_trivia(db);
            let mut struct_members_types = Vec::new();
            let mut struct_generics = Vec::new();

            // Look up the Struct members types
            let members_node = find_children(db, &node, SyntaxKind::MemberList).unwrap();
            for node in members_node.descendants(db) {
                if node.kind(db) == SyntaxKind::TypeClause {
                    struct_members_types.push(node);
                }
            }
            // Look up the Struct generics
            if let Some(child) = find_children(db, &node, SyntaxKind::WrappedGenericParamList) {
                for node in child.descendants(db) {
                    if node.kind(db) == SyntaxKind::GenericParamType {
                        let generic_type = node.get_text_without_trivia(db);
                        struct_generics.push(generic_type);
                    }
                }
            }
            cairo_structs.push(CairoStruct {
                name: struct_name,
                generics: struct_generics,
                members_types: struct_members_types,
            });
        }
    }
    cairo_structs
}

const CORELIB_STRUCTS: &str = "
struct Span<T> {
   snapshot: @Array<T>
}
";

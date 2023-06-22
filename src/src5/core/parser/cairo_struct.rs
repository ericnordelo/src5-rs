// Module for handling cairo Structs
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;

use super::utils::find_children;

#[derive(Debug)]
pub struct CairoStruct {
    pub name: String,
    pub generics: Vec<String>,
    pub members_types: Vec<String>,
}

pub fn get_corelib_structs() -> [CairoStruct; 1] {
    [CairoStruct {
        name: "Span".into(),
        generics: vec!["T".into()],
        members_types: vec!["@Array<T>".into()],
    }]
}

pub fn get_cairo_structs(db: &RootDatabase, syntax_tree: &SyntaxNode) -> Vec<CairoStruct> {
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
                    let expr_path_node = find_children(db, &node, SyntaxKind::ExprPath).unwrap();
                    let member_type = expr_path_node.get_text_without_trivia(db);
                    struct_members_types.push(member_type);
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
    // Include corelib structs
    cairo_structs.extend(get_corelib_structs());
    cairo_structs
}

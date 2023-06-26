// Module for handling Cairo enums
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;

use super::ast::get_syntax_tree;
use super::utils::find_children;

#[derive(Debug)]
pub struct CairoEnum {
    pub name: String,
    pub generics: Vec<String>,
    pub variants_types: Vec<SyntaxNode>,
}

pub fn get_corelib_enums(db: &RootDatabase) -> Vec<CairoEnum> {
    // Get the syntax tree
    let tree = get_syntax_tree(db, CORELIB_ENUMS.into());

    get_cairo_enums_no_corelib(db, &tree)
}

pub fn get_cairo_enums(db: &RootDatabase, syntax_tree: &SyntaxNode) -> Vec<CairoEnum> {
    let mut cairo_enums = get_cairo_enums_no_corelib(db, syntax_tree);
    // Include corelib structs
    cairo_enums.extend(get_corelib_enums(db));
    cairo_enums
}

fn get_cairo_enums_no_corelib(db: &RootDatabase, syntax_tree: &SyntaxNode) -> Vec<CairoEnum> {
    let mut cairo_enums = Vec::new();
    for node in syntax_tree.descendants(db) {
        if SyntaxKind::ItemEnum == node.kind(db) {
            // Look up the Enum name
            let id_node = find_children(db, &node, SyntaxKind::TerminalIdentifier).unwrap();
            let enum_name = id_node.get_text_without_trivia(db);

            let mut struct_members_types = Vec::new();
            let mut struct_generics = Vec::new();

            // Look up the Enum variants types
            let members_node = find_children(db, &node, SyntaxKind::MemberList).unwrap();
            for node in members_node.descendants(db) {
                if node.kind(db) == SyntaxKind::TypeClause {
                    struct_members_types.push(node);
                }
            }
            // Look up the Enum generics
            if let Some(child) = find_children(db, &node, SyntaxKind::WrappedGenericParamList) {
                for node in child.descendants(db) {
                    if node.kind(db) == SyntaxKind::GenericParamType {
                        let generic_type = node.get_text_without_trivia(db);
                        struct_generics.push(generic_type);
                    }
                }
            }
            cairo_enums.push(CairoEnum {
                name: enum_name,
                generics: struct_generics,
                variants_types: struct_members_types,
            });
        }
    }
    cairo_enums
}

const CORELIB_ENUMS: &str = "
enum bool {
    True: (),
    False: (),
}
enum U128sFromFelt252Result {
    Narrow: u128,
    Wide: (u128, u128),
}
enum never {}
enum FromNullableResult<T> {
    Null: (),
    NotNull: Box<T>,
}
enum Option<T> {
    Some: T,
    None: (),
}
enum PanicResult<T> {
    Ok: T,
    Err: (Panic, Array<felt252>),
}
enum Result<T, E> {
    Ok: T,
    Err: E,
}
enum IsZeroResult<T> {
    Zero: (),
    NonZero: NonZero<T>,
}
";

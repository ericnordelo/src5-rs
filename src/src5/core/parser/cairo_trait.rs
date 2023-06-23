// Module for handling Cairo traits
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;

use super::cairo_function::get_functions_from_trait_body;
use super::cairo_function::CairoNonGenericFunction;
use super::utils::find_children;

#[derive(Debug)]
pub struct CairoNonGenericTrait {
    pub name: String,
    pub functions: Vec<CairoNonGenericFunction>,
}

pub fn get_non_generic_traits(
    db: &RootDatabase,
    syntax_tree: &SyntaxNode,
) -> Vec<CairoNonGenericTrait> {
    let mut no_generic_traits = Vec::new();
    for node in syntax_tree.descendants(db) {
        if SyntaxKind::ItemTrait == node.kind(db) {
            // Check if has no generic types
            if let Some(_) =
                find_children(db, &node, SyntaxKind::OptionWrappedGenericParamListEmpty)
            {
                // Look up the Trait name
                let id_node = find_children(db, &node, SyntaxKind::TerminalIdentifier).unwrap();
                let trait_name = id_node.get_text_without_trivia(db);

                // Look up the Trait functions
                let trait_body = find_children(db, &node, SyntaxKind::TraitBody).unwrap();
                let functions = get_functions_from_trait_body(db, &trait_body);

                no_generic_traits.push(CairoNonGenericTrait {
                    name: trait_name,
                    functions,
                });
            }
        }
    }
    no_generic_traits
}

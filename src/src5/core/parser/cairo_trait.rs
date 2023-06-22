use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;

use super::utils::find_children;

#[derive(Debug)]
pub struct CairoNonGenericTrait {
    pub name: String,
    pub functions: Vec<CairoNonGenericFunction>,
}

#[derive(Debug)]
pub struct CairoNonGenericFunction {
    pub name: String,
    pub inputs_types: Vec<String>,
    pub return_type: String,
}

pub fn get_non_generic_traits(db: &RootDatabase, syntax_tree: &SyntaxNode) -> Vec<CairoNonGenericTrait> {
    let mut no_generic_traits = Vec::new();
    for node in syntax_tree.descendants(db) {
        if SyntaxKind::ItemTrait == node.kind(db) {
            // Check if has no generic types
            if let Some(_) = find_children(db, &node, SyntaxKind::OptionWrappedGenericParamListEmpty) {
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

fn get_functions_from_trait_body(db: &RootDatabase, trait_body: &SyntaxNode) -> Vec<CairoNonGenericFunction> {
    let mut functions = Vec::new();

    let trait_items = find_children(db, &trait_body, SyntaxKind::TraitItemList).unwrap();
    for node in trait_items.children(db) {
        if node.kind(db) == SyntaxKind::TraitItemFunction {
            // Look up the Function name
            let declaration_node = find_children(db, &node, SyntaxKind::FunctionDeclaration).unwrap();
            let id_node = find_children(db, &declaration_node, SyntaxKind::TerminalIdentifier).unwrap();
            let function_name = id_node.get_text_without_trivia(db);

            // Look up the Function inputs types
            let signature_node = find_children(db, &declaration_node, SyntaxKind::FunctionSignature).unwrap();
            let function_inputs = find_children(db, &signature_node, SyntaxKind::ParamList).unwrap();
            let mut inputs_types = Vec::new();
            for node in function_inputs.descendants(db) {
                if node.kind(db) == SyntaxKind::TypeClause {
                    let expr_path_node = find_children(db, &node, SyntaxKind::ExprPath).unwrap();
                    let input_type = expr_path_node.get_text_without_trivia(db);
                    inputs_types.push(input_type);
                }
            }

            // Look up the Function return type
            let function_return = find_children(db, &signature_node, SyntaxKind::ReturnTypeClause).unwrap();
            let expr_path_node = find_children(db, &function_return, SyntaxKind::ExprPath).unwrap();
            let return_type = expr_path_node.get_text_without_trivia(db);

            functions.push(CairoNonGenericFunction {
                name: function_name,
                inputs_types,
                return_type,
            });
        }
    }
    functions
}

// pub fn concrete_type(db: &RootDatabase, type_clause: &SyntaxNode) -> String {
//     match type_clause.kind(db) {
//         SyntaxKind::TypeClause | SyntaxKind::ReturnTypeClause => {
//             let expr_path = find_children(db, &type_clause, SyntaxKind::ExprPath).unwrap();
//             if let Some(path_segment_simple) = find_children(db, &expr_path, SyntaxKind::PathSegmentSimple) {
//             };
//         }
//         _ => {
//             panic!("Unexpected type clause kind: {:?}", type_clause.kind(db))
//         }
//     };

//     "".into()
// }

// fn resolve_type(db: &RootDatabase, expr_path: &SyntaxNode) -> String {
//     if let Some(path_segment_simple) = find_children(db, &expr_path, SyntaxKind::PathSegmentSimple) {
//         let type_name = find_children(db, &path_segment_simple, SyntaxKind::TerminalIdentifier).unwrap().get_text(db);

//         // Base type doesn't need to be resolved more
//         if type_name.is_base_type() {
//             return type_name;
//         }
//     };

//     match type_clause.kind(db) {
//         SyntaxKind::TypeClause | SyntaxKind::ReturnTypeClause => {
//             let expr_path = find_children(db, &type_clause, SyntaxKind::ExprPath).unwrap();
//             if let Some(path_segment_simple) = find_children(db, &expr_path, SyntaxKind::PathSegmentSimple) {
//                 abstract_type = path_segment.text(db).to_string();
//             };
//         }
//         _ => {
//             panic!("Unexpected type clause kind: {:?}", type_clause.kind(db))
//         }
//     }
// }

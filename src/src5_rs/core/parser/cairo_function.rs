use anyhow::{Ok, Result};
// Module for handling Cairo functions
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;

use super::cairo_enum::CairoEnum;
use super::cairo_struct::CairoStruct;
use super::utils::find_children;
use crate::core::src5_type::SRC5Typed;

#[derive(Debug)]
pub struct CairoNonGenericFunction {
    pub name: String,
    pub inputs_types: Vec<SyntaxNode>,
    pub return_type: Option<SyntaxNode>,
}

impl CairoNonGenericFunction {
    pub fn new(
        name: String,
        inputs_types: Vec<SyntaxNode>,
        return_type: Option<SyntaxNode>,
    ) -> CairoNonGenericFunction {
        CairoNonGenericFunction {
            name,
            inputs_types,
            return_type,
        }
    }

    /// Get the Extended Function Selector signature
    pub fn get_efs_signature(
        &self,
        db: &RootDatabase,
        cairo_structs: &[CairoStruct],
        cairo_enums: &[CairoEnum],
    ) -> Result<String> {
        let mut efs_signature = format!("{}(", self.name);
        // Resolve each member type
        for input in self.inputs_types.iter() {
            efs_signature.push_str(&input.get_src5_type(db, cairo_structs, cairo_enums)?);
            efs_signature.push(',');
        }
        if efs_signature.ends_with(',') {
            efs_signature.pop(); // Remove last comma
        }
        efs_signature.push(')');

        // Resolve return type
        if let Some(return_type) = &self.return_type {
            efs_signature.push_str("->");
            efs_signature.push_str(&return_type.get_src5_type(db, cairo_structs, cairo_enums)?);
        }
        Ok(efs_signature)
    }
}

pub fn get_functions_from_trait_body(
    db: &RootDatabase,
    trait_body: &SyntaxNode,
) -> Vec<CairoNonGenericFunction> {
    let mut functions = Vec::new();

    let trait_items = find_children(db, trait_body, SyntaxKind::TraitItemList).unwrap();
    for node in trait_items.children(db) {
        if node.kind(db) == SyntaxKind::TraitItemFunction {
            // Look up the Function name
            let declaration_node =
                find_children(db, &node, SyntaxKind::FunctionDeclaration).unwrap();
            let id_node =
                find_children(db, &declaration_node, SyntaxKind::TerminalIdentifier).unwrap();
            let function_name = id_node.get_text_without_trivia(db);

            // Look up the Function inputs types
            let signature_node =
                find_children(db, &declaration_node, SyntaxKind::FunctionSignature).unwrap();
            let function_inputs =
                find_children(db, &signature_node, SyntaxKind::ParamList).unwrap();
            let mut inputs_types = Vec::new();
            for node in function_inputs.descendants(db) {
                if node.kind(db) == SyntaxKind::TypeClause {
                    inputs_types.push(node);
                }
            }

            // Look up the Function return type
            let return_type = find_children(db, &signature_node, SyntaxKind::ReturnTypeClause);

            functions.push(CairoNonGenericFunction {
                name: function_name,
                inputs_types,
                return_type,
            });
        }
    }
    functions
}

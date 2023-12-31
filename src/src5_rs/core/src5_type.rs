// Module for computing SRC5 compliant types
use anyhow::{bail, Ok, Result};
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_syntax::node::{kind::SyntaxKind, SyntaxNode};
use std::collections::HashMap;

use crate::parser::{
    cairo_base_type::get_cairo_base_type_from_name, cairo_enum::CairoEnum,
    cairo_struct::CairoStruct, utils::find_children,
};

pub trait SRC5Typed {
    fn get_src5_type(
        &self,
        db: &RootDatabase,
        cairo_structs: &[CairoStruct],
        cairo_enums: &[CairoEnum],
    ) -> Result<String>;
}

impl SRC5Typed for SyntaxNode {
    fn get_src5_type(
        &self,
        db: &RootDatabase,
        cairo_structs: &[CairoStruct],
        cairo_enums: &[CairoEnum],
    ) -> Result<String> {
        let replacements = &mut HashMap::new();
        match self.kind(db) {
            SyntaxKind::TypeClause | SyntaxKind::ReturnTypeClause => {
                get_src5_type_from_type_clause_nodes_kind(
                    db,
                    self,
                    cairo_structs,
                    cairo_enums,
                    replacements,
                )
            }
            _ => {
                bail!("Node is not a Type Clause");
            }
        }
    }
}

/// Get the SRC5 type from a TypeClause, GenericArgExpr, or ReturnTypeClause AST nodes
fn get_src5_type_from_type_clause_nodes_kind(
    db: &RootDatabase,
    node: &SyntaxNode,
    cairo_structs: &[CairoStruct],
    cairo_enums: &[CairoEnum],
    replacements: &mut HashMap<String, String>,
) -> Result<String> {
    let mut src5_type = String::new();

    // Handle Unary expressions
    if let Some(unary_node) = find_children(db, node, SyntaxKind::ExprUnary) {
        src5_type.push_str(&get_src5_type_from_expr_unary_node(
            db,
            &unary_node,
            cairo_structs,
            cairo_enums,
            replacements,
        )?);
        Ok(src5_type)
    }
    // Handle Tuple expressions
    else if let Some(expr_tuple) = find_children(db, node, SyntaxKind::ExprTuple) {
        src5_type.push_str(&get_src5_type_from_expr_tuple_node(
            db,
            &expr_tuple,
            cairo_structs,
            cairo_enums,
            replacements,
        )?);
        Ok(src5_type)
    }
    // Handle Path expressions
    else if let Some(expr_path) = find_children(db, node, SyntaxKind::ExprPath) {
        src5_type.push_str(&get_src5_type_from_expr_path_node(
            db,
            &expr_path,
            cairo_structs,
            cairo_enums,
            replacements,
        )?);
        Ok(src5_type)
    } else {
        bail!("Unexpected Type node kind: {:?}", node.kind(db));
    }
}

/// Get the SRC5 type from an ExprPath AST node.
/// TODO: Handle complex paths (ex: `starknet::ContractAddress`)
fn get_src5_type_from_expr_path_node(
    db: &RootDatabase,
    node: &SyntaxNode,
    structs: &[CairoStruct],
    enums: &[CairoEnum],
    replacements: &mut HashMap<String, String>,
) -> Result<String> {
    let mut src5_type = String::new();

    // Handle no generics type
    if let Some(path_segment_simple) = find_children(db, node, SyntaxKind::PathSegmentSimple) {
        let name = path_segment_simple.get_text_without_trivia(db);

        // Handle replacements
        if let Some(replacement) = get_replacement_from_name(&name, replacements) {
            src5_type.push_str(&replacement);
        }
        // Handle base types
        else if let Some(base_type) = get_cairo_base_type_from_name(&name) {
            src5_type.push_str(&base_type.name);
        }
        // Handle struct types
        else if let Some(struct_type) = get_cairo_struct_from_name(&name, structs) {
            src5_type.push('(');
            // Resolve each member type
            for ty in struct_type.members_types.iter() {
                let src5_type_for_ty = get_src5_type_from_type_clause_nodes_kind(
                    db,
                    ty,
                    structs,
                    enums,
                    replacements,
                )?;
                src5_type.push_str(&src5_type_for_ty);
                src5_type.push(',');
            }
            if src5_type.ends_with(',') {
                src5_type.pop(); // Remove last comma
            }
            src5_type.push(')');
        }
        // Handle enum types
        else if let Some(enum_type) = get_cairo_enum_from_name(&name, enums) {
            src5_type.push_str("E(");
            // Resolve each member type
            for ty in enum_type.variants_types.iter() {
                let src5_type_for_ty = get_src5_type_from_type_clause_nodes_kind(
                    db,
                    ty,
                    structs,
                    enums,
                    replacements,
                )?;
                src5_type.push_str(&src5_type_for_ty);
                src5_type.push(',');
            }
            if src5_type.ends_with(',') {
                src5_type.pop(); // Remove last comma
            }
            src5_type.push(')');
        } else {
            bail!("Unexpected Cairo type: {}", name);
        }
    }
    // Handle type with generics
    else if let Some(path_segment_generics) =
        find_children(db, node, SyntaxKind::PathSegmentWithGenericArgs)
    {
        let id_node =
            find_children(db, &path_segment_generics, SyntaxKind::TerminalIdentifier).unwrap();
        let name = id_node.get_text_without_trivia(db);

        // Handle base types
        if let Some(base_type) = get_cairo_base_type_from_name(&name) {
            src5_type.push_str(&base_type.name);

            src5_type.push('<');
            // Resolve each generic type
            let generic_args_node =
                find_children(db, &path_segment_generics, SyntaxKind::GenericArgs).unwrap();
            let generic_args_list =
                find_children(db, &generic_args_node, SyntaxKind::GenericArgList).unwrap();
            for node in generic_args_list.children(db) {
                if node.kind(db) == SyntaxKind::GenericArgExpr {
                    let src5_type_for_generic_arg = get_src5_type_from_type_clause_nodes_kind(
                        db,
                        &node,
                        structs,
                        enums,
                        replacements,
                    )?;
                    src5_type.push_str(&src5_type_for_generic_arg);
                    src5_type.push(',');
                }
            }
            if src5_type.ends_with(',') {
                src5_type.pop(); // Remove last comma
            }
            src5_type.push('>');
        }
        // Handle struct types
        else if let Some(struct_type) = get_cairo_struct_from_name(&name, structs) {
            src5_type.push('(');
            // Resolve each generic type first
            let generic_args_node =
                find_children(db, &path_segment_generics, SyntaxKind::GenericArgs).unwrap();
            let generic_args_list =
                find_children(db, &generic_args_node, SyntaxKind::GenericArgList).unwrap();
            let new_replacements = &mut HashMap::new();
            let mut generic_index = 0;
            for node in generic_args_list.children(db) {
                if node.kind(db) == SyntaxKind::GenericArgExpr {
                    let src5_type_for_generic_arg = get_src5_type_from_type_clause_nodes_kind(
                        db,
                        &node,
                        structs,
                        enums,
                        replacements,
                    )?;
                    new_replacements.insert(
                        struct_type.generics[generic_index].clone(),
                        src5_type_for_generic_arg,
                    );
                    generic_index += 1;
                }
            }
            // Resolve each member type with replacements
            for ty in struct_type.members_types.iter() {
                let src5_type_for_ty = get_src5_type_from_type_clause_nodes_kind(
                    db,
                    ty,
                    structs,
                    enums,
                    new_replacements,
                )?;
                src5_type.push_str(&src5_type_for_ty);
                src5_type.push(',');
            }
            if src5_type.ends_with(',') {
                src5_type.pop(); // Remove last comma
            }
            src5_type.push(')');
        }
        // Handle enum types
        else if let Some(enum_type) = get_cairo_enum_from_name(&name, enums) {
            src5_type.push_str("E(");
            // Resolve each generic type first
            let generic_args_node =
                find_children(db, &path_segment_generics, SyntaxKind::GenericArgs).unwrap();
            let generic_args_list =
                find_children(db, &generic_args_node, SyntaxKind::GenericArgList).unwrap();
            let new_replacements = &mut HashMap::new();
            let mut generic_index = 0;
            for node in generic_args_list.children(db) {
                if node.kind(db) == SyntaxKind::GenericArgExpr {
                    let src5_type_for_generic_arg = get_src5_type_from_type_clause_nodes_kind(
                        db,
                        &node,
                        structs,
                        enums,
                        replacements,
                    )?;
                    new_replacements.insert(
                        enum_type.generics[generic_index].clone(),
                        src5_type_for_generic_arg,
                    );
                    generic_index += 1;
                }
            }
            // Resolve each variant type with replacements
            for ty in enum_type.variants_types.iter() {
                let src5_type_for_ty = get_src5_type_from_type_clause_nodes_kind(
                    db,
                    ty,
                    structs,
                    enums,
                    new_replacements,
                )?;
                src5_type.push_str(&src5_type_for_ty);
                src5_type.push(',');
            }
            if src5_type.ends_with(',') {
                src5_type.pop(); // Remove last comma
            }
            src5_type.push(')');
        } else {
            bail!("Unexpected Cairo type: {}", name);
        }
    } else {
        bail!("Unexpected node kind");
    }
    Ok(src5_type)
}

fn get_src5_type_from_expr_tuple_node(
    db: &RootDatabase,
    tuple_node: &SyntaxNode,
    cairo_structs: &[CairoStruct],
    cairo_enums: &[CairoEnum],
    replacements: &mut HashMap<String, String>,
) -> Result<String> {
    let mut src5_type = String::new();
    let expr_list = find_children(db, tuple_node, SyntaxKind::ExprList).unwrap();
    src5_type.push('(');
    for node in expr_list.children(db) {
        match node.kind(db) {
            SyntaxKind::ExprPath => {
                let src5_type_for_ty = get_src5_type_from_expr_path_node(
                    db,
                    &node,
                    cairo_structs,
                    cairo_enums,
                    replacements,
                )?;
                src5_type.push_str(&src5_type_for_ty);
                src5_type.push(',');
            }
            SyntaxKind::ExprTuple => {
                let src5_type_for_ty = get_src5_type_from_expr_tuple_node(
                    db,
                    &node,
                    cairo_structs,
                    cairo_enums,
                    replacements,
                )?;
                src5_type.push_str(&src5_type_for_ty);
                src5_type.push(',');
            }
            SyntaxKind::ExprUnary => {
                let src5_type_for_ty = get_src5_type_from_expr_unary_node(
                    db,
                    &node,
                    cairo_structs,
                    cairo_enums,
                    replacements,
                )?;
                src5_type.push_str(&src5_type_for_ty);
                src5_type.push(',');
            }
            _ => {}
        }
    }
    if src5_type.ends_with(',') {
        src5_type.pop(); // Remove last comma
    }
    src5_type.push(')');
    Ok(src5_type)
}

fn get_src5_type_from_expr_unary_node(
    db: &RootDatabase,
    unary_node: &SyntaxNode,
    cairo_structs: &[CairoStruct],
    cairo_enums: &[CairoEnum],
    replacements: &mut HashMap<String, String>,
) -> Result<String> {
    let mut src5_type = String::new();
    let leading_type_node = find_children(db, unary_node, SyntaxKind::TerminalAt).unwrap();
    src5_type.push_str(&leading_type_node.get_text_without_trivia(db));

    // Handle Path expressions
    if let Some(expr_path_node) = find_children(db, unary_node, SyntaxKind::ExprPath) {
        src5_type.push_str(&get_src5_type_from_expr_path_node(
            db,
            &expr_path_node,
            cairo_structs,
            cairo_enums,
            replacements,
        )?);
    }
    // Handle Tuple expressions
    else if let Some(expr_tuple) = find_children(db, unary_node, SyntaxKind::ExprTuple) {
        src5_type.push_str(&get_src5_type_from_expr_tuple_node(
            db,
            &expr_tuple,
            cairo_structs,
            cairo_enums,
            replacements,
        )?);
    }
    // Handle Unary expressions
    else if let Some(expr_unary) = find_children(db, unary_node, SyntaxKind::ExprUnary) {
        src5_type.push_str(&get_src5_type_from_expr_unary_node(
            db,
            &expr_unary,
            cairo_structs,
            cairo_enums,
            replacements,
        )?);
    } else {
        bail!("Unexpected Expr node kind: {:?}", unary_node.kind(db));
    }
    Ok(src5_type)
}

fn get_cairo_struct_from_name<'a>(
    name: &str,
    structs: &'a [CairoStruct],
) -> Option<&'a CairoStruct> {
    structs.iter().find(|&enum_type| enum_type.name == name)
}

fn get_cairo_enum_from_name<'a>(name: &str, enums: &'a [CairoEnum]) -> Option<&'a CairoEnum> {
    enums.iter().find(|&enum_type| enum_type.name == name)
}

fn get_replacement_from_name(name: &str, replacements: &HashMap<String, String>) -> Option<String> {
    if let Some(replacement) = replacements.get(name) {
        return Some(replacement.clone());
    }
    None
}

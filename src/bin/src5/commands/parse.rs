use anyhow::{Ok, Result};
use async_trait::async_trait;
use cairo_lang_syntax::node::kind::SyntaxKind;
use clap::Parser;
use src5::parser::cairo_struct::get_cairo_structs;
use src5::parser::ast::get_database_with_starknet_plugin;
// use src5::parser::cairo_trait::get_non_generic_traits;
use src5::parser::ast::get_syntax_tree;
use src5::parser::cairo_trait::get_non_generic_traits;

use super::CliCommand;

#[derive(Parser, Debug)]
pub struct Parse {
    #[clap(help = "File path to the Cairo source code")]
    pub cairo_path: String,
}

#[async_trait]
impl CliCommand for Parse {
    // Parse a file generating interface signatures for each trait
    async fn run(&self) -> Result<()> {
        println!("Parsing...");

        // Read Cairo file content
        let cairo_code = std::fs::read_to_string(&self.cairo_path)?;

        // Create a new database with the StarkNet plugin
        let db = get_database_with_starknet_plugin();

        // Get the syntax tree
        let tree = get_syntax_tree(&db, cairo_code);

        // for node in tree.descendants(&db) {
        //     // println!("{}", node.kind(&db));
        //     match node.kind(&db) {
        //         SyntaxKind::ReturnTypeClause => {
        //             println!();
        //             for node2 in node.children(&db) {
        //                 println!("{} - {}", node2.kind(&db), node2.get_text_without_trivia(&db));
        //             }
        //         }
        //         _ => {}
        //     }
        // }

        println!("{:?}", get_non_generic_traits(&db, &tree));
        Ok(())
    }
}

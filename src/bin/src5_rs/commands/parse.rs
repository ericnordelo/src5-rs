use anyhow::{Ok, Result};
use async_trait::async_trait;
use clap::Parser;
use num_bigint::BigUint;
use prettytable::{format, Table};
use src5_rs::parser::ast::get_database_with_starknet_plugin;
use src5_rs::parser::ast::get_syntax_tree;
use src5_rs::parser::cairo_enum::get_cairo_enums;
use src5_rs::parser::cairo_struct::get_cairo_structs;
use src5_rs::parser::cairo_trait::get_non_generic_traits;
use src5_rs::selector::get_selector_from_signature;

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
        // Read Cairo file content
        let cairo_code = std::fs::read_to_string(&self.cairo_path)?;
        // Create a new database with the StarkNet plugin
        let db = get_database_with_starknet_plugin();
        // Get the syntax tree
        let tree = get_syntax_tree(&db, cairo_code);

        let traits = get_non_generic_traits(&db, &tree);
        let cairo_structs = get_cairo_structs(&db, &tree);
        let cairo_enums = get_cairo_enums(&db, &tree);

        println!();
        let mut trait_table = Table::new();
        trait_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        trait_table
            .set_titles(row![bFg->"SRC5 Function Signature:", bFg->"Extended Function Selector:"]);

        for (i, cairo_trait) in traits.iter().enumerate() {
            if i > 0 {
                trait_table.add_empty_row();
            }
            let mut interface_id = BigUint::from(0u8);
            trait_table.add_row(row![bFg->cairo_trait.name]);
            for function in &cairo_trait.functions {
                let signature = function.get_efs_signature(&db, &cairo_structs, &cairo_enums);
                let selector = get_selector_from_signature(&signature);
                interface_id ^= selector.clone();
                trait_table.add_row(row![signature, format!("0x{:x}", selector)]);
            }
            trait_table.add_row(row![bFg->format!("Id: 0x{:x}", interface_id)]);
        }
        trait_table.printstd();

        Ok(())
    }
}

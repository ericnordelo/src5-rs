pub fn get_extended_function_selectors(
    db: &RootDatabase,
    trait_node: SyntaxNode,
) -> String {
  ""
}

pub fn get_extended_function_selector(
    db: &RootDatabase,
    function_node: SyntaxNode,
) -> String {
    // Find function name
    let function_name;
    for node in function_node.descendants(db) {
        if let SyntaxKind::TerminalIdentifier = node.kind(db) {
          function_name = node.text(db).to_string();
        }
    }
}
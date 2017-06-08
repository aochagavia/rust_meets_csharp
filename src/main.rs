mod ast;
mod node_map;
mod pretty;
mod programs;
mod visitor;

use node_map::NodeMap;

fn main() {
    let hw = programs::hello_world();
    println!("=== Hello world:");
    println!("{}", hw);
    let nm = NodeMap::build(&hw);
    println!("Nodemap: {:?}", nm);
    //let l = find_label_for_elem(&hw, "Console").expect("Label not found");
    //println!("Method list for Console: {:?}", get_method_list(&hw, &nm, l).unwrap());
}

fn find_label_for_elem(program: &ast::Program, name: &str) -> Option<ast::Label> {
    None
}

// Note: item_name must be unique
fn get_method_list<'a>(program: &'a ast::Program, nm: &NodeMap, label: ast::Label) -> Option<Vec<&'a str>> {
    // Find the element in the tree that has the given label...
    // We need a map from labels to node elements

    // Now what?
    Some(vec!["bananas"])
}

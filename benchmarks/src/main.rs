extern crate frontend;
extern crate time;
extern crate type_checker;

mod get_decl;
mod get_methods;
mod get_ty;

fn main() {
    // Get type
    let (program, expr) = get_ty::prepare();
    let on_demand_ns = get_ty::on_demand(&program, expr);
    let traditional_ns = get_ty::traditional(&program, expr.as_label());
    println!("Get type");
    println!("On demand (seconds): {}", on_demand_ns as f64 / 1000000000.0);
    println!("Traditional (seconds): {}", traditional_ns as f64 / 1000000000.0);
    println!();

    // Get decl
    let (program, decl, use_) = get_decl::prepare();
    let on_demand_ns = get_decl::on_demand(&program, decl, use_);
    let traditional_ns = get_decl::traditional(&program, decl, use_);
    println!("Get decl");
    println!("On demand (seconds): {}", on_demand_ns as f64 / 1000000000.0);
    println!("Traditional (seconds): {}", traditional_ns as f64 / 1000000000.0);
    println!();

    // Get methods
    let program = get_methods::prepare();
    let on_demand_ns = get_methods::on_demand(&program);
    let traditional_ns = get_methods::traditional(&program);
    println!("Get methods");
    println!("On demand (seconds): {}", on_demand_ns as f64 / 1000000000.0);
    println!("Traditional (seconds): {}", traditional_ns as f64 / 1000000000.0);
    println!();
}

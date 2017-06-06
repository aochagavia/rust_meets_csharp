mod ast;
mod pretty;
mod programs;

fn main() {
    println!("=== Hello world:");
    println!("{}", programs::hello_world());
    println!("=== Inheritance:");
    println!("{}", programs::inheritance());
}

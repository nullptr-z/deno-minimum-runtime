use deno_core::op;

#[op]
pub fn fetch() {
    println!("fetch");
}

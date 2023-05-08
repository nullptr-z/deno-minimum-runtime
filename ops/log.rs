use deno_core::op;

#[op]
pub fn log(args: &str) {
    println!("my log: {args}");
}

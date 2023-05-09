use deno_core::op;

#[op]
pub fn log(
    message: &str,
    arg1: &str,
    arg2: &str,
    arg3: &str,
    arg4: &str,
    // arg5: &str,
    // arg6: &str,
    // arg7: &str,
) {
    println!("-log-\n{message}\t{arg1}\t{arg2}\t{arg3}\t{arg4}");
}

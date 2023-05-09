use deno_core::{anyhow::Result, resolve_url_or_path};
use deno_minimum_runtime::create_runtime;

#[tokio::main]
async fn main() -> Result<()> {
    // let loader = MockLoader::new();
    let mut runtime = create_runtime()?;
    let js_file_path = &format!("{}/js/hello.js", env!("CARGO_MANIFEST_DIR"));
    // 传入 url 或者 path
    // 如果是 url 则使用 url；-- 内部使用 specifier_has_uri_scheme 验证
    // 如果不是则使用 path
    let js_file_url = resolve_url_or_path(&"", js_file_path.as_ref())?;
    let module_id = runtime.load_main_module(&js_file_url, None).await?;

    let mut received = runtime.mod_evaluate(module_id);
    loop {
        tokio::select! {
            resolved = &mut received=>{
               return resolved.expect("failed to evaluate module`无法评估模块");
            }
            _=runtime.run_event_loop(false)=>{
                // received.await.expect("failed to rvaluate module")?;
            }
        };
    }

    // Ok(())
}

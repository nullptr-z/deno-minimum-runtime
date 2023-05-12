use deno_core::{anyhow::Result, include_ascii_string, resolve_url_or_path, Extension};
use deno_minimum_runtime::{create_runtime, ops::fetch};

#[tokio::main]
async fn main() -> Result<()> {
    // let loader = MockLoader::new();
    let ext = Extension::builder("snapshot")
        .ops(vec![fetch::fetch::decl()])
        .build();
    let mut runtime = create_runtime(vec![ext])?;

    // #region 加载op
    // let exec_source = format!("globalThis.exec({request_value})").into();
    // runtime.execute_script_static(specifier, code);

    runtime
        .execute_script("log", include_ascii_string!("../snapshot/js/fetch.js"))
        .expect("Failed execute function the execute_script");
    // #endregion

    let js_file_path = &format!("{}/js/fetch.js", env!("CARGO_MANIFEST_DIR"));
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

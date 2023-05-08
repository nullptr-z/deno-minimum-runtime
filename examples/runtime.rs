use std::rc::Rc;

use deno_core::{
    anyhow::Result, resolve_url_or_path, Extension, FsModuleLoader, JsRuntime, RuntimeOptions,
};
use deno_minimum_runtime::ops::log::log;

#[tokio::main]
async fn main() -> Result<()> {
    // let loader = MockLoader::new();
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![Extension::builder("log").ops(vec![log::decl()]).build()],
        ..Default::default()
    };
    let mut runtime = JsRuntime::new(options);
    runtime.execute_script_static(
        "filename.js",
        r#"
        function log(args) {
            Deno.core.ops.log("log", args);
        }
    "#,
    )?;

    let js_file_path = &format!("{}/js/hello.js", env!("CARGO_MANIFEST_DIR"));
    // 传入 url 或者 path
    // 如果是 url 则使用 url；-- 内部使用 specifier_has_uri_scheme 验证
    // 如果不是则使用 path
    let js_file_url = resolve_url_or_path(&"", js_file_path.as_ref())?;
    let module_id = runtime.load_main_module(&js_file_url, None).await?;
    // .expect(&format!(
    //     "failed loaded load_main_module, module file: {js_file_url:}"
    // ));
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

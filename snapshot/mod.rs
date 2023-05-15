use deno_core::{Extension, JsRuntime, RuntimeOptions};
use std::fs::{self, DirEntry};
use std::path::Path;

use crate::ops::{fetch, log};

// const OPS_JS_FILE: &[&str] = &["js/**/*.js"];

pub fn create_snapshot() {
    let options = RuntimeOptions {
        will_snapshot: true,
        // module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![Extension::builder("snapshot")
            .ops(vec![log::log::decl(), fetch::op_fetch::decl()])
            .build()],
        ..Default::default()
    };
    let runtime = JsRuntime::new(options);

    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    // println!("【 path 】==> {:?}", path);
    // println!("【 path 】==> {:?}", path.join("/snapshot/js"));
    // println!("【 path 】==> {:?}", path.join("/snapshot/"));
    // 读取 js 文件目录
    let js_path = path.join("snapshot/js/");
    println!("【 js_path 】==> {:?}", js_path);
    let ops_file = read_directory(js_path.clone()).unwrap();
    println!("【 ops_file 】==> {:?}", ops_file);
    // 将每个 op 的 js 代码注入到 snapshot 中；提供一层 js 包装的 api
    // name的作用：在 js 中可以通过 Deno.core.ops()[name] 获取到对应的 op;
    // for opf in ops_file {
    //     let op_file = opf.path().to_str().unwrap().to_string();
    //     println!("【 op_file 】==> {:?}", op_file);
    //     let op_name = op_file
    //         .replace(js_path.to_str().unwrap(), "")
    //         .replace(".js", "");
    //     let op_name: Box<str> = op_name.into_boxed_str();
    //     let op_name: &'static str = Box::leak(op_name);
    //     println!("【 op name 】==> {:?}", op_name);

    //     let op_code = fs::read_to_string(&op_file).expect("failed to read file");
    //     let op_code: Box<str> = op_code.into_boxed_str();
    //     let op_code: &'static str = Box::leak(op_code);
    //     runtime.execute_script_static(op_name, op_code).unwrap();
    // }

    // runtime
    //     .execute_script("log", include_ascii_string!("js/log.js"))
    //     .expect("Failed execute function the execute_script");

    // runtime.execute_script_static("fe", r#"console.log(213)").unwrap();

    let snapshot = runtime.snapshot();
    // 将 snapshot 保存到文件中，写入二进制到文件中

    let snapshot_binary_path = path.join("snapshot.bin");
    println!("【 snapshot_binary_path 】==> {:?}", snapshot_binary_path);
    fs::write(snapshot_binary_path.clone(), snapshot)
        .expect(format!("failed to write snapshot the file: {snapshot_binary_path:?}").as_str());

    println!("snapshot created done, the file snapshot.bin");
}

fn read_directory(path: impl AsRef<Path>) -> Result<Vec<DirEntry>, std::io::Error> {
    let mut entries = Vec::new();
    let path = path.as_ref();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            entries.push(entry?);
        }
    }
    Ok(entries)
}

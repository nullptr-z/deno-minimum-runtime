pub mod ops;
pub mod snapshot;
pub mod utils;

use std::rc::Rc;

use deno_core::{anyhow::Result, Extension, FsModuleLoader, JsRuntime, RuntimeOptions};
use lazy_static::lazy_static;

lazy_static! {
    /// 代码加载时，加载快照数据
    static ref SNAPSHOT: &'static [u8] = {
        let data = include_bytes!("snapshot.bin");
        // let decompressed = decode_all(&data[..]).unwrap().into_boxed_slice();
        let a=data.to_vec().into_boxed_slice();
        Box::leak(a)
    };
}

pub fn create_runtime(ext: Vec<Extension>) -> Result<JsRuntime> {
    // let snapshot_bytes: &'static [u8] = include_bytes!("./snapshot.bin");
    // let a = &*SNAPSHOT;
    // let snapshot = Snapshot::Static(&*SNAPSHOT);

    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: ext,
        // startup_snapshot: Some(Snapshot::Static(&*SNAPSHOT)),
        ..Default::default()
    };

    let runtime = JsRuntime::new(options);

    Ok(runtime)
}

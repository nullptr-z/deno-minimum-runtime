pub mod ops;
pub mod snapshot;

use std::rc::Rc;

use deno_core::{anyhow::Result, FsModuleLoader, JsRuntime, RuntimeOptions, Snapshot};

pub fn create_runtime() -> Result<JsRuntime> {
    let snapshot_bytes: &'static [u8] = include_bytes!("snapshot/snapshot.bin");

    let snapshot = Snapshot::Static(snapshot_bytes);

    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        startup_snapshot: Some(snapshot),
        ..Default::default()
    };

    let runtime = JsRuntime::new(options);

    Ok(runtime)
}

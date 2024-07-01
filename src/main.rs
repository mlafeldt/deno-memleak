use std::path::Path;
use std::rc::Rc;

use deno_runtime::{
    deno_core::{error::AnyError, FsModuleLoader, ModuleSpecifier},
    deno_permissions::PermissionsContainer,
    worker::{MainWorker, WorkerOptions},
};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    sleep(Duration::from_secs(10)).await;
    loop {
        run_js().await?;
        sleep(Duration::from_secs(1)).await;
    }
}

async fn run_js() -> Result<(), AnyError> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("main.js");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();
    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        PermissionsContainer::allow_all(),
        WorkerOptions {
            module_loader: Rc::new(FsModuleLoader),
            ..Default::default()
        },
    );
    worker.execute_main_module(&main_module).await?;
    worker.run_event_loop(false).await?;
    Ok(())
}

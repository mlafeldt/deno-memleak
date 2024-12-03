use std::{path::Path, rc::Rc, sync::Arc};

use deno_runtime::{
    deno_core::{error::AnyError, FsModuleLoader, ModuleSpecifier},
    deno_fs::RealFs,
    deno_permissions::PermissionsContainer,
    permissions::RuntimePermissionDescriptorParser,
    worker::{MainWorker, WorkerOptions, WorkerServiceOptions},
};
use structopt::StructOpt;
use tokio::time::{sleep, Duration};

#[derive(StructOpt, Debug)]
#[structopt(name = "deno-memleak")]
struct Opt {
    #[structopt(
        short,
        long,
        default_value = "10",
        help = "Optional startup delay in seconds"
    )]
    delay: u64,

    #[structopt(
        short,
        long,
        default_value = "30",
        help = "Number of times to run the JS code"
    )]
    count: u64,
}

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let opt = Opt::from_args();

    sleep(Duration::from_secs(opt.delay)).await;

    for _ in 1..=opt.count {
        run_js().await?;
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

async fn run_js() -> Result<(), AnyError> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("main.js");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();
    let fs = Arc::new(RealFs);
    let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));
    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        WorkerServiceOptions {
            blob_store: Default::default(),
            broadcast_channel: Default::default(),
            feature_checker: Default::default(),
            fs,
            module_loader: Rc::new(FsModuleLoader),
            node_services: Default::default(),
            npm_process_state_provider: Default::default(),
            permissions: PermissionsContainer::allow_all(permission_desc_parser),
            root_cert_store_provider: Default::default(),
            fetch_dns_resolver: Default::default(),
            shared_array_buffer_store: Default::default(),
            compiled_wasm_module_store: Default::default(),
            v8_code_cache: Default::default(),
        },
        WorkerOptions::default(),
    );
    worker.execute_main_module(&main_module).await?;
    worker.run_event_loop(false).await?;
    Ok(())
}

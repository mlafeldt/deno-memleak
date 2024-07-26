use std::path::Path;

use deno_runtime::{
    deno_core::{error::AnyError, ModuleSpecifier},
    deno_permissions::{Permissions, PermissionsContainer},
    worker::{MainWorker, WorkerOptions},
};
use structopt::StructOpt;
use tokio::time::{sleep, Duration};

#[derive(StructOpt, Debug)]
#[structopt(name = "deno-memleak")]
struct Opt {
    #[structopt(
        short,
        long,
        default_value = "0",
        help = "Optional startup delay in seconds"
    )]
    delay: u64,

    #[structopt(
        short,
        long,
        default_value = "10",
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
    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        PermissionsContainer::new(Permissions::none_without_prompt()),
        WorkerOptions::default(),
    );
    worker.execute_main_module(&main_module).await?;
    worker.run_event_loop(false).await?;
    Ok(())
}

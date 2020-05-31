use deno_cli::global_state::GlobalState;
use deno_cli::worker::MainWorker;
use deno_core::ErrBox;
use deno_core::ModuleSpecifier;
fn create_main_worker(
    global_state: GlobalState,
    main_module: ModuleSpecifier,
) -> Result<MainWorker, ErrBox> {
    let state = State::new(global_state, None, main_module, false)?;

    let mut worker = MainWorker::new("main".to_string(), startup_data::deno_isolate_init(), state);

    {
        let (stdin, stdout, stderr) = get_stdio();
        let mut t = worker.resource_table.borrow_mut();
        t.add("stdin", Box::new(stdin));
        t.add("stdout", Box::new(stdout));
        t.add("stderr", Box::new(stderr));
    }

    worker.execute("bootstrap.mainRuntime()")?;
    Ok(worker)
}

fn main() {
    println!("bind deno");
}

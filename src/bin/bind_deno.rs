use deno_cli::flags;
use deno_cli::run_command;
use deno_cli::tokio_util;

fn main() {
    let args: Vec<String> = vec![
        "deno".to_string(),
        "run".to_string(),
        "./main.ts".to_string(),
    ];
    let flags = flags::flags_from_vec(args);
    let f = run_command(flags, "./main.ts".to_string());
    tokio_util::run_basic(f);
    println!("bind deno");
}

use std::env;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    wei_env::bin_init("wei-transmission");
    let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     help();
    //     std::process::exit(1);
    // }
    let command = &args[1];

    match command.as_str() {
        "install" => {
            println!("Installing...");
        },
        "uninstall" => {
            println!("Uninstalling...");
        },
        "check" => {

        },
        "api" => {
            // api().await?;
        },
        _ | "help" => {
            help();
            std::process::exit(1);
        }
    }

    Ok(())
}

fn help() {
    let args: Vec<String> = env::args().collect();
    println!("Usage:");
    println!("  {} install", args[0]);
    println!("  {} uninstall", args[0]);
    println!("  {} api <report_url_process> <url> <json>", args[0]);
}

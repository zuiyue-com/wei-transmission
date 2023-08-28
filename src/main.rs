use std::env;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    wei_env::bin_init("wei-transmission");
    let args: Vec<String> = env::args().collect();

    let url = &args[1];

    match command.as_str() {
        "url" => {
            
        },
        "check" => {

        },
        "api" => {
            // api().await?;
        },
        _  => {
            help();
            std::process::exit(1);
        }
    }

    Ok(())
}

fn help() {
    let args: Vec<String> = env::args().collect();
    println!("Usage:");
    println!("  {} <url>", args[0]);
}

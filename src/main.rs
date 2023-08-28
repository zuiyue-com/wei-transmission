use std::env;
use serde_json::Value;

use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    wei_env::bin_init("wei-transmission");
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    match command.as_str() {
        "add" => {
            let client = Client::new();

            let resp = client.post("http://localhost:9091/transmission/rpc")
                .send().await?;

            let session_id = &resp.headers()["X-Transmission-Session-Id"]; 

            println!("{:#?}", session_id);


            let resp = client.post("http://localhost:9091/transmission/rpc")
                .header("X-Transmission-Session-Id", session_id) 
                .json(&json!({
                    "method": "torrent-add",
                    "arguments": {
                        "paused": false, 
                        "filename": " magnet:?xt=urn:btih:759066CF9DBD7A473F2C5BB71BE9F77F05BF3191"
                    }
                }))
                .send().await?
                .text().await?;
                //.json::<serde_json::Value>().await?;
        
            println!("{:#?}", resp);

            let resp = client.post("http://localhost:9091/transmission/rpc")
            .header("X-Transmission-Session-Id", session_id) 
    .json(&json!({
        "method": "torrent-start",
        "arguments": {
            "ids": "2"
        }
    }))
    .send().await?
    .text().await?;

  println!("{:#?}", resp);
        },
        "uninstall" => {
            println!("Uninstalling...");
        },
        "check" => {

        },
        "api" => {
            // api().await?;
        },
        _ => {
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

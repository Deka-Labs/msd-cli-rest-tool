use std::time::Duration;

use clap::StructOpt;
use processors::ProcessorErrorStatus;
use reqwest::{blocking::Client, header};

extern crate serde;

mod cli;
mod processors;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn main() {
    let args = cli::MainCliArgs::parse();

    let mut client_builder = Client::builder();
    if let Some(k) = &args.api {
        let mut headers = header::HeaderMap::new();
        headers.insert("x-api-key", header::HeaderValue::from_str(k).unwrap());

        client_builder = client_builder.default_headers(headers)
    }

    let mut client = client_builder
        .user_agent(APP_USER_AGENT)
        .connect_timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to build client");

    let processors = processors::init_processors();

    for p in processors {
        let res = p.process_args(&args, &mut client);
        match res {
            Ok(_) => break,
            Err(ProcessorErrorStatus::NotMyCommand) => continue,
            Err(ProcessorErrorStatus::Error) => break,
        }
    }
}
